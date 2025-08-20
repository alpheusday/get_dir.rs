use async_std::{
    fs, io,
    path::{Path, PathBuf},
    stream::StreamExt as _,
};

use crate::{structs::target::Target, util::GetDir};

async fn is_target_exists<'a>(
    path: &'a Path,
    target: &'a Target,
) -> bool {
    match target {
        | Target::Dir(tg) => {
            let target_path: PathBuf = path.join(tg.name.clone());

            if target_path.is_dir().await {
                return true;
            }

            false
        },
        | Target::File(tg) => {
            let target_path: PathBuf = path.join(tg.name.clone());

            if target_path.is_file().await {
                return true;
            }

            false
        },
    }
}

async fn search_targets<'a>(
    dir: &'a PathBuf,
    targets: &'a Vec<Target>,
) -> Option<PathBuf> {
    for target in targets {
        if is_target_exists(dir, target).await {
            return Some(dir.to_owned());
        }
    }

    None
}

async fn get_dir<'a>(
    dir: &'a PathBuf,
    targets: &'a Vec<Target>,
) -> io::Result<PathBuf> {
    if let Some(found) = search_targets(dir, targets).await {
        return Ok(found);
    }

    let mut entries: fs::ReadDir = fs::read_dir(dir).await?;

    while let Some(entry) = entries.next().await {
        let current: PathBuf = entry?.path();

        if current.is_dir().await {
            if let Some(found) = search_targets(&current, targets).await {
                return Ok(found);
            }

            if let Ok(found) = Box::pin(get_dir(&current, targets)).await {
                return Ok(found);
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

async fn get_dir_reverse(
    dir: &PathBuf,
    targets: &Vec<Target>,
) -> io::Result<PathBuf> {
    let current: PathBuf = dir.into();

    for ancestor in current.ancestors() {
        for target in targets {
            if is_target_exists(ancestor, target).await {
                return Ok(ancestor.to_path_buf());
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

/// Trait for getting directory with async-std.
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

impl GetDirAsyncExt for GetDir {
    async fn run_async(&self) -> io::Result<PathBuf> {
        get_dir(&self.dir.clone().into(), &self.targets).await
    }

    async fn run_reverse_async(&self) -> io::Result<PathBuf> {
        get_dir_reverse(&self.dir.clone().into(), &self.targets).await
    }
}
