//! # Color Formatting Crate
//!
//! This crate provides macros and functions for formatting terminal output with colors,
//! background colors, and styles using ANSI escape codes. It supports various colors
//! and allows nesting of colors for flexible and dynamic terminal output.
//!
//! ## Features
//!
//! - Apply predefined colors: Red, Green, Blue, White, Black, Yellow, Magenta, Cyan
//! - Apply background colors
//! - Apply text styles: Bold, Italic, Underline
//! - Custom RGB color support for text and background
//! - HSL and HSV color support for text and background
//! - Nested color support
//!
//! ## Usage
//!
//! To use this crate, add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! term_ansi = "0.2.5"
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
//! // Background color
//! println!("{}", bg_blue!("Blue background"));
//!
//! // Styles
//! println!("{}", bold!("Bold text"));
//!
//! // Combining color, background, and style
//! println!("{}", red!("{}", bg_yellow!("{}", bold!("Warning: Important message"))));
//!
//! // Custom RGB color
//! println!("{}", rgb!(255, 0, 0, "This is red text"));
//!
//! // HSL and HSV colors
//! println!("{}", hsl!(120.0, 1.0, 0.5, "This is green text"));
//! println!("{}", bg_hsv!(240.0, 1.0, 1.0, "This has a blue background"));
//! ```
//!
//! ## Macros
//!
//! ### Text Colors
//! - `red!`, `green!`, `blue!`, `white!`, `black!`, `yellow!`, `magenta!`, `cyan!`
//!
//! ### Background Colors
//! - `bg_red!`, `bg_green!`, `bg_blue!`, `bg_white!`, `bg_black!`, `bg_yellow!`, `bg_magenta!`, `bg_cyan!`
//!
//! ### Styles
//! - `bold!`, `italic!`, `underline!`
//!
//! ### Custom Colors
//! - `rgb!`, `bg_rgb!`: Apply custom RGB colors for text and background.
//! - `hsl!`, `hsv!`, `bg_hsl!`, `bg_hsv!`: Apply colors using HSL or HSV color models for text and background.
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
pub struct ColorContext;

#[allow(dead_code)]
impl ColorContext {
    pub fn push(color: &str) {
        COLOR_CONTEXT.with(|ctx| {
            ctx.borrow_mut().push(color.to_string());
        });
    }

    pub fn pop() {
        COLOR_CONTEXT.with(|ctx| {
            ctx.borrow_mut().pop();
        });
    }

    pub fn current_color() -> String {
        COLOR_CONTEXT.with(|ctx| {
            ctx.borrow()
                .last()
                .cloned()
                .unwrap_or_else(|| String::from("\x1b[37m"))
        })
    }
}

pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c: f64 = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x: f64 = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m: f64 = l - c / 2.0;

    let (r, g, b): (f64, f64, f64) = match h {
        h if h >= 0.0 && h < 60.0 => (c, x, 0.0),
        h if h >= 60.0 && h < 120.0 => (x, c, 0.0),
        h if h >= 120.0 && h < 180.0 => (0.0, c, x),
        h if h >= 180.0 && h < 240.0 => (0.0, x, c),
        h if h >= 240.0 && h < 300.0 => (x, 0.0, c),
        h if h >= 300.0 && h <= 360.0 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    let r: u8 = ((r + m) * 255.0).round() as u8;
    let g: u8 = ((g + m) * 255.0).round() as u8;
    let b: u8 = ((b + m) * 255.0).round() as u8;

    (r, g, b)
}

pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c: f64 = v * s;
    let x: f64 = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m: f64 = v - c;

    let (r, g, b): (f64, f64, f64) = match h {
        h if h >= 0.0 && h < 60.0 => (c, x, 0.0),
        h if h >= 60.0 && h < 120.0 => (x, c, 0.0),
        h if h >= 120.0 && h < 180.0 => (0.0, c, x),
        h if h >= 180.0 && h < 240.0 => (0.0, x, c),
        h if h >= 240.0 && h < 300.0 => (x, 0.0, c),
        h if h >= 300.0 && h <= 360.0 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    let r: u8 = ((r + m) * 255.0).round() as u8;
    let g: u8 = ((g + m) * 255.0).round() as u8;
    let b: u8 = ((b + m) * 255.0).round() as u8;

    (r, g, b)
}

pub fn reset_all() -> &'static str {
    "\x1b[0m"
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
        let result = format!("{}{}{}", $color_code, format!($($arg)*), $crate::reset_all());
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
/// println!("{}", rgb!(255, 0, 0, "This is {} text", "bright red"));
/// ```
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr, $($arg:tt)*) => {{
        let color_code = format!("\x1b[38;2;{};{};{}m", $r, $g, $b);
        $crate::apply_color!(&color_code, $($arg)*)
    }};
}

#[macro_export]
/// Applies HSL color to the provided format string.
///
/// # Arguments
///
/// * `h` - The hue (0-360).
/// * `s` - The saturation (0-1).
/// * `l` - The lightness (0-1).
/// * `args` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", hsl!(120.0, 1.0, 0.5, "This is green text"));
/// ```
macro_rules! hsl {
    ($h:expr, $s:expr, $l:expr, $($arg:tt)*) => {{
        let (r, g, b) = $crate::hsl_to_rgb($h, $s, $l);
        $crate::rgb!(r, g, b, $($arg)*)
    }};
}

