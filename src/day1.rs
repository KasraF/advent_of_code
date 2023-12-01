use crate::Error;
use regex::Regex;
use smallvec::SmallVec;
use std::{fs::File, io::BufRead, io::BufReader};

fn parse_digit(d: &str) -> Result<u32, Error> {
    match d {
        "one" | "1" => Ok(1),
        "two" | "2" => Ok(2),
        "three" | "3" => Ok(3),
        "four" | "4" => Ok(4),
        "five" | "5" => Ok(5),
        "six" | "6" => Ok(6),
        "seven" | "7" => Ok(7),
        "eight" | "8" => Ok(8),
        "nine" | "9" => Ok(9),
        _ => Err(format!("Not a valid digit: {d}").into()),
    }
}

/// Parse a given line.
/// TODO Good excuse to learn nom here?
fn parse_digits(line: &str) -> u32 {
    let digit_rx =
        Regex::new(r"(\d)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)").unwrap();
    let matches: SmallVec<[&str; 8]> = digit_rx
        .captures_iter(line)
        .map(|x| x.iter().next().unwrap().unwrap().as_str())
        .collect();
    let first = parse_digit(matches[0]).unwrap();
    let last = parse_digit(matches[matches.len() - 1]).unwrap();
    println!("{line}: {matches:?} -> {}", first * 10 + last);
    first * 10 + last
}

pub fn main() -> Result<(), Error> {
    let rs: u32 = BufReader::new(File::open("inputs/day1.txt")?)
        .lines()
        .filter(|line| match line {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Failed to read line: {e}");
                false
            }
        })
        .map(|line| parse_digits(&line.unwrap()))
        .sum();

    println!("Day 1, Answer 1: {rs}");

    Ok(())
}
