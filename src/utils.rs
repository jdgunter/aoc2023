use std::fs;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_lines(file_path: &str) -> Vec<String> {
    let file = fs::File::open(file_path).expect("Unable to read file");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.expect("Could not parse line")).collect()
}