mod calculator_v1;
mod calculator_v2;
mod hello;
mod ui;

fn main() {
    let functions = [
        ("Hello", hello::hello as fn()),
        ("Calculator V1", calculator_v1::calculator as fn()),
        ("Calculator V2", calculator_v2::calculator as fn()),
    ];

    let input: u8 = ui::select_operation(
        "Select a function to run:",
        functions.iter().map(|(name, _)| *name).collect(),
    );
    if input > 0 && input <= functions.len() as u8 {
        functions[input as usize - 1].1();
    } else {
        println!("Invalid input");
    }
}
