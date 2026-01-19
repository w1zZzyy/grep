# Grep

A simple Rust implementation of a `grep`-like utility with **multithreading support**.  
It scans files or directories for a given pattern and prints the matching lines, file paths, and match counts.  

This project uses **Crossbeam channels** for multithreading.

---

## Features

- Search for a pattern in a **file or directory**  
- Supports **multiple threads** (`--threads`) for faster scanning  
- Outputs results to **terminal** or **file** (`--output`)  
- Shows **line numbers** and **file paths** for matches  

---

## Build

```bash
git clone https://github.com/w1zZzyy/grep.git
cd grep
cargo build --release
```

## Run 
```bash 
cargo run -- <path> <pattern> [options]
```

### Required arguments
- <path> — file or directory to search in
- <pattern> — pattern to search for

### Optional arguments 
- --threads <num_threads> — number of threads to use (optional)
- --output <file_path> — write results to a file (optional, file must already exist)

## Examples 

```bash
cargo run -- ./example.txt "hello" # Search for a pattern in a single file 
cargo run -- ./my_project "fn main" --threads 4 # Search for a pattern in a directory using multiple threads
cargo run -- ./my_project "TODO" --output out.txt # Search in a directory and save results to a file 
cargo run -- ./dir "pattern" --threads 8 --output out.txt # MultiThreading + Output dir
```

## Notes / Limitations

- **Hidden directories** (names starting with `.`) are **skipped** during scanning.  
- Only **regular files** are scanned; **symlinks are followed**, but other special files (sockets, devices) are ignored.  
- Files must be valid **UTF-8** text; **binary files** may cause errors.  
- The **output file** must **already exist** if using `--output`.