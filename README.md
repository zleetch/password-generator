# Password Generator

A simple and customizable password generator implemented in Rust. Generate secure passwords with control over length, character (uppercase & lowercase), number, and symbol.

## Features

- **Customizable Password Length**: Specify the length of the password.
- **Character Set Options**:
  - Include uppercase letters
  - Include lowercase letters
  - Include numbers
  - Include symbol characters
- **Secure Randomization**: Uses Rust's `rand` crate for secure random number generation.

## Installation

1. Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).
2. Clone this repository:

   ```bash
   git clone https://github.com/zleetch/password-generator.git
   cd password-generator
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

Run the application from the command line:

```bash
cargo run --release -- --length 16 --uppercase --lowercase --numbers --special
```

### Options

| Option Short | Option Long   | Description                               | Default  |
| ------------ | ------------- | ----------------------------------------- | -------- |
| `-L`         | `--length`    | Length of the password                    | 8        |
| `-u`         | `--uppercase` | Include uppercase letters (A-Z)           | Disabled |
| `-l`         | `--lowercase` | Include lowercase letters (a-z)           | Disabled |
| `-n`         | `--numbers`   | Include numbers (0-9)                     | Disabled |
| `-s`         | `--symbol`    | Include special characters (!@#$%^&\*...) | Disabled |

Example:

```bash
cargo run --release -- --length 20 --uppercase --numbers
```

This generates a 20-character password with uppercase letters and numbers.

## Code Overview

The project is structured as follows:

```
password-generator-rust/
├── src/
│   ├── main.rs  # Entry point of the application and Logic for password generation
├── Cargo.toml  # Project dependencies and metadata
```

### Key Dependencies

- [`rand`](https://crates.io/crates/rand): For generating secure random numbers.
- [`clap`](https://crates.io/crates/clap): For parsing command-line arguments.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

---

Happy password generating! 🎉
