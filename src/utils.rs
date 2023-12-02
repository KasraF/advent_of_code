use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub type Error = Box<dyn std::error::Error>;

pub fn read_lines(day: u16) -> Result<Lines<BufReader<File>>, Error> {
    let path = format!("inputs/day{day}.txt");
    Ok(BufReader::new(File::open(&path)?).lines())
}
