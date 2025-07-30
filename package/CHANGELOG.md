## Next

### What's New

- Add support for smol with `smol` feature

## 0.4.3 (2025-05-28)

### What's Changed

- Update documentation

## 0.4.2 (2025-05-28)

### What's Changed

- Update documentation

## 0.4.1 (2025-03-12)

### What's Changed

- Downgrade `tokio` dependency from `^1.43.0` to `^1.40.0` for compatibility

## 0.4.0 (2025-02-22)

### Breaking Changes

- Remove `get_dir_by_targets`
- Remove `get_dir_by_target`
- Remove `get_dir_by_targets_reverse`
- Remove `get_dir_by_target_reverse`
- Remove `get_project_root` (use [workspace_root](https://github.com/alpheusday/workspace_root.rs) instead)
- Changes in the `Target` struct

### What's New

- Add `GetDir` struct as a handler
- Add support for `async-std` (require `async-std`/`async_std` feature)
- Add support for `tokio` (require `tokio` feature)
- Add support for specifying a target directory to run the process.

### What's Changed

- Update to 2024 edition

### Migrating from 0.3.0 to 0.4.0

For getting directory by target, use `GetDir` struct.

```diff
use get_dir::{
    Target,
-   TargetType,
-   get_dir_by_target,
+   GetDir,
+   DirTarget,
};

- get_dir_by_target(Target { 
-   name: "src", 
-   ty: TargetType::Dir,
- });

+ GetDir::new()
+   .targets(vec![
+       Target::Dir(DirTarget { 
+           name: "src",
+       }),
+   ])
+   .run();
```

For getting directory by target in reverse, use `GetDir` struct as well.

```diff
use get_dir::{
    Target,
-   TargetType,
-   get_dir_by_target_reverse,
+   GetDir,
+   FileTarget,
};

- get_dir_by_target_reverse(Target { 
-   name: "LICENSE", 
-   ty: TargetType::File,
- });

+ GetDir::new()
+   .targets(vec![
+       Target::File(FileTarget { 
+           name: "LICENSE",
+       }),
+   ])
+  .run_reverse();
```

## 0.3.0 (2024-10-13)

### Breaking Changes

- Changes in accepted value type of `name` in `Target`:
    - `String` => `&str`

### What's New

- Add different dervices for the following structs:
    - `TargetType`
    - `Target`

### What's Changed

- Updates in documentation

## 0.2.0 (2024-10-10)

### Breaking Changes

- Function rename:
    - `get_dir_by_target` => `get_dir_by_target_reverse`
    - `get_dir_by_targets` => `get_dir_by_targets_reverse`
- Old function name, new functionality:
    - `get_dir_by_target`
    - `get_dir_by_targets`

### What's New

- Add `get_project_root_directory` to handle the error manually.

## 0.1.1 (2024-09-27)

### What's Changed

- Documentation updates

## 0.1.0 (2024-09-26)

First release
