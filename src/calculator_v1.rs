use crate::utils::ui;
fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}
fn divide(a: f64, b: f64) -> f64 {
    a / b
}
pub fn calculator() {
    let input = ui::ui_select(
        "Select an operation:",
        vec!["Addition", "Subtraction", "Multiplication", "Division"],
    );

    let a: f64 = ui::get_input("Enter the first number:");
    let b: f64 = ui::get_input("Enter the second number:");

    let result = match input {
        0 => add(a, b),
        1 => subtract(a, b),
        2 => multiply(a, b),
        3 => divide(a, b),
        _ => {
            println!("Invalid input");
            return;
        }
    };

    println!("Result: {}", result);
}
