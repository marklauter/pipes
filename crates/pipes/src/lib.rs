//! Watch and route.

/// The crate's version, from its manifest. A placeholder until the first
/// real code lands, so coverage has a line to measure.
#[must_use]
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_is_the_manifest_version() {
        assert_eq!(super::version(), "0.1.0");
    }
}
