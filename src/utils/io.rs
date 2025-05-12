use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
pub enum FileMode {
    Read,
    Append,
}

pub fn create_file_with_dirs<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    if let Ok(_) = File::create_new(&path) {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn read_to_string_or_default<P: AsRef<Path>>(path: P, default: &str) -> io::Result<String> {
    if let Ok(content) = fs::read_to_string(path) {
        Ok(content)
    } else {
        Ok(default.to_string())
    }
}

pub fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    create_file_with_dirs(&path)?;
    fs::write(&path, content).unwrap();
    Ok(())
}

pub fn open_file<P: AsRef<Path>>(path: P, mode: FileMode) -> io::Result<File> {
    create_file_with_dirs(&path)?;
    match mode {
        FileMode::Read => OpenOptions::new().read(true).open(path),
        FileMode::Append => OpenOptions::new().write(true).append(true).open(path),
    }
}

pub fn search_line_exact<P: AsRef<Path>>(path: P, search: &str) -> io::Result<bool> {
    let file = open_file(path, FileMode::Read)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if line.trim().to_string() == search {
                    return Ok(true);
                }
            }
            Err(_) => break,
        }
    }
    Ok(false)
}
