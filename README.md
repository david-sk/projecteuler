# Project Euler

Solving Project Euler problems in the Rust programming language!

Project Euler website: <https://projecteuler.net>

Rust programming language website: <https://www.rust-lang.org>

## Setup

1. Install Rust:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Run a problem, for example:

```
cargo run 1 v1
```

Or, optimized build and run:

```
cargo build --release && ./target/release/projecteuler 1 v1
```

See usage explained in `display_usage` function of `src/main.rs` file.
Note that this main file must be updated when adding new problems.
