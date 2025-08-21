# Get Dir

A utility to get directory.

This utility searches for a target directory by checking for any directories or files that match the provided input.

## Installation

Install this package as a dependency in the project:

```bash
cargo add get_dir
```

## Usage

Get directory by target with the following code:

```rust
use std::path::PathBuf;

use get_dir::{
    GetDir,
    Target,
    DirTarget,
};

let path: PathBuf = GetDir::new()
    .target(
        Target::Dir(DirTarget::new("src")),
    )
    .run()
    .unwrap();
```

Or get directory by target in reverse with the following code:

```rust
use std::path::PathBuf;

use get_dir::{
    GetDir,
    Target,
    FileTarget,
};

let path: PathBuf = GetDir::new()
    .target(
        Target::File(FileTarget::new("LICENSE")),
    )
    .run_reverse()
    .unwrap();
```

Async version also available with `async_std`, `smol` and `tokio` features:

```rust
use std::path::PathBuf;

use get_dir::{
    GetDir,
    Target,
    DirTarget,
    // async_std
    async_std::GetDirAsyncExt,
    // smol
    smol::GetDirAsyncExt,
    // tokio
    tokio::GetDirAsyncExt,
};

let path: PathBuf = GetDir::new()
    .target(
        Target::Dir(DirTarget::new("src")),
    )
    .run_async()
    .await
    .unwrap();
```

## See also

For resolving the root of a workspace, consider using the [`workspace_root`](https://github.com/alpheusday/workspace_root.rs) library.

## License

This project is licensed under the terms of the MIT license.
