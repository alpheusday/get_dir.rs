#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::PathBuf};

    use get_dir::{
        get_dir_by_target, get_dir_by_target_reverse, get_project_root, Target,
    };

    #[test]
    fn test_get_dir_by_target_dir() {
        let dir: PathBuf = get_dir_by_target(Target {
            name: "src".to_string(),
            ty: get_dir::TargetType::Dir,
        })
        .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[test]
    fn test_get_dir_by_target_file() {
        let dir: PathBuf = get_dir_by_target(Target {
            name: "Cargo.toml".to_string(),
            ty: get_dir::TargetType::File,
        })
        .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[test]
    fn test_get_dir_by_target_reverse_dir() {
        let dir: PathBuf = get_dir_by_target_reverse(Target {
            name: "target".to_string(),
            ty: get_dir::TargetType::Dir,
        })
        .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[test]
    fn test_get_dir_by_target_reverse_file() {
        let dir: PathBuf = get_dir_by_target_reverse(Target {
            name: "LICENSE".to_string(),
            ty: get_dir::TargetType::File,
        })
        .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[test]
    fn test_get_project_root() {
        let dir: PathBuf = get_project_root();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }
}
