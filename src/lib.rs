//! Loading values from paths.
//!
//! This crate provides the [`FromPath`] trait for types that can have values loaded from paths.
//!
//! There are also the [`load`] convenience function and the [`Load`] extension trait for
//! when the types can be inferred.
//!
//! # Example
//!
//! Firstly, we need to implement the [`FromPath`] trait, and then use it via [`load`] or [`Load`]:
//!
//! ```rust
//! use std::{fs::read_to_string, io::Error, path::Path};
//!
//! use from_path::{load, FromPath, Load};
//!
//! #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
//! pub struct Content {
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
//! let path = "hello-world";
//!
//! let content: Content = load(path).unwrap();
//!
//! assert_eq!(content.string.trim(), "Hello, world!");
//!
//! let extended: Content = path.load().unwrap();
//!
//! assert_eq!(content, extended);
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
pub fn load<F: FromPath, P: AsRef<Path>>(path: P) -> Result<F, F::Error> {
    F::from_path(path)
}

mod sealed {
    pub trait Sealed {}
}

/// Loading values from paths (sealed extension trait).
pub trait Load: sealed::Sealed {
    /// Loads the value of type [`F`] from the path.
    ///
    /// # Errors
    ///
    /// Returns [`F::Error`] when loading fails.
    ///
    /// [`F`]: FromPath
    /// [`F::Error`]: FromPath::Error
    fn load<F: FromPath>(&self) -> Result<F, F::Error>;
}

impl<P: AsRef<Path> + ?Sized> sealed::Sealed for P {}

impl<P: AsRef<Path> + ?Sized> Load for P {
    fn load<F: FromPath>(&self) -> Result<F, F::Error> {
        load(self)
    }
}
