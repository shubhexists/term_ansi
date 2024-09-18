// pub mod types;
// #[allow(unused_imports)]
// use crate::types::Colours;

// #[cfg(test)]
// mod test;

// #[allow(unused_imports)]
// use std::io::{self, Write};

// #[macro_export]
// macro_rules! red {
//     ($($arg:tt)*) => {{
//         let output = format!("\x1B[{}m{}\x1B[0m", Colours::Red.as_str(), format!($($arg)*));
//         let stdout = io::stdout();
//         let mut handle = stdout.lock();
//         writeln!(handle, "{}", output).unwrap();
//         handle.flush().unwrap();
//         format!($($arg)*)
//     }};
// }

// #[macro_export]
// macro_rules! green {
//     ($($arg:tt)*) => {{
//         let output = format!("\x1B[{}m{}\x1B[0m", Colours::Green.as_str(), format!($($arg)*));
//         let stdout = io::stdout();
//         let mut handle = stdout.lock();
//         writeln!(handle, "{}", output).unwrap();
//         handle.flush().unwrap();
//         format!($($arg)*)
//     }};
// }

// #[macro_export]
// macro_rules! blue {
//     ($($arg:tt)*) => {{
//         let output = format!("\x1B[{}m{}\x1B[0m", Colours::Blue.as_str(), format!($($arg)*));
//         let stdout = io::stdout();
//         let mut handle = stdout.lock();
//         writeln!(handle, "{}", output).unwrap();
//         handle.flush().unwrap();
//         format!($($arg)*)
//     }};
// }

// #[macro_export]
// macro_rules! white {
//     ($($arg:tt)*) => {{
//         let output = format!("\x1B[{}m{}\x1B[0m", Colours::White.as_str(), format!($($arg)*));
//         let stdout = io::stdout();
//         let mut handle = stdout.lock();
//         writeln!(handle, "{}", output).unwrap();
//         handle.flush().unwrap();
//         format!($($arg)*)
//     }};
// }
