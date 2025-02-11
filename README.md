# Get Dir

An utility to get directory.

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
    Target,
    TargetType,
    GetDir,
};

GetDir::new()
    .targets(vec![Target {
        name: "src".to_string(),  
        r#type: TargetType::Dir,
    }])
    .get();
```

Or get directory by target in reverse with the following code:

```rust
use get_dir::{
    Target,
    TargetType,
    GetDir,
};

GetDir::new()
    .targets(vec![Target {
        name: "LICENSE".to_string(),  
        r#type: TargetType::File,
    }])
    .get_reverse();
```

Async version also available with `async-std`/`async_std` and `tokio` features:

```rust
// This is a `async-std` example

use get_dir::{
    Target,
    TargetType,
    GetDir,
    async_std::AsyncGetterExt,
};

GetDir::new()
    .targets(vec![Target {
        name: "src".to_string(),  
        r#type: TargetType::Dir,
    }])
    .get_async()
    .await;
```

```rust
// This is a `tokio` example

use get_dir::{
    Target,
    TargetType,
    GetDir,
    tokio::AsyncGetterExt,
};

GetDir::new()
    .targets(vec![Target {
        name: "src".to_string(),  
        r#type: TargetType::Dir,
    }])
    .get_async()
    .await;
```

## License

This project is licensed under the terms of the MIT license.
