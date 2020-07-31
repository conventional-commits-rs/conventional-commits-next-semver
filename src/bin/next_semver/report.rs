use conventional_commits_next_semver::types::TraversedCommit;

use semver::Version;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Report<'a> {
    pub commits: Vec<TraversedCommit<'a>>,
    pub from: &'a str,
    pub to: &'a str,
    pub current_version: Version,
    pub next_version: Version,
}
