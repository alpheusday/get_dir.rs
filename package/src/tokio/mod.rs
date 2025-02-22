use std::path::{Path, PathBuf};

use tokio::{
    fs::{self, ReadDir},
    io,
};

use crate::{GetDir, Target};

async fn target_exists<'a>(
    path: &'a Path,
    target: &'a Target<'a>,
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

async fn search_targets<'a>(
    dir: &'a PathBuf,
    targets: &'a Vec<Target<'a>>,
) -> Option<PathBuf> {
    for target in targets {
        if target_exists(dir, target).await {
            return Some(dir.to_owned());
        }
    }

    None
}

async fn search_dir<'a>(
    dir: &'a PathBuf,
    targets: &'a Vec<Target<'a>>,
) -> io::Result<PathBuf> {
    if let Some(found) = search_targets(dir, targets).await {
        return Ok(found);
    }

    let mut entries: ReadDir = fs::read_dir(dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let current: PathBuf = entry.path();

        if current.is_dir() {
            if let Some(found) = search_targets(&current, targets).await {
                return Ok(found);
            }

            if let Ok(found) = Box::pin(search_dir(&current, targets)).await {
                return Ok(found);
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

/// Trait for getting directory with tokio.
pub trait GetDirAsyncExt {
    /// Get directory asynchronously.
    fn run_async(
        &self
    ) -> impl std::future::Future<Output = io::Result<PathBuf>> + Send;

    /// Get directory in reverse asynchronously.
    fn run_reverse_async(
        &self
    ) -> impl std::future::Future<Output = io::Result<PathBuf>> + Send;
}

impl GetDirAsyncExt for GetDir<'_> {
    async fn run_async(&self) -> io::Result<PathBuf> {
        search_dir(&self.dir, &self.targets).await
    }

    async fn run_reverse_async(&self) -> io::Result<PathBuf> {
        for ancestor in self.dir.ancestors() {
            for target in &self.targets {
                if target_exists(ancestor, target).await {
                    return Ok(ancestor.to_path_buf());
                }
            }
        }

        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}
