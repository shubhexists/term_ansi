use crate::*;

#[test]
fn test_red_macro() {
    red!("This is a red message");
}

#[test]
fn test_green_macro() {
    white!("{}, {}", red!("Hi"), green!("This is {} message", blue!("a nested")));
}

// #[test]
// fn test_black_macro() {
//     black!("This is a black message");
// }

// #[test]
// fn test_yellow_macro() {
//     yellow!("This is a yellow message");
// }

#[test]
fn test_blue_macro() {
    blue!("This is a blue message");
}

// #[test]
// fn test_magenta_macro() {
//     magenta!("This is a magenta message");
// }

// #[test]
// fn test_cyan_macro() {
//     cyan!("This is a cyan message");
// }

#[test]
fn test_white_macro() {
    white!("This is a white message");
}
