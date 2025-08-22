#[cfg(test)]
mod tests {

    use std::env::current_dir;

    use async_std::{fs::read_to_string, path::PathBuf};

    use get_dir::{
        DirTarget, FileTarget, GetDir, Target, async_std::GetDirAsyncExt,
    };

    #[async_std::test]
    async fn test_get_dir_by_target_dir() {
        let dir: PathBuf = GetDir::new()
            .target(Target::Dir(DirTarget::new("src")))
            .run_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[async_std::test]
    async fn test_get_dir_by_target_file() {
        let dir: PathBuf = GetDir::new()
            .target(Target::File(FileTarget::new("Cargo.toml")))
            .run_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[async_std::test]
    async fn test_get_dir_by_tarrun_reverse_dir() {
        let dir: PathBuf = GetDir::new()
            .target(Target::Dir(DirTarget::new("target")))
            .run_reverse_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[async_std::test]
    async fn test_get_dir_by_tarrun_reverse_file() {
        let dir: PathBuf = GetDir::new()
            .target(Target::File(FileTarget::new("LICENSE")))
            .run_reverse_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[async_std::test]
    async fn test_get_dir_by_target_file_in_specific_dir() {
        let dir: PathBuf = GetDir::new()
            .dir(current_dir().unwrap().join("..").join("package"))
            .target(Target::File(FileTarget::new("lib.rs")))
            .run_async()
            .await
            .unwrap();

        let content: String = read_to_string(dir.join("lib.rs")).await.unwrap();

        assert!(content.contains("# Get Dir"));
    }

    #[async_std::test]
    async fn test_get_dir_with_depth_limit() {
        let dir: PathBuf = current_dir().unwrap().into();
        let target: Target = Target::File(FileTarget::new("lib.rs"));

        if let Ok(_) = GetDir::new()
            .dir(&dir)
            .depth(1)
            .target(target.clone())
            .run_async()
            .await
        {
            panic!("Should fail");
        }

        if let Err(_) = GetDir::new().dir(&dir).depth(2).target(target).run() {
            panic!("Should succeed");
        }
    }

    #[async_std::test]
    async fn test_get_dir_reverse_with_depth_limit() {
        let dir: PathBuf = current_dir().unwrap().into();
        let target: Target = Target::File(FileTarget::new("Cargo.lock"));

        if let Ok(_) = GetDir::new()
            .dir(&dir)
            .depth(1)
            .target(target.clone())
            .run_reverse_async()
            .await
        {
            panic!("Should fail");
        }

        if let Err(_) =
            GetDir::new().dir(&dir).depth(2).target(target).run_reverse()
        {
            panic!("Should succeed");
        }
    }
}
