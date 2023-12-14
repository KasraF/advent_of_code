use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Error = Box<dyn std::error::Error>;

pub type ACResult = Result<(), Error>;

pub type Lines = Box<dyn Iterator<Item = String>>;

pub fn read_lines(day: u16) -> Result<Lines, Error> {
    let path = format!("inputs/day{day}.txt");
    let iter = BufReader::new(File::open(&path)?)
        .lines()
        .filter_map(|line| match line {
            Ok(l) => {
                let trimmed = l.trim();
                if trimmed.is_empty() {
                    None
                } else if l.len() == trimmed.len() {
                    Some(l)
                } else {
                    Some(trimmed.to_string())
                }
            }
            Err(e) => {
                eprintln!("Failed to read line: {e}");
                None
            }
        });
    Ok(Box::new(iter))
}
