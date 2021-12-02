use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub type Error = Box<dyn std::error::Error>;

pub fn readlines(path: &str) -> Result<Lines<BufReader<File>>, Error> {
    Ok(BufReader::new(File::open(path)?).lines())
}
