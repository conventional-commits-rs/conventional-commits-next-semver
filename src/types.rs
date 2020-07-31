//! Contains structs, enums and traits provided by this crate.

use conventional_commits_parser::Commit as ParsedCommit;
use git2::Oid;
use semver::Version;
#[cfg(feature = "serde")]
use serde::Serialize;

/// A git tag that is compatible with semantic release versions.
pub struct SemverCompatibleGitTag {
    /// The git commit the tag references.
    pub oid: Oid,
    /// The raw git tag name.
    pub raw: String,
    /// The semantic release version this matches to.
    pub version: Version,
}

impl SemverCompatibleGitTag {
    /// Creates a new instance with the given values.
    ///
    /// # Arguments
    ///
    /// - `oid`: The object id of the tag.
    /// - `raw`: The tag's annotation.
    /// - `version`: The semantic version of the tag.
    pub fn from(oid: Oid, raw: String, version: Version) -> Self {
        Self { oid, raw, version }
    }
}

/// A commit that has been traversed to determine the next semantic version.
///
/// Traversed commits can either adhere to the specification and be a
/// conventional commit, or are just normal commits.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum TraversedCommit<'a> {
    /// A conventional commit. A commit of this type has been successfully
    /// parsed by the underlying parser, [conventional-commits-parser](https://crates.io/crates/conventional-commits-parser).
    Conventional(ConventionalCommit<'a>),
    /// A normal commit. Its message can't be used to deduce the next semantic
    /// version of the crate.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_oid"))]
    Normal(Oid),
}

/// A conventional commit inside a repository.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ConventionalCommit<'a> {
    /// The object id of the commit.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_oid"))]
    pub oid: Oid,
    /// The parsed commit message.
    pub msg: ParsedCommit<'a>,
}

impl<'a> ConventionalCommit<'a> {
    /// Creates a conventional commit with the given values.
    ///
    /// # Arguments
    ///
    /// - `oid`: The object id of the commit.
    /// - `msg`: The parsed commit message.
    pub fn from(oid: Oid, msg: ParsedCommit<'a>) -> Self {
        Self { oid, msg }
    }
}

// Every Oid has a string representation, just use that one to serialize.
#[cfg(feature = "serde")]
fn serialize_oid<S>(oid: &Oid, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    ser.serialize_str(&oid.to_string())
}
