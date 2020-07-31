//! Git-related functions for retrieving various information about a repository.

use crate::{error::Error, types::SemverCompatibleGitTag};
use git2::{Error as GitError, Oid, Repository};
use semver::{SemVerError, Version};

/// Returns a list of commits that are in a given range.
///
/// The range should be of the form `<commit>..<commit>` where each `<commit>`
/// is in the form accepted by the `git rev-parse` command. The left-hand commit
/// will be the lower bound and the right-hand commit the upper bound (both
/// inclusive).
pub fn git_commits_in_range<'a>(
    repo: &'a Repository,
    range: &'a str,
) -> Result<Vec<Oid>, GitError> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_range(range)?;

    let mut oids = Vec::new();
    for commit in revwalk {
        oids.push(commit?);
    }

    Ok(oids)
}

/// Returns the latest semantic version compatible git tag inside a repository.
///
/// This method is a shorthand for
/// [latest_semver_compatible_git_tag_with_pattern](fn.
/// latest_semver_compatible_git_tag_with_pattern.html) that uses this pattern:
/// `v*`.
pub fn latest_semver_compatible_git_tag<'a>(
    repo: &'a Repository,
) -> Result<SemverCompatibleGitTag, Error<'a>> {
    latest_semver_compatible_git_tag_with_pattern(repo, Some("v*"))
}

/// Returns the latest semantic version compatible git tag inside a repository.
///
/// # Arguments
///
/// - `repo`: The git repository to search for matching tags.
/// - `pattern`: The pattern used to match against all found git tags.
///
/// # Returns
///
/// `Ok(SemverCompatibleGitTag)` if a tag could be found,
/// `Err(Error::NoSemverCompatibleTags)` if no tags could be found and
/// `Err(Error::GitError)` if any errors occurred while querying the repository.
fn latest_semver_compatible_git_tag_with_pattern<'a>(
    repo: &'a Repository,
    pattern: Option<&'a str>,
) -> Result<SemverCompatibleGitTag, Error<'a>> {
    let tag_names = repo.tag_names(pattern)?;
    let tag_to_semver_mapped: Option<(&str, Version)> = tag_names
        .iter()
        .flatten()
        .map(|v: &str| {
            let trimmed = v.trim_start_matches("v");
            (v, trimmed)
        })
        .map::<Result<(&str, Version), SemVerError>, _>(|(tag, semver)| {
            Ok((tag, Version::parse(semver)?))
        })
        .flatten()
        .max_by(|v1, v2| v1.1.cmp(&v2.1));

    match tag_to_semver_mapped {
        Some((tag, version)) => {
            // The tag has to be mapped to a commit hash first.
            let oid = repo.resolve_reference_from_short_name(tag)?;
            debug_assert!(oid.is_tag());
            let peeled_tag = oid.peel_to_commit()?;

            Ok(SemverCompatibleGitTag {
                oid: peeled_tag.id(),
                raw: tag.to_string(),
                version,
            })
        }
        None => Err(Error::NoSemverCompatibleTags),
    }
}
