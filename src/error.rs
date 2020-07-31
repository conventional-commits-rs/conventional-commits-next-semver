//! The crate's error type.

use git2::Error as GitError;
use semver::Version;
use thiserror::Error;

/// Library error types.
#[derive(Debug, Error)]
pub enum Error<'a> {
    /// An error occurred while querying the git repository.
    #[error("an error occurred while querying the git repository")]
    GitError(#[from] GitError),
    /// No semver compatible tags could be found inside the repository.
    #[error("no semver compatible tags where found. Make sure that tags are annotated as vX.X.X")]
    NoSemverCompatibleTags,
    /// No commits have been found that indicate a version bump.
    #[error("no new version to bump to")]
    SameVersion(Version),
    /// Other error types.
    #[error("an unknown error occurred")]
    Other(&'a str),
}
