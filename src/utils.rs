use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Error = Box<dyn std::error::Error>;

pub type Lines = Box<dyn Iterator<Item = String>>;

pub fn read_lines(day: u16) -> Result<Lines, Error> {
    let path = format!("inputs/day{day}.txt");
    let iter = BufReader::new(File::open(&path)?)
        .lines()
        .filter_map(|line| match line {
            Ok(l) => Some(l),
            Err(e) => {
                eprintln!("Failed to read line: {e}");
                None
            }
        });
    Ok(Box::new(iter))
}
