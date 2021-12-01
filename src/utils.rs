use std::fs::File;
use std::io::{BufRead, BufReader};

pub const UNIMPLEMENTED: &str = "unimplemented";

pub type Error = Box<dyn std::error::Error>;

pub fn readlines(path: String) -> Result<Vec<String>, Error> {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut rs = Vec::new();

    for line in lines.into_iter() {
        rs.push(line?)
    }

    Ok(rs)
}
