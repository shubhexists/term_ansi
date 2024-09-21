# 🌈 term_ansi

[![Crates.io](https://img.shields.io/crates/v/term_ansi.svg)](https://crates.io/crates/term_ansi)
[![Documentation](https://docs.rs/term_ansi/badge.svg)](https://docs.rs/term_ansi)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> Colorize your terminal output with ease! 🎨✨

`term_ansi` is a lightweight Rust crate that provides a simple and intuitive way to add colors and formatting to your terminal output using ANSI escape codes. With support for various predefined colors, custom RGB values, and text styles, you can make your CLI applications more visually appealing and user-friendly.

## 📚 Table of Contents

- [Features](#-features)
- [Installation](#-installation)
- [Usage](#-usage)
  - [Basic Colors](#basic-colors)
  - [Background Colors](#background-colors)
  - [Text Styles](#text-styles)
  - [Custom Colors](#custom-colors)
  - [Nested Formatting](#nested-formatting)
- [Available Macros](#-available-macros)
- [Examples](#-examples)
- [Contributing](#-contributing)
- [License](#-license)

## ✨ Features

- 🎨 Easy-to-use macros for applying colors and styles
- 🌈 Support for 8 predefined foreground and background colors
- 🔢 Custom RGB, HSL, and HSV color support for text and background
- 🧵 Text styles: Bold, Italic, Underline
- 🪆 Nested color and style application
- 🧪 Thread-safe color context management

## 📦 Installation

Add `term_ansi` to your `Cargo.toml`:

```toml
[dependencies]
term_ansi = "0.2.5"
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

### Background Colors

Apply background colors to your text:

```rust
println!("{}", bg_yellow!("This has a yellow background"));
println!("{}", bg_cyan!("This has a cyan background"));
```

### Text Styles

Apply text styles:

```rust
println!("{}", bold!("This text is bold"));
println!("{}", italic!("This text is italic"));
println!("{}", underline!("This text is underlined"));
```

### Custom Colors

Use custom RGB, HSL, or HSV colors:

```rust
println!("{}", rgb!(255, 128, 0, "This is orange text"));
println!("{}", hsl!(120.0, 1.0, 0.5, "This is green text"));
println!("{}", bg_hsv!(240.0, 1.0, 1.0, "This has a blue background"));
```

### Nested Formatting

Combine multiple styles and colors:

```rust
println!("{}", red!("{}", bg_white!("{}" bold!("Important: {}"), "Read this!")));
```

## 📚 Available Macros

### Text Colors
- `red!`, `green!`, `blue!`, `white!`, `black!`, `yellow!`, `magenta!`, `cyan!`

### Background Colors
- `bg_red!`, `bg_green!`, `bg_blue!`, `bg_white!`, `bg_black!`, `bg_yellow!`, `bg_magenta!`, `bg_cyan!`

### Text Styles
- `bold!`, `italic!`, `underline!`

### Custom Colors
- `rgb!`, `hsl!`, `hsv!`: Custom foreground colors
- `bg_rgb!`, `bg_hsl!`, `bg_hsv!`: Custom background colors

## 💡 Examples

### Error Message with Style

```rust
println!("{}", red!("{}", bold!("Error: {}"), "File not found"));
```

### Colorful Status Message

```rust
println!("{} {} {}", green!("✓"), blue!("Building project:"), yellow!("in progress"));
```

### Custom Color Gradient

```rust
for i in 0..=255 {
    print!("{}", rgb!(i, 0, 255 - i, "█"));
}
println!();
```

### Complex Nested Formatting

```rust
println!("{}",
    bg_blue!("{}",
        white!("{}",
            bold!("Status: {} | {}",
                green!("OK"),
                red!("{}", underline!("Failed: {}"), 3)
            )
        )
    )
);
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Happy colorful coding! 🦀🌈