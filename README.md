# litcat

**litcat** is a simple Rust CLI tool for viewing patch and diff files with  highlighting. It colors added (`+`), removed (`-`), and context lines to make git diffs easier to read in your terminal.


## Features

- Colors added lines in **green**
- Colors removed lines in **red**
- Colors diff headers and hunk markers
- Reads from a file or standard input (pipe)


## Usage

### From a patch file

```sh
litcat patch.diff
```

### From git diff output

```sh
git diff | litcat
```

## Example Output

- Lines starting with `+` (but not `+++`) are **green**
- Lines starting with `-` (but not `---`) are **red**
- Diff headers (`+++`, `---`) are **cyan**
- Hunk markers (`@@ ... @@`) are **yellow**
- All other lines are default color


## Installation

### 1. Build from Source

1. Clone the repo:
    ```sh
    git clone https://github.com/shan-shaji/litcat.git
    cd litcat
    ```
2. Build:
    ```sh
    cargo build --release
    ```
3. Run as shown above.

### 2. Install from crates.io

you can install directly using cargo:

```sh
cargo install litcat
```

## Dependencies

- [clap](https://crates.io/crates/clap) for CLI argument parsing
- [colored](https://crates.io/crates/colored) for colored terminal output



