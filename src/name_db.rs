use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use crate::utils::ui;

const DIR_PATH: &str = "db";
const FILE_PATH: &str = "db/name_db.txt";

#[derive(Debug)]
enum FileMode {
    Read,
    Append,
}
fn _get_or_create_file(mode: FileMode) -> File {
    if !Path::new(DIR_PATH).exists() {
        create_dir_all(DIR_PATH).unwrap();
    }
    if !Path::new(FILE_PATH).exists() {
        File::create(FILE_PATH).unwrap();
    }
    match mode {
        FileMode::Read => OpenOptions::new().read(true).open(FILE_PATH).unwrap(),
        FileMode::Append => OpenOptions::new()
            .write(true)
            .append(true)
            .open(FILE_PATH)
            .unwrap(),
    }
}

fn _search_lines(name: &str) -> bool {
    let mut reader = BufReader::new(_get_or_create_file(FileMode::Read));
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if line.trim().to_string() == name {
                    return true;
                }
            }
            Err(_) => break,
        }
    }
    false
}

pub fn name_db() {
    println!("Name db");
    println!("enter 'exit' to quit");
    println!("############");
    loop {
        let input: String = ui::get_input("Enter a name to add for: ");
        if input == "exit" {
            break;
        }

        if _search_lines(&input) {
            println!("Name already exists");
        } else {
            let mut file = _get_or_create_file(FileMode::Append);
            writeln!(file, "{}", input).unwrap();
            println!("Name added");
        }
    }
}
