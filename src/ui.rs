use std::str::FromStr;

fn _get_input<T: FromStr>() -> T {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    {
        let this = input.trim().parse();
        match this {
            Ok(t) => t,
            Err(_e) => {
                println!("Error parsing input");
                _get_input()
            }
        }
    }
}

pub fn get_input<T: FromStr>(message: &str) -> T {
    println!("{}", message);
    _get_input()
}

pub fn select_operation(message: &str, operations: Vec<&str>) -> u8 {
    println!("{}", message);
    for (index, operation) in operations.iter().enumerate() {
        println!("{}. {}", index + 1, operation);
    }
    _get_input()
}
