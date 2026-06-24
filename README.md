![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

# Command-Line Rust Code-Along

This repository contains my code-along implementations from the book **Command-Line Rust**.

It currently includes three small CLI programs:

1. `echor`: a Rust version of `echo`
2. `catr`: a Rust version of `cat`
3. `headr`: a Rust version of `head`

The goal is to practice idiomatic Rust for CLI tooling, argument parsing with `clap`, and test-driven development with integration tests.

## Project Layout

This repo is organized as separate crates:

- `echor/`: echo-like command
- `catr/`: cat-like command
- `headr/`: head-like command

Each crate has its own:

- `Cargo.toml`
- `src/main.rs`
- `tests/` integration tests

## Prerequisites

- Rust toolchain (recommended via `rustup`)
- Cargo (installed with Rust)

Verify your setup:

```bash
rustc --version
cargo --version
```

## Quick Start

From the repository root, run each crate by pointing Cargo to its manifest.

### Run `echor`

```bash
cargo run --manifest-path echor/Cargo.toml -- Hello there
cargo run --manifest-path echor/Cargo.toml -- -n Hello there
```

### Run `catr`

```bash
cargo run --manifest-path catr/Cargo.toml -- catr/tests/inputs/fox.txt
cargo run --manifest-path catr/Cargo.toml -- -n catr/tests/inputs/spiders.txt
cargo run --manifest-path catr/Cargo.toml -- -b catr/tests/inputs/the-bustle.txt
```

You can also pass `-` to `catr` to read from standard input.

```bash
printf "line one\n\nline two\n" | cargo run --manifest-path catr/Cargo.toml -- -b -
```

## Testing

Run tests per crate:

```bash
cargo test --manifest-path echor/Cargo.toml
cargo test --manifest-path catr/Cargo.toml
```

Both crates use integration tests in `tests/cli.rs` with:

- `assert_cmd` for running binaries
- `predicates` for output assertions
- `pretty_assertions` for readable diffs

## Crate Notes

### `echor`

`echor` supports:

- one or more text arguments
- `-n` / `--omit-newline` to suppress trailing newline

Examples:

```bash
cargo run --manifest-path echor/Cargo.toml -- "Hello there"
cargo run --manifest-path echor/Cargo.toml -- -n "Hello" "there"
```

### `catr`

`catr` supports:

- one or more input files (or `-` for stdin)
- `-n` / `--number` to number all lines
- `-b` / `--number-nonblank` to number nonblank lines only

Examples:

```bash
cargo run --manifest-path catr/Cargo.toml -- catr/tests/inputs/fox.txt
cargo run --manifest-path catr/Cargo.toml -- --number catr/tests/inputs/spiders.txt
cargo run --manifest-path catr/Cargo.toml -- --number-nonblank catr/tests/inputs/the-bustle.txt
```

## Regenerating Expected Output Fixtures

Each crate includes a helper script to regenerate expected output files used by tests:

```bash
(cd echor && ./mk-outs.sh)
(cd catr && ./mk-outs.sh)
```

## Why This Repo Exists

This is a learning repository for:

- building robust CLI tools in Rust
- designing small, testable programs
- comparing behavior against classic Unix utilities

As I continue through the book, more crates and exercises can be added in the same style.

