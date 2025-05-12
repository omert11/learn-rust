use crate::utils::ui;
enum Operator {
    Eqq,
    Add,
    Sub,
    Mul,
    Div,
}

fn _try_parse_float(value: &str) -> f64 {
    match value.parse::<f64>() {
        Ok(v) => v,
        Err(_) => 0.0,
    }
}

fn _do_operation(operator: Operator, current: f64, value: f64) -> f64 {
    match operator {
        Operator::Eqq => value,
        Operator::Add => current + value,
        Operator::Sub => current - value,
        Operator::Mul => current * value,
        Operator::Div => current / value,
    }
}

fn _is_operator(value: char) -> bool {
    value == '+' || value == '-' || value == '*' || value == '/'
}

fn _get_operator(value: char) -> Operator {
    match value {
        '+' => Operator::Add,
        '-' => Operator::Sub,
        '*' => Operator::Mul,
        '/' => Operator::Div,
        _ => Operator::Eqq,
    }
}
pub fn calculator() {
    println!("Calculator v2");
    println!("enter 'exit' to quit");
    println!("############");

    let mut operator: Operator = Operator::Eqq;
    let mut result: f64 = 0.0;

    loop {
        let input: String = ui::get_input("");

        if input == "exit" {
            break;
        }

        let mut value: String = String::new();

        for x in input.chars() {
            if x == ' ' {
                continue;
            }

            if _is_operator(x) {
                let parsed_value = _try_parse_float(&value);
                result = _do_operation(operator, result, parsed_value);
                operator = _get_operator(x);
                value.clear();
                continue;
            }

            value.push(x);
        }
        if !value.is_empty() {
            let parsed_value = _try_parse_float(&value);
            result = _do_operation(operator, result, parsed_value);
        }
        operator = Operator::Eqq;
        println!("Result: {}", result)
    }
}
