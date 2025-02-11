#[cfg(feature = "async-std")]
pub mod async_std;

#[cfg(feature = "tokio")]
pub mod tokio;

use std::{
    env::current_dir,
    fs::{self, DirEntry, ReadDir},
    io,
    path::{Path, PathBuf},
};

/// Enum to determine whether the target is a directory or a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    /// The target is a directory.
    Dir,
    /// The target is a file.
    File,
}

/// Target struct for searching functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    pub name: String,
    pub r#type: TargetType,
}

fn target_exists(
    path: &Path,
    target: &Target,
) -> bool {
    match target.r#type {
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

/// Utility to get directory.
#[derive(Debug, Clone)]
pub struct GetDir {
    pub targets: Vec<Target>,
}

impl GetDir {
    /// Create a new GetDir instance.
    pub fn new() -> Self {
        GetDir { targets: Vec::new() }
    }

    /// Create a new GetDir instance from another GetDir instance.
    pub fn from(get_dir: GetDir) -> Self {
        get_dir
    }

    /// Add targets to the GetDir instance.
    pub fn targets(
        mut self,
        targets: Vec<Target>,
    ) -> Self {
        self.targets.extend(targets);
        self
    }

    /// Add a target to the GetDir instance.
    pub fn target(
        mut self,
        target: Target,
    ) -> Self {
        self.targets.push(target);
        self
    }

    /// Get the first directory containing any of the specified targets.
    pub fn get(&self) -> io::Result<PathBuf> {
        let current: PathBuf = current_dir()?;

        match search_dir(&current, &self.targets) {
            | Ok(Some(path)) => Ok(path),
            | _ => Err(io::Error::from(io::ErrorKind::NotFound)),
        }
    }

    /// Get the first directory containing any of the specified targets in reverse.
    pub fn get_reverse(&self) -> io::Result<PathBuf> {
        let current: PathBuf = current_dir()?;

        for ancestor in current.ancestors() {
            for target in &self.targets {
                let target_path: PathBuf = ancestor.join(&target.name);
                if target_exists(&target_path, target) {
                    return Ok(ancestor.to_path_buf());
                }
            }
        }

        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}

impl Default for GetDir {
    fn default() -> Self {
        GetDir::new()
    }
}

/// Get the project root directory by searching for
/// the `target` folder and the `Cargo.lock` file.
/// Use [`get_project_root`] to handle the error automatically.
pub fn get_project_root_directory() -> io::Result<PathBuf> {
    GetDir::new()
        .targets(vec![
            Target { name: "target".to_string(), r#type: TargetType::Dir },
            Target { name: "Cargo.lock".to_string(), r#type: TargetType::File },
        ])
        .get_reverse()
}

/// Get the project root directory by searching for
/// the `target` folder and the `Cargo.lock` file.
/// Use [`get_project_root_directory`] to handle the error manually.
pub fn get_project_root() -> PathBuf {
    get_project_root_directory().expect("Failed to get project root")
}
