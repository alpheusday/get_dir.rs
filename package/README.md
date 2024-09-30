# Get Dir

An utility to get directory.

A Directory searching utility that will check whether the target file or directory exists in the directory. The search process will start from the current directory and go to the root. Therefore, targets in other subdirectories will not be found, but a better performance is expected.

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

This project is MIT licensed, you can find the license file [here](https://github.com/alpheustangs/get_dir.rs/blob/main/LICENSE).
