//! # Get Dir
//!
//! An utility to get directory.
//!
//! This utility searches for a target directory by checking
//! for any directories or files that match the provided input.
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
//! get_dir_by_target(Target {
//!     name: "src".to_string(),
//!     ty: TargetType::Dir,
//! });
//! ```
//!
//! Or get directory by target in reverse with the following code:
//!
//! ```no_run
//! use get_dir::{
//!     Target,
//!     TargetType,
//!     get_dir_by_target_reverse,
//! };
//!
//! get_dir_by_target_reverse(Target {
//!     name: "LICENSE".to_string(),
//!     ty: TargetType::File,
//! });
//! ```

use std::{
    env::current_dir,
    fs::{self, DirEntry, ReadDir},
    io,
    path::{Path, PathBuf},
};

/// Enum to determine whether the target is a file or a directory.
#[derive(Clone)]
pub enum TargetType {
    File,
    Dir,
}

/// Target struct for searching functions.
#[derive(Clone)]
pub struct Target {
    /// The name of the target.
    pub name: String,
    /// The type of the target.
    pub ty: TargetType,
}

fn target_exists(
    path: &Path,
    target: &Target,
) -> bool {
    match target.ty {
        | TargetType::Dir => path.is_dir(),
        | TargetType::File => path.is_file(),
    }
}

fn search_targets(
    dir: &PathBuf,
    targets: &Vec<Target>,
) -> Option<PathBuf> {
    for target in targets {
        let target_path: PathBuf = dir.join(&target.name);
        if target_exists(&target_path, target) {
            return Some(dir.to_owned());
        }
    }

    None
}

fn search_dir(
    dir: &PathBuf,
    targets: &Vec<Target>,
) -> io::Result<Option<PathBuf>> {
    let entries: ReadDir = fs::read_dir(dir)?;

    if let Some(found) = search_targets(dir, targets) {
        return Ok(Some(found));
    }

    for entry in entries {
        let entry: DirEntry = entry?;
        let current: PathBuf = entry.path();

        if current.is_dir() {
            if let Some(found) = search_targets(&current, targets) {
                return Ok(Some(found));
            }

            if let Some(found) = search_dir(&current, targets)? {
                return Ok(Some(found));
            }
        }
    }

    Ok(None)
}

/// Search for the first directory containing any of the specified targets
/// from the current directory downwards.
pub fn get_dir_by_targets(targets: Vec<Target>) -> io::Result<PathBuf> {
    let current: PathBuf = current_dir()?;

    match search_dir(&current, &targets) {
        | Ok(Some(path)) => Ok(path),
        | _ => Err(io::Error::from(io::ErrorKind::NotFound)),
    }
}

/// Search for the first directory containing the specified target
/// from the current directory downwards.
pub fn get_dir_by_target(target: Target) -> io::Result<PathBuf> {
    get_dir_by_targets(vec![target])
}

/// Search for the first directory containing any of the specified targets
/// from the current directory upwards.
pub fn get_dir_by_targets_reverse(targets: Vec<Target>) -> io::Result<PathBuf> {
    let current: PathBuf = current_dir()?;

    for ancestor in current.ancestors() {
        for target in &targets {
            let target_path: PathBuf = ancestor.join(&target.name);
            if target_exists(&target_path, target) {
                return Ok(ancestor.to_path_buf());
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

/// Search for the first directory containing the specified target
/// from the current directory upwards.
pub fn get_dir_by_target_reverse(target: Target) -> io::Result<PathBuf> {
    get_dir_by_targets_reverse(vec![target])
}

/// Get the project root directory by searching for
/// the `target` folder and the `Cargo.lock` file.
/// Use [`get_project_root`] to handle the error automatically.
pub fn get_project_root_directory() -> io::Result<PathBuf> {
    get_dir_by_targets_reverse(vec![
        Target { name: "target".to_string(), ty: TargetType::Dir },
        Target { name: "Cargo.lock".to_string(), ty: TargetType::File },
    ])
}

/// Get the project root directory by searching for
/// the `target` folder and the `Cargo.lock` file.
/// Use [`get_project_root_directory`] to handle the error manually.
pub fn get_project_root() -> PathBuf {
    get_project_root_directory().expect("Failed to get project root")
}
