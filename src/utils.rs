use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};
use std::os::unix::prelude::MetadataExt;

pub type Error = Box<dyn std::error::Error>;

pub fn readlines(path: &str) -> Result<Lines<BufReader<File>>, Error> {
    Ok(BufReader::new(File::open(path)?).lines())
}

pub fn readfile(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut rs = String::with_capacity(file.metadata()?.size() as usize);
    file.read_to_string(&mut rs)?;
    Ok(rs)
}
