#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use get_dir::{
        tokio::GetDirAsyncExt, DirTarget, FileTarget, GetDir, Target,
    };
    use tokio::fs::read_to_string;

    #[tokio::test]
    async fn test_get_dir_by_target_dir() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target::Dir(DirTarget { name: "src" })])
            .get_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[tokio::test]
    async fn test_get_dir_by_target_file() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target::File(FileTarget { name: "Cargo.toml" })])
            .get_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[tokio::test]
    async fn test_get_dir_by_target_reverse_dir() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target::Dir(DirTarget { name: "target" })])
            .get_reverse_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[tokio::test]
    async fn test_get_dir_by_target_reverse_file() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target::File(FileTarget { name: "LICENSE" })])
            .get_reverse_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }
}
