use crate::{
    bg_green, bg_hsl, bg_hsv, black, blue, bold, cyan, green, hsl, magenta, red, rgb, white, yellow,
};

#[test]
fn test_simple_color() {
    assert_eq!(red!("Hello"), "\x1b[31mHello\x1b[0m\u{1b}[37m");
}

#[test]
fn test_nested_colors() {
    assert_eq!(
        white!("This is {} with {} color", red!("red"), green!("green")),
        "\x1b[37mThis is \x1b[31mred\x1b[0m\x1b[37m with \x1b[32mgreen\x1b[0m\x1b[37m color\x1b[0m\u{1b}[37m"
    );
}

#[test]
fn test_complex_nesting() {
    assert_eq!(
        white!(
            "Outer {}, Inner {}",
            red!("A"),
            green!("B with {}", blue!("C"))
        ),
        "\x1b[37mOuter \x1b[31mA\x1b[0m\x1b[37m, Inner \x1b[32mB with \x1b[34mC\x1b[0m\x1b[32m\x1b[0m\x1b[37m\x1b[0m\u{1b}[37m"
    );
}

#[test]
fn test_multiple_colors_in_line() {
    assert_eq!(
        red!("Error: {}, {}", blue!("File not found"), green!("Please check your path")),
        "\x1b[31mError: \x1b[34mFile not found\x1b[0m\x1b[31m, \x1b[32mPlease check your path\x1b[0m\x1b[31m\x1b[0m\u{1b}[37m"
    );
}

#[test]
fn test_color_reset() {
    assert_eq!(
        red!("Red text {} and reset", green!("Green text")),
        "\x1b[31mRed text \x1b[32mGreen text\x1b[0m\x1b[31m and reset\x1b[0m\u{1b}[37m"
    );
}

#[test]
fn test_black_color() {
    assert_eq!(black!("Black text"), "\x1b[30mBlack text\x1b[0m\u{1b}[37m");
}

#[test]
fn test_yellow_color() {
    assert_eq!(yellow!("Yellow text"), "\x1b[33mYellow text\x1b[0m\u{1b}[37m");
}

#[test]
fn test_magenta_color() {
    assert_eq!(magenta!("Magenta text"), "\x1b[35mMagenta text\x1b[0m\u{1b}[37m");
}

#[test]
fn test_cyan_color() {
    assert_eq!(cyan!("Cyan text"), "\x1b[36mCyan text\x1b[0m\u{1b}[37m");
}

#[test]
fn test_rgb_color() {
    assert_eq!(
        rgb!(255, 0, 0, "Red RGB"),
        "\x1b[38;2;255;0;0mRed RGB\x1b[0m\u{1b}[37m"
    );
    assert_eq!(
        rgb!(0, 255, 0, "Green RGB"),
        "\x1b[38;2;0;255;0mGreen RGB\x1b[0m\u{1b}[37m"
    );
    assert_eq!(
        rgb!(0, 0, 255, "Blue RGB"),
        "\x1b[38;2;0;0;255mBlue RGB\x1b[0m\u{1b}[37m"
    );
}

#[test]
fn test_bg_hsl() {
    assert_eq!(
        bg_hsl!(120.0, 1.0, 0.5, "HSL Green background"),
        "\x1b[48;2;0;255;0mHSL Green background\x1b[0m\u{1b}[37m"
    );
}

#[test]
fn test_bg_hsv() {
    assert_eq!(
        bg_hsv!(240.0, 1.0, 1.0, "HSV Blue background"),
        "\x1b[48;2;0;0;255mHSV Blue background\x1b[0m\u{1b}[37m"
    );
}

#[test]
fn test_nested_formatting() {
    assert_eq!(
        red!("{}", bg_green!("{}", bold!("Red text on green background"))),
        "\x1b[31m\x1b[42m\x1b[1mRed text on green background\x1b[0m\x1b[42m\x1b[0m\x1b[31m\x1b[0m\u{1b}[37m"
    );
}

#[test]
fn test_nested_hsl_hsv() {
    assert_eq!(
        hsl!(0.0, 1.0, 0.5, "{}", bg_hsv!(120.0, 1.0, 1.0, "Red text on green background")),
        "\x1b[38;2;255;0;0m\x1b[48;2;0;255;0mRed text on green background\x1b[0m\x1b[38;2;255;0;0m\x1b[0m\u{1b}[37m"
    );
}
