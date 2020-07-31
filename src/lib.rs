use crate::utils::{increment_major, increment_minor, increment_patch};
use conventional_commits_parser::Commit;

use semver::Version;

pub mod error;
mod git;
pub mod types;
pub mod utils;

// Public API exports
pub use error::Error;
pub use git::{git_commits_in_range, latest_semver_compatible_git_tag};
pub use types::SemverCompatibleGitTag;

/// Returns the next semantic release version.
///
/// # Arguments
///
/// - `current_version`: The current semantic release version.
/// - `commits`: The list of conventional commits.
///
/// # Returns
///
/// `Ok(Version)` if the version needs to be bumped, `Err(Error::SameVersion)`
/// if the commits do not result in any version increment.`
pub fn next_version<'a>(
    current_version: Version,
    commits: &'a [&'a Commit<'a>],
) -> Result<Version, Error<'a>> {
    // Detect if any breaking changes happened.
    let is_breaking_change = commits.iter().any(|c| c.is_breaking_change);
    if is_breaking_change {
        return Ok(increment_major(current_version));
    }

    // Detect minor changes.
    let is_new_feature_available = commits.iter().any(|c| c.ty == "feat");
    if is_new_feature_available {
        return Ok(increment_minor(current_version));
    }

    // Detect patch changes.
    let is_new_fix_available = commits.iter().any(|c| c.ty == "fix");
    if is_new_fix_available {
        return Ok(increment_patch(current_version));
    }

    Err(Error::SameVersion(current_version))
}
