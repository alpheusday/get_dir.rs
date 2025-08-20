#[cfg(test)]
mod tests {

    use std::{env::current_dir, path::PathBuf};

    use tokio::fs::read_to_string;

    use get_dir::{
        DirTarget, FileTarget, GetDir, Target, tokio::GetDirAsyncExt,
    };

    #[tokio::test]
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

    #[tokio::test]
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

    #[tokio::test]
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

    #[tokio::test]
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

    #[tokio::test]
    async fn test_get_dir_by_target_file_in_specific_dir() {
        let dir: PathBuf = GetDir::new()
            .directory(current_dir().unwrap().join("..").join("package"))
            .target(Target::File(FileTarget::new("lib.rs")))
            .run_async()
            .await
            .unwrap();

        let content: String = read_to_string(dir.join("lib.rs")).await.unwrap();

        assert!(content.contains("# Get Dir"));
    }
}
