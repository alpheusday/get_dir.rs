use std::path::PathBuf;

use tokio::{fs, io};

use crate::util::{GetDir, is_targets_exist};

async fn get_dir(options: GetDir) -> io::Result<PathBuf> {
    let GetDir { dir, depth, targets, .. } = options;

    if depth == 0 {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }

    if is_targets_exist(&dir, &targets) {
        return Ok(dir);
    }

    let mut entries: fs::ReadDir = fs::read_dir(dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let current: PathBuf = entry.path();

        if !current.is_dir() {
            continue;
        }

        let opts: GetDir = GetDir::new()
            .dir(current)
            .depth(depth - 1)
            .targets(targets.clone());

        if let Ok(found) = Box::pin(get_dir(opts)).await {
            return Ok(found);
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

async fn get_dir_reverse(options: GetDir) -> io::Result<PathBuf> {
    let GetDir { dir, depth, targets, .. } = options;

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
