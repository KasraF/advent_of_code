use crate::Error;

use rayon::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Line {
    c: char,
    min: usize,
    max: usize,
    password: String,
}

fn read() -> Result<Vec<Line>, Error> {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$")?;
    let rs = BufReader::new(File::open("resources/input_2.txt")?)
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let cap = re.captures(&line).unwrap();
            Line {
                min: cap[1].parse().unwrap(),
                max: cap[2].parse().unwrap(),
                c: cap[3].chars().nth(0).unwrap(),
                password: cap[4].into(),
            }
        })
        .collect();

    Ok(rs)
}

pub fn solve() -> Result<(), Error> {
    let data = read()?;

    // Part 1: validate!
    let valid_count = data
        .par_iter()
        .filter(|line| {
            let count = line.password.chars().filter(|c| c == &line.c).count();
            count >= line.min && count <= line.max
        })
        .count();
    println!("Day 2, Part 1: {}", valid_count);

    let valid_count = data
        .par_iter()
        .filter(|line| {
            let fst = line.password.chars().nth(line.min - 1).unwrap() == line.c;
            let snd = line.password.chars().nth(line.max - 1).unwrap() == line.c;

            // Rust doesn't have logical XOR for some reason. :/
            (fst && !snd) || (!fst && snd)
        })
        .count();
    println!("Day 2, Part 2: {}", valid_count);

    Ok(())
}
