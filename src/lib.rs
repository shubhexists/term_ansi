//! # Color Formatting Crate
//!
//! This crate provides macros and functions for formatting terminal output with colors using ANSI escape codes. It supports various colors and allows nesting of colors for flexible and dynamic terminal output.
//!
//! ## Features
//!
//! - Apply predefined colors: Red, Green, Blue, White, Black, Yellow, Magenta, Cyan
//! - Custom RGB color support
//! - Nested color support
//!
//! ## Usage
//!
//! To use this crate, add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! term_ansi = "0.1.0"
//! ```
//!
//! Then you can use the provided macros to format your terminal output. Here are some examples:
//!
//! ```rust
//! use term_ansi::*; 
//!
//! // Simple color application
//! println!("{}", red!("Error message"));
//!
//! // Multiple colors in a single line
//! println!("{}", red!("Error: {}, {}", blue!("File not found"), green!("Please check your path")));
//!
//! // Nested colors
//! println!("{}", white!("This is {} with {} color", red!("red"), green!("green")));
//!
//! // Custom RGB color
//! println!("{}", rgb!(255, 0, 0, "This is red text"));
//! ```
//!
//! ## Macros
//!
//! - `red!`: Apply red color.
//! - `green!`: Apply green color.
//! - `blue!`: Apply blue color.
//! - `white!`: Apply white color.
//! - `black!`: Apply black color.
//! - `yellow!`: Apply yellow color.
//! - `magenta!`: Apply magenta color.
//! - `cyan!`: Apply cyan color.
//! - `colour!`: Apply a custom RGB color.
//!
//! ## License
//!
//! This crate is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

use std::cell::RefCell;

#[cfg(test)]
mod test;

thread_local! {
    static COLOR_CONTEXT: RefCell<Vec<String>> = RefCell::new(vec![String::from("\x1b[37m")]);
}

#[allow(dead_code)]
pub(crate) struct ColorContext;

#[allow(dead_code)]
impl ColorContext {
    pub(crate) fn push(color: &str) {
        COLOR_CONTEXT.with(|ctx| {
            ctx.borrow_mut().push(color.to_string());
        });
    }

    pub(crate) fn pop() {
        COLOR_CONTEXT.with(|ctx| {
            ctx.borrow_mut().pop();
        });
    }

    pub(crate) fn current_color() -> String {
        COLOR_CONTEXT.with(|ctx| {
            ctx.borrow()
                .last()
                .cloned()
                .unwrap_or_else(|| String::from("\x1b[37m"))
        })
    }
}

#[macro_export]
/// Applies a color code to the provided format string.
///
/// # Arguments
///
/// * `color_code` - The ANSI escape code for the desired color.
/// * `args` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// let colored_text = apply_color!("\x1b[31m", "This is {}", "red");
/// println!("{}", colored_text);
/// ```
///
/// # Notes
///
/// The color context is managed using `ColorContext` to ensure colors are correctly nested.
macro_rules! apply_color {
    ($color_code:expr, $($arg:tt)*) => {{
        $crate::ColorContext::push($color_code);
        let result = format!("{}{}", $color_code, format!($($arg)*));
        $crate::ColorContext::pop();
        format!("{}{}", result, $crate::ColorContext::current_color())
    }};
}

#[macro_export]
/// Applies red color to the provided format string.
///
/// # Arguments
///
/// * `args` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", red!("This is {} text", "red"));
/// ```
macro_rules! red {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[31m", $($arg)*)
    }};
}

#[macro_export]
/// Applies green color to the provided format string.
///
/// # Arguments
///
/// * `args` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", green!("This is {} text", "green"));
/// ```
macro_rules! green {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[32m", $($arg)*)
    }};
}

#[macro_export]
/// Applies blue color to the provided format string.
///
/// # Arguments
///
/// * `args` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", blue!("This is {} text", "blue"));
/// ```
macro_rules! blue {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[34m", $($arg)*)
    }};
}

#[macro_export]
/// Applies white color to the provided format string.
///
/// # Arguments
///
/// * `args` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", white!("This is {} text", "white"));
/// ```
macro_rules! white {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[37m", $($arg)*)
    }};
}

#[macro_export]
/// Applies black color to the provided format string.
///
/// # Arguments
///
/// * `args` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", black!("This is {} text", "black"));
/// ```
macro_rules! black {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[30m", $($arg)*)
    }};
}

#[macro_export]
/// Applies yellow color to the provided format string.
///
/// # Arguments
///
/// * `args` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", yellow!("This is {} text", "yellow"));
/// ```
macro_rules! yellow {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[33m", $($arg)*)
    }};
}

#[macro_export]
/// Applies magenta color to the provided format string.
///
/// # Arguments
///
/// * `args` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", magenta!("This is {} text", "magenta"));
/// ```
macro_rules! magenta {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[35m", $($arg)*)
    }};
}

#[macro_export]
/// Applies cyan color to the provided format string.
///
/// # Arguments
///
/// * `args` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", cyan!("This is {} text", "cyan"));
/// ```
macro_rules! cyan {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[36m", $($arg)*)
    }};
}

#[macro_export]
/// Applies a custom RGB color to the provided format string.
///
/// # Arguments
///
/// * `r` - The red component (0-255).
/// * `g` - The green component (0-255).
/// * `b` - The blue component (0-255).
/// * `args` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", colour!(255, 0, 0, "This is {} text", "bright red"));
/// ```
macro_rules! colour {
    ($r:expr, $g:expr, $b:expr, $($arg:tt)*) => {{
        let color_code = format!("\x1b[38;2;{};{};{}m", $r, $g, $b);
        $crate::apply_color!(color_code, $($arg)*)
    }};
}
