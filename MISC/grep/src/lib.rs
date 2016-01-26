// 'pub' to make func visible to integration tests in test/lib.rs, because
// these tests belong to another root module.

//! The `grep` crate provides functions to search strings in files.
//!
//! # Examples
//!
//! ```
//! assert_eq!(1, 1);
//! ```

/// This function search for a string in a file.
///
/// # Examples
///
/// ```
/// use grep::grep;
///
/// assert!(grep());
/// ```
pub fn grep() -> bool {
    return true;
}

// unit tests -- cargo test

// Attr 'cfg(test)' compiles the module only with 'cargo test'. The tests
// would be not part when releasing the module with 'cargo build'.
#[cfg(test)]
mod test {

    // import names from implicit root module to test module
    use super::*;

    // func to be called by 'cargo test'
    #[test]
    fn test_inner_scope() {
        assert!(grep());
    }

}
