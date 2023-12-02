use crate::utils::Error;

#[inline]
fn nom(s: &str, target: &str) -> bool {
    if s.len() < target.len() {
        return false;
    }

    &s[0..target.len()] == target
}

fn parse_digit(slice: &str) -> Option<u32> {
    let rs = match &slice[0..=0] {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "o" if nom(slice, "one") => 1,
        "t" if nom(slice, "two") => 2,
        "t" if nom(slice, "three") => 3,
        "f" if nom(slice, "four") => 4,
        "f" if nom(slice, "five") => 5,
        "s" if nom(slice, "six") => 6,
        "s" if nom(slice, "seven") => 7,
        "e" if nom(slice, "eight") => 8,
        "n" if nom(slice, "nine") => 9,
        _ => return None,
    };
    Some(rs)
}

fn parse_digits(line: &str) -> u32 {
    let l = line.len();
    let mut first = 0;
    for i in 0..l {
        if let Some(digit) = parse_digit(&line[i..]) {
            first = digit;
            break;
        }
    }

    let mut second = 0;
    for i in (0..l).rev() {
        if let Some(digit) = parse_digit(&line[i..]) {
            second = digit;
            break;
        }
    }

    first * 10 + second
}

pub fn main() -> Result<(), Error> {
    let rs: u32 = crate::utils::read_lines(1)?
        .map(|line| parse_digits(&line))
        .sum();

    println!("Day 1, Answer 1: {rs}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::parse_digits;

    #[test]
    fn example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let rs: u32 = input.lines().map(|l| parse_digits(l)).sum();
        assert_eq!(rs, 281);
    }

    #[test]
    fn test_parse_digits() {
        assert_eq!(parse_digits("two1nine"), 29);
        assert_eq!(parse_digits("eightwothree"), 83);
        assert_eq!(parse_digits("abcone2threexyz"), 13);
        assert_eq!(parse_digits("xtwone3four"), 24);
        assert_eq!(parse_digits("4nineeightseven2"), 42);
        assert_eq!(parse_digits("zoneight234"), 14);
        assert_eq!(parse_digits("7pqrstsixteen"), 76);
        assert_eq!(parse_digits("nineasd"), 99);

        assert_eq!(parse_digits("1abc2"), 12);
        assert_eq!(parse_digits("pqr3stu8vwx"), 38);
        assert_eq!(parse_digits("a1b2c3d4e5five"), 15);
        assert_eq!(parse_digits("treb7uchet"), 77);
    }
}
