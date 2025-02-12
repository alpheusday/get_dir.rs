# Get Dir

An utility to get directory.

This utility searches for a target directory by checking for any directories or files that match the provided input.

## Usage

Get directory by target with the following code:

```rust
use get_dir::{
    GetDir,
    Target,
    DirTarget,
};

GetDir::new()
    .targets(vec![
        Target::Dir(DirTarget {
            name: "src".to_string(),  
        }),
    ])
    .get();
```

Or get directory by target in reverse with the following code:

```rust
use get_dir::{
    GetDir,
    Target,
    FileTarget,
};

GetDir::new()
    .targets(vec![
        Target::File(FileTarget {
            name: "LICENSE".to_string(),  
        }),
    ])
    .get_reverse();
```

Async version also available with `async-std`/`async_std` and `tokio` features:

```rust
// This is a `async-std` example

use get_dir::{
    GetDir,
    Target,
    DirTarget,
    async_std::GetDirAsyncExt,
};

GetDir::new()
    .targets(vec![
        Target::Dir(DirTarget {
            name: "src".to_string(),  
        }),
    ])
    .get_async()
    .await;
```

```rust
// This is a `tokio` example

use get_dir::{
    GetDir,
    Target,
    DirTarget,
    tokio::GetDirAsyncExt,
};

GetDir::new()
    .targets(vec![
        Target::Dir(DirTarget {
            name: "src".to_string(),
        }),
    ])
    .get_async()
    .await;
```

## License

This project is licensed under the terms of the MIT license.
