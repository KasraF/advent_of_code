use crate::utils::{readlines, Error};

/// Get the n-th binary digit of a number (0 or 1)
#[inline]
fn get_binary_digit(n: u8, number: u16) -> u16 {
    (number >> n) & 0b1
}

/// Set the n-th binary digit of a number (0 or 1)
#[inline]
fn set_binary_digit(n: u8, number: u16, digit: u16) -> u16 {
    let mask = 1 << n;
    (number & !mask) | (digit << n)
}

fn most_common_bit(bit: u8, nums: &[u16]) -> u16 {
    let count = nums
        .iter()
        .map(|num| get_binary_digit(bit, *num))
        .sum::<u16>() as usize;

    if count >= (nums.len() / 2) {
        1
    } else {
        0
    }
}

fn least_common_bit(bit: u8, nums: &[u16]) -> u16 {
    let count = nums
        .iter()
        .map(|num| 1 - get_binary_digit(bit, *num))
        .sum::<u16>() as usize;

    if count > (nums.len() / 2) {
        1
    } else {
        0
    }
}

fn most_common_bits(nums: &[u16]) -> u16 {
    let count = nums.len();
    let mut counts = [0u16; 12];

    // Count each digit in lines and store in counts
    for num in nums.iter() {
        for i in 0..12 {
            counts[i] += get_binary_digit(i as u8, *num);
        }
    }

    let mut rs = 0u16;

    // Set the n-th binary digit of gamma_rate to 1 if the n-th digit of counts is larger than count/2
    for i in 0..12 {
        if (counts[i] as usize) > count / 2 {
            rs = set_binary_digit(i as u8, rs, 1);
        }
    }

    rs
}

fn part2(nums: &[u16]) -> (usize, usize) {
    let mut o2 = Vec::from(nums);
    let mut co2 = Vec::from(nums);

    for bit in (0..12).rev() {
        if o2.len() > 1 {
            let bit_value = most_common_bit(bit, &o2);
            o2.retain(|num| get_binary_digit(bit, *num) == bit_value);
        }

        if co2.len() > 1 {
            let bit_value = least_common_bit(bit, &co2);
            co2.retain(|num| get_binary_digit(bit, *num) == bit_value);
        }

        if co2.len() < 2 && o2.len() < 2 {
            return (o2[0] as usize, co2[0] as usize);
        }
    }

    return (0, 0);
}

pub fn main(file: Option<String>) -> Result<(String, String), Error> {
    let lines = readlines(&file.unwrap_or("resources/day3.txt".to_string()))?
        .map(|l| l.unwrap())
        .map(|l| u16::from_str_radix(&l, 2).expect(&format!("Failed to parse line: {}", l)))
        .collect::<Vec<_>>();

    // Create variable epsilon_rate to flip the binary digits of gamma_rate
    let gamma_rate = most_common_bits(&lines);
    let epsilon_rate = (-(gamma_rate as i16) - 1) as u16 & 0b0000_1111_1111_1111;

    let (oxygen_generator_rating, co2_scrubber_rating) = part2(&lines);

    Ok((
        (gamma_rate as usize * epsilon_rate as usize).to_string(),
        (oxygen_generator_rating * co2_scrubber_rating).to_string(),
    ))
}
