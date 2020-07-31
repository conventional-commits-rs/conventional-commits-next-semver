use semver::Version;

/// Increments the semver major version.
///
/// In the case of a major of `0`, this does increment the minor instead.
pub fn increment_major(current_version: Version) -> Version {
    let mut cloned_version = current_version.clone();
    match current_version.major {
        0 => cloned_version.increment_minor(),
        _ => cloned_version.increment_major(),
    }

    cloned_version
}

/// Increments the semver minor version.
///
/// In the case of a major of `0`, this does increment the patch instead.
pub fn increment_minor(current_version: Version) -> Version {
    let mut cloned_version = current_version.clone();
    match current_version.major {
        0 => cloned_version.increment_patch(),
        _ => cloned_version.increment_minor(),
    }

    cloned_version
}

/// Increments the semver patch version.
pub fn increment_patch(current_version: Version) -> Version {
    let mut cloned_version = current_version.clone();
    cloned_version.increment_patch();
    cloned_version
}