#[macro_export]
/// Applies HSV color to the provided format string.
///
/// # Arguments
///
/// * `h` - The hue (0-360).
/// * `s` - The saturation (0-1).
/// * `v` - The value (0-1).
/// * `args` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", hsv!(240.0, 1.0, 1.0, "This is blue text"));
/// ```
macro_rules! hsv {
    ($h:expr, $s:expr, $v:expr, $($arg:tt)*) => {{
        let (r, g, b) = $crate::hsv_to_rgb($h, $s, $v);
        $crate::rgb!(r, g, b, $($arg)*)
    }};
}
/// Applies a red background color to the provided format string.
///
/// # Arguments
///
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", bg_red!("This has a red background"));
/// ```
#[macro_export]
macro_rules! bg_red {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[41m", $($arg)*)
    }};
}

/// Applies a green background color to the provided format string.
///
/// # Arguments
///
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", bg_green!("This has a green background"));
/// ```
#[macro_export]
macro_rules! bg_green {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[42m", $($arg)*)
    }};
}

/// Applies a blue background color to the provided format string.
///
/// # Arguments
///
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", bg_blue!("This has a blue background"));
/// ```
#[macro_export]
macro_rules! bg_blue {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[44m", $($arg)*)
    }};
}

/// Applies a white background color to the provided format string.
///
/// # Arguments
///
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", bg_white!("This has a white background"));
/// ```
#[macro_export]
macro_rules! bg_white {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[47m", $($arg)*)
    }};
}

/// Applies a black background color to the provided format string.
///
/// # Arguments
///
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", bg_black!("This has a black background"));
/// ```
#[macro_export]
macro_rules! bg_black {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[40m", $($arg)*)
    }};
}

/// Applies a yellow background color to the provided format string.
///
/// # Arguments
///
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", bg_yellow!("This has a yellow background"));
/// ```
#[macro_export]
macro_rules! bg_yellow {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[43m", $($arg)*)
    }};
}

/// Applies a magenta background color to the provided format string.
///
/// # Arguments
///
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", bg_magenta!("This has a magenta background"));
/// ```
#[macro_export]
macro_rules! bg_magenta {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[45m", $($arg)*)
    }};
}

/// Applies a cyan background color to the provided format string.
///
/// # Arguments
///
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", bg_cyan!("This has a cyan background"));
/// ```
#[macro_export]
macro_rules! bg_cyan {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[46m", $($arg)*)
    }};
}

/// Applies bold formatting to the provided format string.
///
/// # Arguments
///
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", bold!("This text is bold"));
/// ```
#[macro_export]
macro_rules! bold {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[1m", $($arg)*)
    }};
}

/// Applies italic formatting to the provided format string.
///
/// # Arguments
///
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", italic!("This text is italic"));
/// ```
#[macro_export]
macro_rules! italic {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[3m", $($arg)*)
    }};
}

/// Applies underline formatting to the provided format string.
///
/// # Arguments
///
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", underline!("This text is underlined"));
/// ```
#[macro_export]
macro_rules! underline {
    ($($arg:tt)*) => {{
        $crate::apply_color!("\x1b[4m", $($arg)*)
    }};
}

/// Applies a custom RGB background color to the provided format string.
///
/// # Arguments
///
/// * `$r` - The red component (0-255).
/// * `$g` - The green component (0-255).
/// * `$b` - The blue component (0-255).
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", bg_rgb!(100, 150, 200, "This has a custom RGB background"));
/// ```
#[macro_export]
macro_rules! bg_rgb {
    ($r:expr, $g:expr, $b:expr, $($arg:tt)*) => {{
        let color_code = format!("\x1b[48;2;{};{};{}m", $r, $g, $b);
        $crate::apply_color!(&color_code, $($arg)*)
    }};
}

/// Applies a background color specified in HSL color space to the provided format string.
///
/// # Arguments
///
/// * `$h` - The hue component (0-360).
/// * `$s` - The saturation component (0-1).
/// * `$l` - The lightness component (0-1).
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", bg_hsl!(180.0, 0.5, 0.5, "This has an HSL-specified background"));
/// ```
#[macro_export]
macro_rules! bg_hsl {
    ($h:expr, $s:expr, $l:expr, $($arg:tt)*) => {{
        let (r, g, b) = $crate::hsl_to_rgb($h, $s, $l);
        $crate::bg_rgb!(r, g, b, $($arg)*)
    }};
}

/// Applies a background color specified in HSV color space to the provided format string.
///
/// # Arguments
///
/// * `$h` - The hue component (0-360).
/// * `$s` - The saturation component (0-1).
/// * `$v` - The value component (0-1).
/// * `$arg` - The format string and its arguments.
///
/// # Example
///
/// ```
/// use term_ansi::*;
///
/// println!("{}", bg_hsv!(270.0, 0.7, 0.9, "This has an HSV-specified background"));
/// ```
#[macro_export]
macro_rules! bg_hsv {
    ($h:expr, $s:expr, $v:expr, $($arg:tt)*) => {{
        let (r, g, b) = $crate::hsv_to_rgb($h, $s, $v);
        $crate::bg_rgb!(r, g, b, $($arg)*)
    }};
}
