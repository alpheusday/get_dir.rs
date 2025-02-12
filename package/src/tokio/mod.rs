use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use tokio::{
    fs::{self, ReadDir},
    io,
};

use crate::{DirTarget, FileTarget, GetDir, Target};

async fn target_exists<'a>(
    path: &Path,
    target: &Target<'a>,
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
    dir: &PathBuf,
    targets: &Vec<Target<'a>>,
) -> Option<PathBuf> {
    for target in targets {
        if target_exists(dir, target).await {
            return Some(dir.to_owned());
        }
    }

    None
}

async fn search_dir<'a>(
    dir: &PathBuf,
    targets: &Vec<Target<'a>>,
) -> io::Result<Option<PathBuf>> {
    let mut entries: ReadDir = fs::read_dir(dir).await?;

    if let Some(found) = search_targets(dir, targets).await {
        return Ok(Some(found));
    }

    while let Ok(Some(entry)) = entries.next_entry().await {
        let current: PathBuf = entry.path();

        if current.is_dir() {
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

/// Trait for getting directory with tokio.
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

impl GetDirAsyncExt for GetDir<'_> {
    async fn get_async(&self) -> io::Result<PathBuf> {
        let current: PathBuf = current_dir()?;

        match search_dir(&current, &self.targets).await {
            | Ok(Some(path)) => Ok(path),
            | _ => Err(io::Error::from(io::ErrorKind::NotFound)),
        }
    }

    async fn get_reverse_async(&self) -> io::Result<PathBuf> {
        let current: PathBuf = current_dir()?;

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
            Target::Dir(DirTarget { name: "target" }),
            Target::File(FileTarget { name: "Cargo.lock" }),
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
