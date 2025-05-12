use std::io::prelude::*;

use crate::utils::io::{open_file, search_line_exact, FileMode};
use crate::utils::ui;

const FILE_PATH: &str = "db/name_db.txt";

pub fn name_db() {
    println!("Name db");
    println!("enter 'exit' to quit");
    println!("############");
    loop {
        let input: String = ui::get_input("Enter a name to add for: ");
        if input == "exit" {
            break;
        }

        if search_line_exact(FILE_PATH, &input).unwrap() {
            println!("Name already exists");
        } else {
            let mut file = open_file(FILE_PATH, FileMode::Append).unwrap();
            writeln!(file, "{}", input).unwrap();
            println!("Name added");
        }
    }
}
