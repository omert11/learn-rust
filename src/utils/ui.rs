use std::str::FromStr;

use inquire::{CustomType, DateSelect, Select};

pub fn get_input<T: FromStr + Clone + ToString>(message: &str) -> T {
    CustomType::<T>::new(message).prompt().unwrap()
}

pub fn get_date(message: &str) -> String {
    let date = DateSelect::new(message).prompt().unwrap();
    date.to_string()
}

pub fn select_operation(message: &str, operations: Vec<&str>) -> u8 {
    let selection = Select::new(message, operations.clone()).prompt().unwrap();
    operations
        .clone()
        .iter()
        .position(|&x| x == selection)
        .unwrap() as u8
}
