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
//! use get_dir::{
//!     GetDir,
//!     Target,
//!     DirTarget,
//! };
//!
//! GetDir::new()
//!     .targets(vec![
//!         Target::Dir(DirTarget {
//!             name: "src",  
//!         }),
//!     ])
//!     .run();
//! ```
//!
//! Or get directory by target in reverse with the following code:
//!
//! ```rust
//! use get_dir::{
//!     GetDir,
//!     Target,
//!     FileTarget,
//! };
//!
//! GetDir::new()
//!     .targets(vec![
//!         Target::File(FileTarget {
//!             name: "LICENSE",  
//!         }),
//!     ])
//!     .run_reverse();
//! ```
//!     
//! Async version also available with `async_std` and `tokio` features:
//!
//! ```rust
//! // This is a `async_std` example
//!
//! use get_dir::{
//!     GetDir,
//!     Target,
//!     FileTarget,
//!     async_std::GetDirAsyncExt,
//! };
//!
//! # async fn example() {
//! GetDir::new()
//!     .targets(vec![
//!         Target::File(FileTarget {
//!             name: "LICENSE",  
//!         }),
//!     ])
//!     .run_reverse_async()
//!     .await;
//! # }
//! ```
//!
//! ```rust
//! // This is a `tokio` example
//!
//! use get_dir::{
//!     GetDir,
//!     Target,
//!     FileTarget,
//!     tokio::GetDirAsyncExt,
//! };
//!
//! # async fn example() {
//! GetDir::new()
//!     .targets(vec![
//!         Target::File(FileTarget {
//!             name: "LICENSE",  
//!         }),
//!     ])
//!     .run_reverse_async()
//!     .await;
//! # }
//! ```

/// Run asynchronously with `async_std` feature.
///
/// To use it, add the following code to the `Cargo.toml` file:
///
/// ```toml
/// [dependencies]
/// get_dir = { version = "*", features = ["async_std"] }
/// ```
#[cfg(feature = "async_std")]
pub mod async_std;

/// Run asynchronously with `tokio` feature.
///
/// To use it, add the following code to the `Cargo.toml` file:
///
/// ```toml
/// [dependencies]
/// get_dir = { version = "*", features = ["tokio"] }
/// ```
#[cfg(feature = "tokio")]
pub mod tokio;

use std::{
    env::current_dir,
    fs, io,
    path::{Path, PathBuf},
};

/// Directory target struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirTarget<'a> {
    pub name: &'a str,
}

/// File target struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTarget<'a> {
    pub name: &'a str,
}

/// Enum to determine whether the target is a directory or a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target<'a> {
    /// The target is a directory.
    Dir(DirTarget<'a>),
    /// The target is a file.
    File(FileTarget<'a>),
}

fn target_exists(
    path: &Path,
    target: &Target,
) -> bool {
    match target {
        | Target::Dir(tg) => {
            let target_path: PathBuf = path.join(tg.name);

            if target_path.exists() && target_path.is_dir() {
                return true;
            }

            false
        },
        | Target::File(tg) => {
            let target_path: PathBuf = path.join(tg.name);

            if target_path.exists() && target_path.is_file() {
                return true;
            }

            false
        },
    }
}

fn search_targets(
    dir: &PathBuf,
    targets: &Vec<Target>,
) -> Option<PathBuf> {
    for target in targets {
        if target_exists(dir, target) {
            return Some(dir.to_owned());
        }
    }

    None
}

fn search_dir(
    dir: &PathBuf,
    targets: &Vec<Target>,
) -> io::Result<PathBuf> {
    if let Some(found) = search_targets(dir, targets) {
        return Ok(found);
    }

    for entry in fs::read_dir(dir)? {
        let current: PathBuf = entry?.path();

        if current.is_dir() {
            if let Some(found) = search_targets(&current, targets) {
                return Ok(found);
            }

            if let Ok(found) = search_dir(&current, targets) {
                return Ok(found);
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

/// Utility to get directory.
#[derive(Debug, Clone)]
pub struct GetDir<'a> {
    pub dir: PathBuf,
    pub targets: Vec<Target<'a>>,
}

impl<'a> GetDir<'a> {
    /// Create a new GetDir instance.
    pub fn new() -> Self {
        GetDir {
            dir: match current_dir() {
                | Ok(path) => path,
                | Err(_) => PathBuf::new(),
            },
            targets: Vec::new(),
        }
    }

    /// Create a new GetDir instance from another GetDir instance.
    pub fn from(get_dir: GetDir<'a>) -> Self {
        get_dir
    }

    /// Specific the directory to run the process.
    /// By default, it runs in current directory.
    pub fn directory<D: Into<PathBuf>>(
        mut self,
        dir: D,
    ) -> Self {
        self.dir = dir.into();
        self
    }

    /// Add targets to the GetDir instance.
    pub fn targets(
        mut self,
        targets: Vec<Target<'a>>,
    ) -> Self {
        self.targets.extend(targets);
        self
    }

    /// Add a target to the GetDir instance.
    pub fn target(
        mut self,
        target: Target<'a>,
    ) -> Self {
        self.targets.push(target);
        self
    }

    /// Get the first directory containing any of the specified targets.
    pub fn run(&self) -> io::Result<PathBuf> {
        search_dir(&self.dir, &self.targets)
    }

    /// Get the first directory containing any of the specified targets in reverse.
    pub fn run_reverse(&self) -> io::Result<PathBuf> {
        for ancestor in self.dir.ancestors() {
            for target in &self.targets {
                if target_exists(ancestor, target) {
                    return Ok(ancestor.to_path_buf());
                }
            }
        }

        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}

impl Default for GetDir<'_> {
    fn default() -> Self {
        GetDir::new()
    }
}
