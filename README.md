# Advent of Code 2025

This repository contains my Advent of Code 2025 solutions implemented in Rust.

## Quickstart

- Build the project:

```bash
cargo build
```

- Run a specific day and part (examples live in `data/dayX/example.txt`):

```bash
# Usage: cargo run -- <day> <part> <use example data (y/n)>
# Example: run day 1, part 1 using example input
cargo run -- 1 1 y
# Example 2: run day 1, part 2 using actual input
cargo run -- 1 2 n
```

If you leave the third argument off, the code will use the actual input files by default.

## Data layout

- `data/dayN/example.txt` — the example input from the puzzle description
- `data/dayN/actual.txt` — your puzzle input for that day

When running the program with the `y` flag it will use `example.txt`; otherwise it uses `actual.txt`.

## Project layout

- `src/main.rs` — CLI entry point. Signature: `cargo run -- <day> <part> <y/n>`.
- `src/dayN/mod.rs` — solution modules for each day.
- `src/shared/mod.rs` — shared helpers and types.

## Whatever

Feel free to use these solutions for reference