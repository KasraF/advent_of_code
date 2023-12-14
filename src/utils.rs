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

pub struct LoopingIter<'a, T> {
    inner: &'a [T],
    idx: usize,
}

impl<'a, T> Iterator for LoopingIter<'a, T> {
    type Item = &'a T;

    fn next<'s>(&'s mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }

        let rs = &self.inner[self.idx];

        self.idx += 1;
        if self.idx >= self.inner.len() {
            self.idx = 0;
        }

        Some(rs)
    }
}

pub trait LoopingIterable<'a, T> {
    fn looping_iter(&'a self) -> LoopingIter<'a, T>;
}

impl<'a, C: 'a + ?Sized, T: 'a> LoopingIterable<'a, T> for C
where
    &'a C: Into<&'a [T]>,
{
    fn looping_iter(&'a self) -> LoopingIter<'a, T> {
        LoopingIter {
            inner: self.into(),
            idx: 0,
        }
    }
}
