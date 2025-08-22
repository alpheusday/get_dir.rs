use std::collections::VecDeque;

use async_std::{
    fs, io,
    path::{Path, PathBuf},
    stream::StreamExt as _,
};

use crate::{structs::target::Target, util::GetDir};

async fn is_target_exists(
    path: &Path,
    target: &Target,
) -> bool {
    match target {
        | Target::Dir(tg) => path.join(&tg.name).is_dir().await,
        | Target::File(tg) => path.join(&tg.name).is_file().await,
    }
}

async fn is_targets_exist(
    dir: &Path,
    targets: &[Target],
) -> bool {
    for target in targets {
        if is_target_exists(dir, target).await {
            return true;
        }
    }

    false
}

async fn get_dir(options: GetDir) -> io::Result<PathBuf> {
    let GetDir { dir, depth, targets } = options;

    let dir: PathBuf = dir.into();

    if depth == 0 {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }

    let mut queue: VecDeque<(PathBuf, usize)> = VecDeque::new();

    queue.push_back((dir, depth));

    while let Some((current_dir, remaining_depth)) = queue.pop_front() {
        if is_targets_exist(&current_dir, &targets).await {
            return Ok(current_dir);
        }

        if remaining_depth <= 1 {
            continue;
        }

        let mut entries: fs::ReadDir = match fs::read_dir(&current_dir).await {
            | Ok(e) => e,
            | Err(_) => continue,
        };

        while let Some(Ok(entry)) = entries.next().await {
            let path: PathBuf = entry.path();

            if path.is_dir().await {
                queue.push_back((path, remaining_depth - 1));
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

async fn get_dir_reverse(options: GetDir) -> io::Result<PathBuf> {
    let GetDir { dir, depth, targets } = options;

    let current: PathBuf = dir.into();

    for (i, ancestor) in current.ancestors().enumerate() {
        if i >= depth {
            break;
        }

        if is_targets_exist(ancestor, &targets).await {
            return Ok(ancestor.to_path_buf());
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

/// Trait for getting directory with async-std.
pub trait GetDirAsyncExt {
    /// Get directory asynchronously.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use async_std::path::PathBuf;
    ///
    /// use get_dir::{
    ///     GetDir,
    ///     async_std::GetDirAsyncExt,
    /// };
    ///
    /// # async fn example() {
    /// let path: PathBuf = GetDir::new()
    ///     .run_async()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    fn run_async(
        self
    ) -> impl std::future::Future<Output = io::Result<PathBuf>> + Send;

    /// Get directory in reverse asynchronously.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use async_std::path::PathBuf;
    ///
    /// use get_dir::{
    ///     GetDir,
    ///     async_std::GetDirAsyncExt,
    /// };
    ///
    /// # async fn example() {
    /// let path: PathBuf = GetDir::new()
    ///     .run_reverse_async()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    fn run_reverse_async(
        self
    ) -> impl std::future::Future<Output = io::Result<PathBuf>> + Send;
}

impl GetDirAsyncExt for GetDir {
    async fn run_async(self) -> io::Result<PathBuf> {
        get_dir(self).await
    }

    async fn run_reverse_async(self) -> io::Result<PathBuf> {
        get_dir_reverse(self).await
    }
}
