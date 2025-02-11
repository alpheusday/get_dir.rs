#[cfg(test)]
mod tests {

    use async_std::{fs::read_to_string, path::PathBuf};

    use get_dir::{
        async_std::{get_project_root, AsyncGetterExt},
        GetDir, Target,
    };

    #[tokio::test]
    async fn test_get_dir_by_target_dir() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target {
                name: "src".to_string(),
                r#type: get_dir::TargetType::Dir,
            }])
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
            .targets(vec![Target {
                name: "Cargo.toml".to_string(),
                r#type: get_dir::TargetType::File,
            }])
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
            .targets(vec![Target {
                name: "target".to_string(),
                r#type: get_dir::TargetType::Dir,
            }])
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
            .targets(vec![Target {
                name: "LICENSE".to_string(),
                r#type: get_dir::TargetType::File,
            }])
            .get_reverse_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[tokio::test]
    async fn test_get_project_root() {
        let dir: PathBuf = get_project_root().await;

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }
}
