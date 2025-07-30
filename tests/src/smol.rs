#[cfg(test)]
mod tests {

    use std::{env::current_dir, path::PathBuf};

    use macro_rules_attribute::apply;
    use smol::fs::read_to_string;
    use smol_macros::test;

    use get_dir::{
        DirTarget, FileTarget, GetDir, Target, smol::GetDirAsyncExt,
    };

    #[apply(test!)]
    async fn test_get_dir_by_target_dir() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target::Dir(DirTarget { name: "src" })])
            .run_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[apply(test!)]
    async fn test_get_dir_by_target_file() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target::File(FileTarget { name: "Cargo.toml" })])
            .run_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[apply(test!)]
    async fn test_get_dir_by_tarrun_reverse_dir() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target::Dir(DirTarget { name: "target" })])
            .run_reverse_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[apply(test!)]
    async fn test_get_dir_by_tarrun_reverse_file() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target::File(FileTarget { name: "LICENSE" })])
            .run_reverse_async()
            .await
            .unwrap();

        let content: String =
            read_to_string(dir.join("Cargo.toml")).await.unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[apply(test!)]
    async fn test_get_dir_by_target_file_in_specific_dir() {
        let dir: PathBuf = GetDir::new()
            .directory(current_dir().unwrap().join("..").join("package"))
            .targets(vec![Target::File(FileTarget { name: "lib.rs" })])
            .run_async()
            .await
            .unwrap();

        let content: String = read_to_string(dir.join("lib.rs")).await.unwrap();

        assert!(content.contains("# Get Dir"));
    }
}
