# Minigrep
Minigrep is a simple, lightweight grep clone written in Rust. Inspired by the classic command-line tool `grep`, Minigrep allows you to search for text within files, with support for case-sensitive and case-insensitive searches. This project is based on the I/O project example from "The Rust Programming Language" book, serving both as an educational tool and a practical utility.

## Features
- Search for text within a file.
- Case-sensitive and case-insensitive search modes.
- Customizable through environment variables.

## Getting Started

### Prerequisites
- Rust and Cargo (the Rust build system and package manager). You can install both with [rustup](https://rustup.rs/).

### Building
Clone this repository, and navigate to the project directory:

```sh
git clone https://your-repository-url/minigrep.git
cd minigrep
```

Build the project with Cargo:

```sh
cargo build --release
```

### Running
To search within a file, use:

```sh
cargo run <query> <file_path>
```

For a case-insensitive search, set the `IGNORE_CASE` environment variable before running the command:

```sh
IGNORE_CASE=1 cargo run <query> <file_path>
```

### Examples
Searching for the word "Rust" in `poem.txt`:

```sh
cargo run Rust poem.txt
```

Performing a case-insensitive search for "rust" in `poem.txt`:

```sh
IGNORE_CASE=1 cargo run rust poem.txt
```

## Testing
Run the tests to ensure everything is working as expected:

```sh
cargo test
```

## Reference
This project follows the guide from "The Rust Programming Language" book, specifically the chapter on creating an I/O project. For more information and to follow along with the project's development steps, visit [The Rust Programming Language - An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).