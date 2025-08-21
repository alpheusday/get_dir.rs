use std::{collections::VecDeque, path::PathBuf};

use tokio::{fs, io};

use crate::util::{GetDir, is_targets_exist};

async fn get_dir(options: GetDir) -> io::Result<PathBuf> {
    let GetDir { dir, depth, targets } = options;

    if depth == 0 {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }

    let mut queue: VecDeque<(PathBuf, usize)> = VecDeque::new();

    queue.push_back((dir, depth));

    while let Some((current_dir, remaining_depth)) = queue.pop_front() {
        if is_targets_exist(&current_dir, &targets) {
            return Ok(current_dir);
        }

        if remaining_depth <= 1 {
            continue;
        }

        let mut entries: fs::ReadDir = match fs::read_dir(&current_dir).await {
            | Ok(e) => e,
            | Err(_) => continue,
        };

        while let Some(entry) = entries.next_entry().await? {
            let path: PathBuf = entry.path();

            if path.is_dir() {
                queue.push_back((path, remaining_depth - 1));
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

async fn get_dir_reverse(options: GetDir) -> io::Result<PathBuf> {
    let GetDir { dir, depth, targets } = options;

    for (i, ancestor) in dir.ancestors().enumerate() {
        if i >= depth {
            break;
        }

        if is_targets_exist(ancestor, &targets) {
            return Ok(ancestor.to_path_buf());
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

/// Trait for getting directory with tokio.
pub trait GetDirAsyncExt {
    /// Get directory asynchronously.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    ///
    /// use get_dir::{
    ///     GetDir,
    ///     tokio::GetDirAsyncExt,
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
    /// use std::path::PathBuf;
    ///
    /// use get_dir::{
    ///     GetDir,
    ///     tokio::GetDirAsyncExt,
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
