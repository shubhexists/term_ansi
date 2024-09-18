# 🌈 term_ansi

[![Crates.io](https://img.shields.io/crates/v/term_ansi.svg)](https://crates.io/crates/term_ansi)
[![Documentation](https://docs.rs/term_ansi/badge.svg)](https://docs.rs/term_ansi)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> Colorize your terminal output with ease! 🎨✨

`term_ansi` is a Rust crate that provides a simple and intuitive way to add colors and formatting to your terminal output using ANSI escape codes. With support for various predefined colors and custom RGB values, you can make your CLI applications more visually appealing and user-friendly.

## ⚠️ Important Version Notice

**Versions before v0.2.2 are unstable.**

**v0.2.2 and above are fully tested and stable. We strongly recommend using this version or later for production use.**

## 📚 Table of Contents

- [Features](#-features)
- [Installation](#-installation)
- [Usage](#-usage)
  - [Basic Colors](#basic-colors)
  - [Nested Colors](#nested-colors)
  - [Custom RGB Colors](#custom-rgb-colors)
- [Available Macros](#-available-macros)
- [Examples](#-examples)
- [Contributing](#-contributing)
- [License](#-license)

## ✨ Features

- 🎨 Easy-to-use macros for applying colors
- 🌈 Support for 8 predefined colors
- 🔢 Custom RGB color support
- 🪆 Nested color application
- 🧵 Thread-safe color context management

## 📦 Installation

Add `term_ansi` to your `Cargo.toml`:

```toml
[dependencies]
term_ansi = "0.2.2"  # Make sure to use version 0.2.2 or later
```

## 🚀 Usage

First, import the crate in your Rust file:

```rust
use term_ansi::*;
```

### Basic Colors

Apply colors to your text using the provided macros:

```rust
println!("{}", red!("This is red text"));
println!("{}", green!("This is green text"));
println!("{}", blue!("This is blue text"));
```

### Nested Colors

You can nest color macros for more complex formatting:

```rust
println!("{}", white!("This is {} with {} color", red!("red"), green!("green")));
```

### Custom RGB Colors

Use the `colour!` macro to apply custom RGB colors:

```rust
println!("{}", colour!(255, 128, 0, "This is orange text"));
```

## 📚 Available Macros

- `red!`: Apply red color
- `green!`: Apply green color
- `blue!`: Apply blue color
- `white!`: Apply white color
- `black!`: Apply black color
- `yellow!`: Apply yellow color
- `magenta!`: Apply magenta color
- `cyan!`: Apply cyan color
- `colour!`: Apply a custom RGB color

## 💡 Examples

### Simple Error Message

```rust
println!("{}", red!("Error: {}", "File not found"));
```

### Colorful Status Message

```rust
println!("{} {} {}", green!("✓"), blue!("Building project:"), yellow!("in progress"));
```

### Nested Color Formatting

```rust
println!("{}", white!("Status: {} | {}", green!("OK"), red!("Failed: {}", 3)));
```

### Custom Color Gradient

```rust
for i in 0..=255 {
    print!("{}", colour!(i, 0, 255 - i, "█"));
}
println!();
```

## 🤝 Contributing

Contributions are welcome! Do discuss any improvements in the Issues first : )
Please make sure to update tests as appropriate and adhere to the existing coding style.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Happy coding! Hope it helps 🦀
