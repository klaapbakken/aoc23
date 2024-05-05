use std::{fs, io::{BufReader, BufRead}};
pub fn read_lines(file_path: &str) -> Vec<String> {
    let file = fs::File::open(file_path).expect("Couldn't read file.");
    let reader = BufReader::new(file);
    let lines = reader.lines()
            .map(|line| line.expect("Failed to read line"))
            .collect();
    return lines;
}