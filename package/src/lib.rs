//! # Get Dir
//!
//! A utility to get directory.
//!
//! This utility searches for a target directory by checking for any directories or files that match the provided input.
//!
//! ## Usage
//!
//! Get directory by target with the following code:
//!
//! ```rust
//! use std::path::PathBuf;
//!
//! use get_dir::{
//!     GetDir,
//!     Target,
//!     DirTarget,
//! };
//!
//! let path: PathBuf = GetDir::new()
//!     .target(
//!         Target::Dir(DirTarget::new("src")),
//!     )
//!     .run()
//!     .unwrap();
//! ```
//!
//! Or get directory by target in reverse with the following code:
//!
//! ```rust
//! use std::path::PathBuf;
//!
//! use get_dir::{
//!     GetDir,
//!     Target,
//!     FileTarget,
//! };
//!
//! let path: PathBuf = GetDir::new()
//!     .target(
//!         Target::File(FileTarget::new("LICENSE")),
//!     )
//!     .run_reverse()
//!     .unwrap();
//! ```
//!     
//! Async version also available with `async_std`, `smol` and `tokio` features:
//!
//! ```ignore
//! use std::path::PathBuf;
//!
//! use get_dir::{
//!     GetDir,
//!     Target,
//!     DirTarget,
//!     // async_std,
//!     async_std::GetDirAsyncExt,
//!     // smol
//!     smol::GetDirAsyncExt,
//!     // tokio
//!     tokio::GetDirAsyncExt,
//! };
//!
//! # async fn example() {
//! let path: PathBuf = GetDir::new()
//!     .target(
//!         Target::Dir(DirTarget::new("src")),
//!     )
//!     .run_async()
//!     .await
//!     .unwrap();
//! # }
//! ```

pub(crate) mod structs;

pub(crate) mod util;

pub use crate::structs::target::dir::DirTarget;

pub use crate::structs::target::file::FileTarget;

pub use crate::structs::target::Target;

pub use crate::util::GetDir;

/// Run asynchronously with `async_std` feature.
///
/// To use it, add the following code to the `Cargo.toml` file:
///
/// ```toml
/// [dependencies]
/// get_dir = { version = "*", features = ["async_std"] }
/// ```
#[cfg(feature = "async_std")]
pub mod async_std {
    pub use crate::util::async_std::GetDirAsyncExt;
}

/// Run asynchronously with `smol` feature.
///
/// To use it, add the following code to the `Cargo.toml` file:
///
/// ```toml
/// [dependencies]
/// get_dir = { version = "*", features = ["smol"] }
/// ```
#[cfg(feature = "smol")]
pub mod smol {
    pub use crate::util::smol::GetDirAsyncExt;
}

/// Run asynchronously with `tokio` feature.
///
/// To use it, add the following code to the `Cargo.toml` file:
///
/// ```toml
/// [dependencies]
/// get_dir = { version = "*", features = ["tokio"] }
/// ```
#[cfg(feature = "tokio")]
pub mod tokio {
    pub use crate::util::tokio::GetDirAsyncExt;
}
