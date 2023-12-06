use rayon::prelude::*;

use crate::utils;

fn ans1() -> Result<usize, utils::Error> {
    let mut games: [(u32, u32); 4] = [(0, 0); 4];

    for line in utils::read_lines(6)? {
        if let Some(rest) = line.strip_prefix("Time:") {
            for (i, num) in rest.split_whitespace().enumerate() {
                games[i].0 = num.parse()?;
            }
        } else if let Some(rest) = line.strip_prefix("Distance:") {
            for (i, num) in rest.split_whitespace().enumerate() {
                games[i].1 = num.parse()?;
            }
        } else {
            return Err(format!("Line format not recognized: {line}").into());
        }
    }

    // TODO There has to be a O(1) solution, right?

    Ok(games
        .par_iter()
        .map(|(time, dist)| {
            (0..=*time)
                .into_par_iter()
                .filter(|t| {
                    let velocity = t;
                    let new_dist = velocity * (time - t);
                    new_dist > *dist
                })
                .count()
        })
        .reduce(|| 1, |a, b| a * b))
}

fn ans2() -> Result<usize, utils::Error> {
    let mut time = 0 as u64;
    let mut dist = 0 as u64;

    for line in utils::read_lines(6)? {
        if let Some(rest) = line.strip_prefix("Time:") {
            time = rest
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_digit(10).unwrap())
                .fold(0, |acc, c| acc * 10 + c as u64);
        } else if let Some(rest) = line.strip_prefix("Distance:") {
            dist = rest
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_digit(10).unwrap())
                .fold(0, |acc, c| acc * 10 + c as u64);
        } else {
            return Err(format!("Line format not recognized: {line}").into());
        }
    }

    // TODO There has to be a O(1) solution, right?

    Ok((0..=time)
        .into_par_iter()
        .filter(|t| {
            let velocity = t;
            let new_dist = velocity * (time - t);
            new_dist > dist
        })
        .count())
}

pub fn main() -> Result<(), utils::Error> {
    println!("Day 6, Answer 1: {}", ans1()?);
    println!("Day 6, Answer 2: {}", ans2()?);
    Ok(())
}
