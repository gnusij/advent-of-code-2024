use std::fs;
use std::path::Path;

pub fn read_file(day: u32, file_type: &str) -> String {
    let file_path = format!("src/days/d{:02}/{}", day, file_type);
    fs::read_to_string(Path::new(&file_path))
        .expect(&format!("Failed to read {}", file_path))
}

pub fn read_input(day: u32) -> String {
    read_file(day, "input")
}

#[cfg(test)]
pub fn read_example(day: u32) -> String {
    read_file(day, "example")
}

