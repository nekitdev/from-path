//! Loading values from paths.
//!
//! This crate provides the [`FromPath`] trait for types that can have values loaded from paths.
//!
//! There is also the [`from_path`] convenience function for when the types can be inferred.
//!
//! # Example
//!
//! Assuming there is some file named `hello-world` with the content `Hello, world!`,
//! we first implement the trait and then load the content from the file:
//!
//! ```rust
//! use std::{fs::read_to_string, io::Error, path::Path};
//!
//! use from_path::{FromPath, from_path};
//!
//! struct Content {
//!     pub string: String,
//! }
//!
//! impl Content {
//!     pub fn new(string: String) -> Self {
//!         Self { string }
//!     }
//! }
//!
//! impl FromPath for Content {
//!     type Error = Error;
//!
//!     fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
//!         read_to_string(path).map(Self::new)
//!     }
//! }
//!
//! let content: Content = from_path("hello-world").unwrap();
//!
//! assert_eq!(content.string.trim(), "Hello, world!");
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::path::Path;

/// Loading values from paths.
pub trait FromPath: Sized {
    /// The associated error type returned from [`from_path`] on failure.
    ///
    /// [`from_path`]: Self::from_path
    type Error;

    /// Loads the value of this type from the given path.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when loading fails.
    ///
    /// [`Error`]: Self::Error
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error>;
}

/// Loads the value of the given type from the given path.
///
/// # Errors
///
/// Returns [`Error`] when loading fails.
///
/// [`Error`]: FromPath::Error
pub fn from_path<F: FromPath, P: AsRef<Path>>(path: P) -> Result<F, F::Error> {
    F::from_path(path)
}
