# Get Dir

A utility to get directory.

This utility searches for a target directory by checking for any directories or files that match the provided input.

## Installation

To install this package, run the following command:

```bash
cargo add get_dir
```

## Usage

Get directory by target with the following code:

```rust
use get_dir::{
    GetDir,
    Target,
    DirTarget,
};

GetDir::new()
    .target(
        Target::Dir(DirTarget::new("src")),
    )
    .run();
```

Or get directory by target in reverse with the following code:

```rust
use get_dir::{
    GetDir,
    Target,
    FileTarget,
};

GetDir::new()
    .target(
        Target::File(FileTarget::new("LICENSE")),
    )
    .run_reverse();
```

Async version also available with `async_std`, `smol` and `tokio` features:

```rust
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

GetDir::new()
    .target(
        Target::Dir(DirTarget::new("src")),
    )
    .run_async()
    .await;
```

## License

This project is licensed under the terms of the MIT license.
