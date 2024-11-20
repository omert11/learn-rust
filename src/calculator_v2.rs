use crate::ui;

fn _try_parse_float(value: &str) -> f64 {
    match value.parse::<f64>() {
        Ok(v) => v,
        Err(_) => 0.0,
    }
}

pub fn calculator() {
    println!("Calculator v2");
    println!("enter 'exit' to quit");
    println!("############");

    let mut result: f64 = 0.0;
    let mut last_value: String = String::new();
    loop {
        let value: String = ui::get_input("");

        if value == "exit" {
            break;
        }

        let mut prev_value: String = String::new();

        value.chars().for_each(|x| {
            if x == ' ' {
                return;
            }

            if x == '+' || x == '-' || x == '*' || x == '/' {
                if last_value.is_empty() {
                    last_value.push_str(&prev_value);
                    prev_value.clear();
                    return;
                }
                
                let v = _try_parse_float(&last_value);
                match x {
                    '+' => result += v,
                    '-' => result -= v,
                    '*' => result *= v,
                    '/' => result /= v,
                    _ => result = v,
                }
                last_value.clear();
            } else {
                prev_value.push(x);
            }
        });

        if !last_value.is_empty() {
            result = _try_parse_float(&last_value);
        }

        println!("Result: {}", result)
    }
}
