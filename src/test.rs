use crate::{blue, green, red, white};

#[test]
fn test_simple_color() {
    assert_eq!(red!("Hello"), "\x1b[31mHello\x1b[37m");
}

#[test]
fn test_nested_colors() {
    assert_eq!(
        white!("This is {} with {} color", red!("red"), green!("green")),
        "\x1b[37mThis is \x1b[31mred\x1b[37m with \x1b[32mgreen\x1b[37m color\x1b[37m"
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
        "\x1b[37mOuter \x1b[31mA\x1b[37m, Inner \x1b[32mB with \x1b[34mC\x1b[32m\x1b[37m"
    );
}

#[test]
fn test_multiple_colors_in_line() {
    assert_eq!(
            red!("Error: {}, {}", blue!("File not found"), green!("Please check your path")),
            "\x1b[31mError: \x1b[34mFile not found\x1b[31m, \x1b[32mPlease check your path\x1b[31m\x1b[37m"
        );
}

#[test]
fn test_color_reset() {
    assert_eq!(
        red!("Red text {} and reset", green!("Green text")),
        "\x1b[31mRed text \x1b[32mGreen text\x1b[31m and reset\x1b[37m"
    );
}
