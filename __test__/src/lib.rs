#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::PathBuf};

    use get_dir::{get_dir_by_target, get_project_root, Target};

    #[test]
    fn test_get_dir_by_target() {
        let dir: PathBuf = get_dir_by_target(Target {
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
