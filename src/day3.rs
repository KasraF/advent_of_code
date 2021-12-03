use crate::utils::{readlines, Error};

/// Get the n-th binary digit of a number (0 or 1)
#[inline]
fn get_binary_digit(n: u8, number: u16) -> u16 {
    (number >> n) & 1
}

/// Set the n-th binary digit of a number (0 or 1)
#[inline]
fn set_binary_digit(n: u8, number: u16, digit: u16) -> u16 {
    let mask = 1 << n;
    (number & !mask) | (digit << n)
}

pub fn main(file: Option<String>) -> Result<(String, String), Error> {
    let lines = readlines(&file.unwrap_or("resources/day3.txt".to_string()))?
        .map(|l| l.unwrap())
        .map(|l| u16::from_str_radix(&l, 2).expect(&format!("Failed to parse line: {}", l)))
        .collect::<Vec<_>>();
    let count = lines.len();
    let mut counts = [0u16; 12];

    // Count each digit in lines and store in counts
    for line in &lines {
        for i in 0..12 {
            counts[i] += get_binary_digit(i as u8, *line);
        }
    }

    let mut gamma_rate = 0u16;

    // Set the n-th binary digit of gamma_rate to 1 if the n-th digit of counts is larger than count/2
    for i in 0..12 {
        if (counts[i] as usize) > count / 2 {
            gamma_rate = set_binary_digit(i as u8, gamma_rate, 1);
        }
    }

    // Create variable epsilon_rate to flip the binary digits of gamma_rate
    let epsilon_rate = (-(gamma_rate as i16) - 1) as u16 & 0b0000_1111_1111_1111;

    Ok((
        (gamma_rate as usize * epsilon_rate as usize).to_string(),
        "unimplemented".to_string(),
    ))
}
