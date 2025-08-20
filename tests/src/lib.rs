pub mod async_std;

pub mod tokio;

#[cfg(test)]
mod tests {
    use std::{env::current_dir, fs::read_to_string, path::PathBuf};

    use get_dir::{DirTarget, FileTarget, GetDir, Target};

    #[test]
    fn test_get_dir_by_target_dir() {
        let dir: PathBuf = GetDir::new()
            .target(Target::Dir(DirTarget::new("src")))
            .run()
            .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[test]
    fn test_get_dir_by_target_file() {
        let dir: PathBuf = GetDir::new()
            .target(Target::File(FileTarget::new("Cargo.toml")))
            .run()
            .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[test]
    fn test_get_dir_by_target_reverse_dir() {
        let dir: PathBuf = GetDir::new()
            .target(Target::Dir(DirTarget::new("target")))
            .run_reverse()
            .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[test]
    fn test_get_dir_by_target_reverse_file() {
        let dir: PathBuf = GetDir::new()
            .target(Target::File(FileTarget::new("LICENSE")))
            .run_reverse()
            .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[test]
    fn test_get_dir_by_target_file_in_specific_dir() {
        let dir: PathBuf = GetDir::new()
            .directory(current_dir().unwrap().join("..").join("package"))
            .target(Target::File(FileTarget::new("lib.rs")))
            .run()
            .unwrap();

        let content: String = read_to_string(dir.join("lib.rs")).unwrap();

        assert!(content.contains("# Get Dir"));
    }
}
