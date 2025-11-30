[< Back](./README.md)

# Contributing to Get Dir

Thanks for your interest in contributing!

This is a guideline for contributing to Get Dir.

## Before the Contribution

Please install the following dependencies:

| Dependencies                                   | Description                            |
| ---------------------------------------------- | -------------------------------------- |
| [Rust](https://rust-lang.org/) 1.85.0          | Programming language                   |
| [just](https://just.systems)                   | Command runner                         |
| [ls-lint](https://ls-lint.org/)                | Linting tool for directories and files |
| [typos-cli](https://github.com/crate-ci/typos) | Spell checker                          |

## Commands

The following commands are available:

### Default Command

This command will do linting, formatting and testing.

```sh
just
```

### Linting

This command will lint the code.

```sh
just lint
```

### Formatting

This command will format the code.

```sh
just fmt
```

### Testing

This command will run all tests.

```sh
just test
```

### Benchmarking

This command will run all benchmarks.

```sh
just bench
```

### Cleaning

This command will clean the unnecessary files.

```sh
just clean
```

## Committing

When committing changes to the code, use the following prefixes:

- `chore`: updates in dependencies/tools
- `build`: changes to the build system
- `fix`: fixes a bug
- `feat`: adds a new feature
- `refactor`: other code changes
- `perf`: performance improvements
- `security`: security related changes
- `style`: style changes
- `test`: adding or updating tests
- `docs`: documentation only changes
- `ci`: CI configuration updates
- `release`: new version release

For example:

```
feat: add xxx feature
docs: fix typos
```
