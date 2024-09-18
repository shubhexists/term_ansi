mod types;

#[cfg(test)]
mod test;

macro_rules! colored_println {
    ($color:expr, $($arg:tt)*) => {{
        print!("\x1B[{}m", $color);
        println!($($arg)*);
        print!("\x1B[0m");
    }};
}

#[macro_export]
macro_rules! red {
    ($($arg:tt)*) => {
        colored_println!(Colours::Red.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! green {
    ($($arg:tt)*) => {
        colored_println!(Colours::Green.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! black {
    ($($arg:tt)*) => {
        colored_println!(Colours::Black.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! yellow {
    ($($arg:tt)*) => {
        colored_println!(Colours::Yellow.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! blue {
    ($($arg:tt)*) => {
        colored_println!(Colours::Blue.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! magenta {
    ($($arg:tt)*) => {
        colored_println!(Colours::Magenta.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! cyan {
    ($($arg:tt)*) => {
        colored_println!(Colours::Cyan.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! white {
    ($($arg:tt)*) => {
        colored_println!(Colours::White.as_str(), $($arg)*);
    };
}
