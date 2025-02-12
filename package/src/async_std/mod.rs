use std::env::current_dir;

use async_std::{
    fs::{self, DirEntry, ReadDir},
    io,
    path::{Path, PathBuf},
    stream::StreamExt as _,
};

use crate::{DirTarget, FileTarget, GetDir, Target};

async fn target_exists(
    path: &Path,
    target: &Target,
) -> bool {
    match target {
        | Target::Dir(tg) => {
            let target_path: PathBuf = path.join(&tg.name);

            if target_path.exists().await && target_path.is_dir().await {
                return true;
            }

            false
        },
        | Target::File(tg) => {
            let target_path: PathBuf = path.join(&tg.name);

            if target_path.exists().await && target_path.is_file().await {
                return true;
            }

            false
        },
    }
}

async fn search_targets(
    dir: &PathBuf,
    targets: &Vec<Target>,
) -> Option<PathBuf> {
    for target in targets {
        if target_exists(dir, target).await {
            return Some(dir.to_owned());
        }
    }

    None
}

async fn search_dir(
    dir: &PathBuf,
    targets: &Vec<Target>,
) -> io::Result<Option<PathBuf>> {
    let mut entries: ReadDir = fs::read_dir(dir).await?;

    if let Some(found) = search_targets(dir, targets).await {
        return Ok(Some(found));
    }

    while let Some(entry) = entries.next().await {
        let entry: DirEntry = entry?;
        let current: PathBuf = entry.path();

        if current.is_dir().await {
            if let Some(found) = search_targets(&current, targets).await {
                return Ok(Some(found));
            }

            if let Ok(Some(found)) =
                Box::pin(search_dir(&current, targets)).await
            {
                return Ok(Some(found));
            }
        }
    }

    Ok(None)
}

/// Trait for getting directory with async-std.
pub trait GetDirAsyncExt {
    /// Get directory asynchronously.
    fn get_async(
        &self
    ) -> impl std::future::Future<Output = io::Result<PathBuf>> + Send;

    /// Get directory in reverse asynchronously.
    fn get_reverse_async(
        &self
    ) -> impl std::future::Future<Output = io::Result<PathBuf>> + Send;
}

impl GetDirAsyncExt for GetDir {
    async fn get_async(&self) -> io::Result<PathBuf> {
        let current: PathBuf = current_dir()?.into();

        match search_dir(&current, &self.targets).await {
            | Ok(Some(path)) => Ok(path),
            | _ => Err(io::Error::from(io::ErrorKind::NotFound)),
        }
    }

    async fn get_reverse_async(&self) -> io::Result<PathBuf> {
        let current: PathBuf = current_dir()?.into();

        for ancestor in current.ancestors() {
            for target in &self.targets {
                if target_exists(ancestor, target).await {
                    return Ok(ancestor.to_path_buf());
                }
            }
        }

        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}

/// Get the project root directory by searching for
/// the `target` folder and the `Cargo.lock` file.
/// Use [`get_project_root`] to handle the error automatically.
pub async fn get_project_root_directory() -> io::Result<PathBuf> {
    GetDir::new()
        .targets(vec![
            Target::Dir(DirTarget { name: "target".to_string() }),
            Target::File(FileTarget { name: "Cargo.lock".to_string() }),
        ])
        .get_reverse_async()
        .await
}

/// Get the project root directory by searching for
/// the `target` folder and the `Cargo.lock` file.
/// Use [`get_project_root_directory`] to handle the error manually.
pub async fn get_project_root() -> PathBuf {
    get_project_root_directory().await.expect("Failed to get project root")
}
