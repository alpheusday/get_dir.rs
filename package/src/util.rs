use std::{
    env::current_dir,
    fs, io,
    path::{Path, PathBuf},
};

use crate::structs::target::Target;

fn is_target_exists(
    path: &Path,
    target: &Target,
) -> bool {
    match target {
        | Target::Dir(tg) => {
            let target_path: PathBuf = path.join(tg.name.clone());

            if target_path.is_dir() {
                return true;
            }

            false
        },
        | Target::File(tg) => {
            let target_path: PathBuf = path.join(tg.name.clone());

            if target_path.is_file() {
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
        if is_target_exists(dir, target) {
            return Some(dir.to_owned());
        }
    }

    None
}

fn get_dir(
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

            if let Ok(found) = get_dir(&current, targets) {
                return Ok(found);
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

fn get_dir_reverse(
    dir: &Path,
    targets: &Vec<Target>,
) -> io::Result<PathBuf> {
    for ancestor in dir.ancestors() {
        for target in targets {
            if is_target_exists(ancestor, target) {
                return Ok(ancestor.to_path_buf());
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

/// Utility to get directory.
#[derive(Debug, Clone)]
pub struct GetDir {
    pub dir: PathBuf,
    pub targets: Vec<Target>,
}

impl GetDir {
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

    /// Specific the directory to run the process.
    /// By default, it runs in current directory.
    pub fn directory<D: Into<PathBuf>>(
        mut self,
        dir: D,
    ) -> Self {
        self.dir = dir.into();
        self
    }

    /// Alias for [`GetDir::directory`] function.
    pub fn dir<D: Into<PathBuf>>(
        mut self,
        dir: D,
    ) -> Self {
        self.dir = dir.into();
        self
    }

    /// Add targets to the GetDir instance.
    pub fn targets<TS, T>(
        mut self,
        targets: TS,
    ) -> Self
    where
        TS: IntoIterator<Item = T>,
        T: Into<Target>,
    {
        self.targets.extend(targets.into_iter().map(|t| t.into()));
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
    pub fn run(&self) -> io::Result<PathBuf> {
        get_dir(&self.dir, &self.targets)
    }

    /// Get the first directory containing any of the specified targets in reverse.
    pub fn run_reverse(&self) -> io::Result<PathBuf> {
        get_dir_reverse(&self.dir, &self.targets)
    }
}

impl Default for GetDir {
    fn default() -> Self {
        GetDir::new()
    }
}
