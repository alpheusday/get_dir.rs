//! # Get Dir
//!
//! An utility to get directory.
//!
//! A Directory searching utility that will check whether
//! the target file or directory exists in the directory.
//! The search process will start from the current directory 
//! and go to the root. Therefore, targets in other subdirectories 
//! will not be found, but a better performance is expected.
//!
//! ## Usage
//!
//! Get directory by target with the following code:
//!
//! ```no_run
//! use get_dir::{
//!     Target,
//!     TargetType,
//!     get_dir_by_target,
//! };
//!
//! // Get the directory of the `LICENSE` file located in.
//! get_dir_by_target(Target {
//!     name: "LICENSE".to_string(),
//!     ty: TargetType::File,
//! });
//! ```

use std::{env::current_dir, io, path::PathBuf};

/// Enum to determine whether the target is a file or a directory.
pub enum TargetType {
    File,
    Dir,
}

/// Target struct for `get_dir_by_targets` function.
pub struct Target {
    pub name: String,
    pub ty: TargetType,
}

/// Get directory by searching whether the targets exists in the directory
/// from the current directory to the root.
pub fn get_dir_by_targets(targets: Vec<Target>) -> io::Result<PathBuf> {
    let current: PathBuf = current_dir()?;

    for ancestor in current.as_path().ancestors() {
        for target in &targets {
            let target_path: PathBuf = ancestor.join(target.name.as_str());

            let target_exists: bool = match target.ty {
                | TargetType::File => target_path.exists(),
                | TargetType::Dir => target_path.is_dir(),
            };

            if target_exists {
                return Ok(ancestor.to_path_buf());
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

/// Get directory by searching whether the target exists in the directory
/// from the current directory to the root.
pub fn get_dir_by_target(target: Target) -> io::Result<PathBuf> {
    get_dir_by_targets(vec![target])
}

/// Get project root directory by searching for
/// `Cargo.lock` and `target` folder.
pub fn get_project_root() -> PathBuf {
    get_dir_by_targets(vec![
        Target { name: "Cargo.lock".to_string(), ty: TargetType::File },
        Target { name: "target".to_string(), ty: TargetType::Dir },
    ])
    .expect("Failed to get project root")
}
