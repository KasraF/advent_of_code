use crate::utils::{readlines, Error};

fn count_increasing<T: PartialOrd>(v: &[T]) -> usize {
    v.iter()
        .zip(v[1..].iter())
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum()
}

pub fn main(file: Option<String>) -> Result<(String, String), Error> {
    let path = file.unwrap_or_else(|| "resources/day1.txt".to_string());
    let lines: Vec<i32> = readlines(&path)?
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    let part1: usize = count_increasing(&lines);

    let part2_windows: Vec<i32> = lines
        .iter()
        .zip(lines[1..].iter())
        .zip(lines[2..].iter())
        .map(|((a, b), c)| a + b + c)
        .collect();

    let part2: usize = count_increasing(&part2_windows);

    Ok((format!("{}", part1), format!("{}", part2)))
}
