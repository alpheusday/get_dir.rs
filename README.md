# Get Dir

A utility to get directory.

## Installation

To install the package, run the following command:

```bash
cargo add get_dir
```

## Usage

Get directory by target with the following code:

```rust
use get_dir::{
    Target,
    TargetType,
    get_dir_by_target,
};

// Get the directory of the `LICENSE` file located in.
get_dir_by_target(Target { 
    name: "LICENSE".to_string(), 
    ty: TargetType::File,
});
```

## License

This project is MIT licensed, 
you can find the license file [here](./LICENSE).
