//! Version information for app.

/// The short version information for app.
///
/// - The latest version from Cargo.toml
/// - The short SHA of the latest commit.
///
/// # Example
///
/// ```text
/// 0.1.0 (defa64b2)
/// ```
pub(crate) const SHORT_VERSION: &str =
    concat!(env!("CARGO_PKG_VERSION"), " (", env!("VERGEN_GIT_SHA"), ")");

/// The long version information for app.
///
/// - The latest version from Cargo.toml
/// - The long SHA of the latest commit.
/// - The build date
/// - The build features
///
/// # Example:
///
/// ```text
/// Version: 0.1.0
/// Commit SHA: defa64b2
/// Build Timestamp: 2023-05-19T01:47:19.815651705Z
/// ```
pub(crate) const LONG_VERSION: &str = const_str::concat!(
    "Version: ",
    env!("CARGO_PKG_VERSION"),
    "\n",
    "Commit SHA: ",
    env!("VERGEN_GIT_SHA"),
    "\n",
    "Build Timestamp: ",
    env!("VERGEN_BUILD_TIMESTAMP"),
    "\n",
    "Build Features: ",
    env!("VERGEN_CARGO_FEATURES")
);

/// The default extradata used for payload building.
///
/// - The latest version from Cargo.toml
///
/// # Example
///
/// ```text
/// app/v{major}.{minor}.{patch}
/// ```
pub fn default_extradata() -> String {
    format!("app/v{}", env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_short_version() {
        println!("short version: {}", SHORT_VERSION.as_bytes().len());
        assert!(
            SHORT_VERSION.as_bytes().len() <= 32,
            "SHORT_VERSION must be less than 32 bytes: {SHORT_VERSION}"
        )
    }

    #[test]
    fn assert_long_version() {
        println!("long version: {}", LONG_VERSION);
        assert!(
            LONG_VERSION.as_bytes().len() >= 32,
            "LONG_VERSION must be greater than 32 bytes: {LONG_VERSION}"
        )
    }

    #[test]
    fn assert_extradata_less_32bytes() {
        let extradata = default_extradata();
        println!("extradata: {}", extradata.clone());
        assert!(
            extradata.as_bytes().len() <= 32,
            "extradata must be less than 32 bytes: {extradata}"
        )
    }
}
