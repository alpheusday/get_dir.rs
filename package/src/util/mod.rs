#[cfg(feature = "async_std")]
pub mod async_std;

#[cfg(feature = "tokio")]
pub mod tokio;

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
        | Target::Dir(tg) => path.join(&tg.name).is_dir(),
        | Target::File(tg) => path.join(&tg.name).is_file(),
    }
}

pub(crate) fn search_targets(
    dir: &Path,
    targets: &[Target],
) -> bool {
    targets.iter().any(|t| is_target_exists(dir, t))
}

fn get_dir(options: GetDir) -> io::Result<PathBuf> {
    let GetDir { dir, depth, targets } = options;

    if depth == 0 {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }

    if search_targets(&dir, &targets) {
        return Ok(dir);
    }

    for entry in fs::read_dir(dir)? {
        let current: PathBuf = entry?.path();

        if !current.is_dir() {
            continue;
        }

        let opts: GetDir = GetDir::new()
            .dir(current)
            .depth(depth - 1)
            .targets(targets.clone());

        if let Ok(found) = get_dir(opts) {
            return Ok(found);
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

fn get_dir_reverse(options: GetDir) -> io::Result<PathBuf> {
    let GetDir { dir, depth, targets } = options;

    for (i, ancestor) in dir.ancestors().enumerate() {
        if i >= depth {
            break;
        }

        if search_targets(ancestor, &targets) {
            return Ok(ancestor.to_path_buf());
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

/// Utility to get directory.
#[derive(Debug, Clone)]
pub struct GetDir {
    /// The directory to run the process.
    ///
    /// By default, it runs in current directory.
    pub dir: PathBuf,
    /// The depth of the search.
    ///
    /// By default, it's [`usize::MAX`].
    pub depth: usize,
    /// The targets to search.
    ///
    /// By default, it's empty.
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
            depth: usize::MAX,
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

    /// Set the depth of the search.
    pub fn depth(
        mut self,
        depth: usize,
    ) -> Self {
        self.depth = depth;
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
    pub fn run(self) -> io::Result<PathBuf> {
        get_dir(self)
    }

    /// Get the first directory containing any of the specified targets in reverse.
    pub fn run_reverse(self) -> io::Result<PathBuf> {
        get_dir_reverse(self)
    }
}

impl Default for GetDir {
    fn default() -> Self {
        GetDir::new()
    }
}
