use crate::ui;

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
    let input = ui::select_operation(
        "Select an operation:",
        vec!["Addition", "Subtraction", "Multiplication", "Division"],
    );

    let a: f64 = ui::get_input("Enter the first number:");
    let b: f64 = ui::get_input("Enter the second number:");

    let result = match input {
        1 => add(a, b),
        2 => subtract(a, b),
        3 => multiply(a, b),
        4 => divide(a, b),
        _ => {
            println!("Invalid input");
            return;
        }
    };

    println!("Result: {}", result);
}
