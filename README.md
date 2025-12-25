# Rusty Grep

This project is a grep clone written in Rust just for learning purposes.

## Features
- Search for patterns in files
- Case-insensitive search (`-i`)
- Line numbers (`-n`)
- Invert matches (`-v`)
- Count matches (`-c`)
- Show filenames only (`-l`)
- Whole word matching (`-w`)

## Usage
```bash
cargo run -- "pattern" file.txt
cargo run -- -i "pattern" file1.txt file2.txt
```

## Installation
```bash
cargo build --release
```
