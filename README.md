### MiniGrep

MiniGrep is a simple command-line utility written in Rust for searching text in files.

#### Usage

To use MiniGrep, run the following command in your terminal:

```bash
minigrep <query> <file_path>
```

- `<query>`: The text to search for.
- `<file_path>`: The path to the file in which to search.

#### Configuration

MiniGrep supports the following configuration options:

- **Case Sensitivity**: By default, searches are case-sensitive. To perform case-insensitive searches, set the `IGNORE_CASE` environment variable.

#### Examples

```bash
# Perform a case-sensitive search for the word "rust" in the file "example.txt"
minigrep rust example.txt

# Perform a case-insensitive search for the word "Rust" in the file "example.txt"
IGNORE_CASE=true minigrep Rust example.txt
```

#### Building and Running

To build and run MiniGrep, follow these steps:

1. Ensure you have Rust installed on your system.
2. Clone the repository: `git clone https://github.com/yourusername/minigrep.git`
3. Navigate to the project directory: `cd minigrep`
4. Build the project: `cargo build --release`
5. Run the executable: `./target/release/minigrep <query> <file_path>`

#### Testing

MiniGrep includes unit tests to ensure correctness. Run the tests with:

```bash
cargo test
```
