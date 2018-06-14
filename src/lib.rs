
//! An implementation of [this RFC](https://github.com/rust-lang/rfcs/pull/2180).
//! 
//! Provides a single `trait` which allows the construction of an `Option` based on a bool value.
//! ```
//! extern crate imply_option;
//! 
//! use imply_option::*;
//! 
//! fn main() {
//!     let pass = true;
//! 
//!     assert_eq!(pass.then(1), Some(1));
//!     assert_eq!(pass.then_do(|| 1), Some(1));
//! 
//!     let fail = false;
//! 
//!     assert_eq!(fail.then(1), None);
//! }
//! ```

#![no_std]

/// Allows construction of an `Option<T>` based on a `bool`.
pub trait ImplyOption<T: Sized>: Sized + PartialEq<bool> {
    /// If `self == true` returns `Some(value)`.
    #[inline]
    fn then(self, value: T) -> Option<T> { self.then_do(move || value) }
    /// If `self == true` returns `Some(value)` where `value` is lazily constructed.
    fn then_do(self, value: impl FnOnce() -> T) -> Option<T> {
        if self == true { Some(value()) } else { None }
    }
}

impl<T: Sized + PartialEq<bool>, I: Sized> ImplyOption<I> for T {}
