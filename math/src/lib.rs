//! This crate provides mathematics.
//!
//! # Examples
//!
//! ```
//! use math::square;
//! assert_eq!(square(0.9).unwrap(), 0.81);
//! ```

extern crate num;
use num::traits::Num;

// For integration-style tests the function must be public to be visible
// from outside this crate.

/// This function returns the square of numeric primitives.
//
/// # Examples
///
/// ```
/// use math::square;
/// assert_eq!(square(0.8).unwrap(), 0.81);
/// ```
pub fn square<T: Num + Copy>(n: T) -> Result<T, String> {
    Result::Ok(n * n)
}

#[allow(dead_code)]
fn main() {
    println!("{}", square(19).unwrap());
    println!("{}", square(1.9).unwrap());
}

// The 'cfg' attribute only compiles the test code if we're currently
// trying to run the tests. This ensures that our tests are entirely
// left out of a normal build.
#[cfg(test)]
mod tests {
    // The module allows us to group all tests and to define helper
    // functions, that don't become part of the rest of the crate.

    // We are in an inner module, so we need to bring our test function
    // into scope. This can be annoying for large modules, and so this
    // is a common use of globs.
    use super::*;
    // use super::square;

    #[test]
    fn unit_test_square_int() {
        assert!(square(9).unwrap() == 81);
    }

    #[test]
    fn unit_test_square_float() {
        assert_eq!(square(0.9).unwrap(), 0.81);
    }
}
