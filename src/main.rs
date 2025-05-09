mod calculator_v1;
mod calculator_v2;
mod calculator_v3;
mod hello;
mod name_db;
mod ui;

fn main() {
    let functions = [
        ("Hello", hello::hello as fn()),
        ("Calculator V1", calculator_v1::calculator as fn()),
        ("Calculator V2", calculator_v2::calculator as fn()),
        ("Calculator V3", calculator_v3::main::calculator as fn()),
        ("Name DB", name_db::name_db as fn()),
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
