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
    let GetDir { dir, depth, targets, .. } = options;

    let dir: PathBuf = dir.into();

    if depth == 0 {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }

    if is_targets_exist(&dir, &targets).await {
        return Ok(dir);
    }

    let mut entries: fs::ReadDir = fs::read_dir(dir).await?;

    while let Some(entry) = entries.next().await {
        let current: PathBuf = entry?.path();

        if !current.is_dir().await {
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
    fn run_async(
        self
    ) -> impl std::future::Future<Output = io::Result<PathBuf>> + Send;

    /// Get directory in reverse asynchronously.
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
