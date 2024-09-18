use std::cell::RefCell;

thread_local! {
    static COLOR_CONTEXT: RefCell<Vec<String>> = RefCell::new(vec![String::from("\x1b[37m")]);
}

struct ColorContext;

impl ColorContext {
    fn push(color: &str) {
        COLOR_CONTEXT.with(|ctx| {
            ctx.borrow_mut().push(color.to_string());
        });
    }

    fn pop() {
        COLOR_CONTEXT.with(|ctx| {
            ctx.borrow_mut().pop();
        });
    }

    fn current_color() -> String {
        COLOR_CONTEXT.with(|ctx| {
            ctx.borrow()
                .last()
                .cloned()
                .unwrap_or_else(|| String::from("\x1b[37m")) // Default to white if stack is empty
        })
    }
}

macro_rules! apply_color {
    ($color_code:expr, $($arg:tt)*) => {{
        ColorContext::push($color_code);
        let result = format!("{}{}", $color_code, format!($($arg)*));
        ColorContext::pop();
        format!("{}{}", result, ColorContext::current_color())
    }};
}

macro_rules! red {
    ($($arg:tt)*) => {{
        apply_color!("\x1b[31m", $($arg)*)
    }};
}

macro_rules! green {
    ($($arg:tt)*) => {{
        apply_color!("\x1b[32m", $($arg)*)
    }};
}

macro_rules! blue {
    ($($arg:tt)*) => {{
        apply_color!("\x1b[34m", $($arg)*)
    }};
}

macro_rules! white {
    ($($arg:tt)*) => {{
        apply_color!("\x1b[37m", $($arg)*)
    }};
}

fn main() {
    // Example 1: Simple Color Application
    println!("{}", red!("Hello World!"));

    // Example 2: Multiple Colors in a Single Line
    println!(
        "{}",
        red!(
            "Error: {}, {}",
            blue!("File not found"),
            green!("Please check your path")
        )
    );

    // Example 3: Nested Colors
    println!(
        "{}",
        white!("This is {} with {} color", red!("red"), green!("green"))
    );

    // Example 4: Deeply Nested Colors
    println!(
        "{}",
        white!("Outer {}, Inner {}", red!("Error"), blue!("Details"))
    );

    // Example 5: Color with Formatting
    println!("{}", green!("The number is: {}", 42));

    // Example 6: Nested Colors with Multiple Levels
    println!(
        "{}",
        white!(
            "Start {}, Middle {}, End {}",
            red!("A"),
            green!("B"),
            blue!("C")
        )
    );

    // Example 7: Simple Color Reset
    println!("{}", red!("Red text {} and reset", green!("Green text")));

    // Example 8: Multiple Nested Colors
    println!(
        "{}",
        blue!("Blue {} with {} colors", red!("red"), green!("green"))
    );

    // Example 9: Color Formatting with Nested Values
    println!(
        "{}",
        white!("This is a {} {} example", blue!("blue"), red!("red"))
    );

    // Example 10: Long Nested Colors
    println!(
        "{}",
        white!(
            "Level 1 {}, Level 2 {}, Level 3 {}",
            red!("1"),
            green!("2"),
            blue!("3")
        )
    );

    // Example 11: Complex Nested Colors
    println!(
        "{}",
        white!(
            "Outer {}, Inner {}",
            red!("A"),
            green!("B with {}", blue!("C"))
        )
    );

    // Example 12: Single Line with Multiple Colors
    println!(
        "{}",
        red!(
            "Error: {} at {} {}",
            blue!("File"),
            green!("line"),
            white!("number")
        )
    );

    // Example 13: Nested Color with Special Characters
    println!(
        "{}",
        white!(
            "Special {} characters {}",
            red!("!@#$%^&*()"),
            blue!("~`|[]")
        )
    );

    // Example 14: Color with Mixed Formatting
    println!(
        "{}",
        green!("Text: {}, Bold: {}", red!("red"), blue!("blue"))
    );

    // Example 15: Color in a Multi-line String
    println!(
        "{}",
        white!(
            "Line 1: {}\nLine 2: {}\nLine 3: {}",
            red!("Error"),
            green!("Warning"),
            blue!("Info")
        )
    );

    // Example 16: Simple Nested and Flat Colors
    println!(
        "{}",
        white!("Flat {} and nested {}", red!("error"), green!("success"))
    );

    // Example 17: Deeply Nested with Color Reset
    println!(
        "{}",
        white!(
            "Level 1 {}, Level 2 {}",
            red!("1"),
            green!("2 {}", blue!("3"))
        )
    );

    // Example 18: Mixed Nested and Sequential Colors
    println!(
        "{}",
        white!("Start {} {} {}", red!("A"), green!("B"), blue!("C"))
    );

    // Example 19: Color Formatting with Variables
    let error_message = red!("An error occurred: {}", "File not found");
    println!("{}", white!("Status: {}", error_message));

    // Example 20: Comprehensive Example
    println!(
        "{}",
        white!(
            "Summary: {} \nDetails: {} \nInfo: {}",
            red!("Critical error: {}", "File not found"),
            green!("Suggestion: {}", "Check the file path"),
            blue!("Additional info: {}", "System log")
        )
    );

    println!("{}, {}", red!("hi"), blue!("bye"))
}
