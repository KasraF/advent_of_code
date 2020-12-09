use crate::Error;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
struct Passport<'a> {
    birth_year: Option<&'a str>,
    issue_year: Option<&'a str>,
    expiration_year: Option<&'a str>,
    height: Option<&'a str>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn from(s: &'a str) -> Self {
        let mut passport = Self::default();
        s.split_ascii_whitespace()
            .for_each(|entry| match &entry[..3] {
                "byr" => passport.birth_year = Some(&entry[4..]),
                "iyr" => passport.issue_year = Some(&entry[4..]),
                "eyr" => passport.expiration_year = Some(&entry[4..]),
                "hgt" => passport.height = Some(&entry[4..]),
                "hcl" => passport.hair_color = Some(&entry[4..]),
                "ecl" => passport.eye_color = Some(&entry[4..]),
                "pid" => passport.passport_id = Some(&entry[4..]),
                "cid" => passport.country_id = Some(&entry[4..]),
                _ => unreachable!(),
            });
        passport
    }

    fn is_present(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_valid(&self) -> bool {
        self.is_present()
            && {
                // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                let year = self.birth_year.unwrap();

                if year.len() != 4 {
                    return false;
                }

                match year.parse::<u32>() {
                    Ok(year) => year >= 1920 && year <= 2002,
                    _ => false,
                }
            }
            && {
                // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                let year = self.issue_year.unwrap();

                if year.len() != 4 {
                    return false;
                }

                match year.parse::<u32>() {
                    Ok(year) => year >= 2010 && year <= 2020,
                    _ => false,
                }
            }
            && {
                // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                let year = self.expiration_year.unwrap();

                if year.len() != 4 {
                    return false;
                }

                match year.parse::<u32>() {
                    Ok(year) => year >= 2020 && year <= 2030,
                    _ => false,
                }
            }
            && {
                // hgt (Height) - a number followed by either cm or in:
                let height = self.height.unwrap();
                let len = height.len();

                if len < 3 {
                    return false;
                }

                match &height[len - 2..len] {
                    "cm" => {
                        match height[..len - 2].parse::<u32>() {
                            // If cm, the number must be at least 150 and at most 193.
                            Ok(height) => height >= 150 && height <= 193,
                            _ => false,
                        }
                    }
                    "in" => {
                        match height[..len - 2].parse::<u32>() {
                            // If in, the number must be at least 59 and at most 76.
                            Ok(height) => height >= 59 && height <= 76,
                            _ => false,
                        }
                    }
                    _ => false,
                }
            }
            && {
                // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                let hair_color = self.hair_color.unwrap();
                &hair_color[0..1] == "#" && hair_color[1..].chars().all(|c| c.is_digit(16))
            }
            && {
                // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                match self.eye_color {
                    Some("amb") => true,
                    Some("blu") => true,
                    Some("brn") => true,
                    Some("gry") => true,
                    Some("grn") => true,
                    Some("hzl") => true,
                    Some("oth") => true,
                    _ => false,
                }
            }
            && {
                // pid (Passport ID) - a nine-digit number, including leading zeroes.
                let passport_id = self.passport_id.unwrap();
                passport_id.len() == 9 && passport_id.parse::<u32>().is_ok()
            }
    }
}

pub fn solve() -> Result<(), Error> {
    // I could do this with nom, but
    // it's quicker to roll my own,
    // given the special case. :/

    let mut present = 0;
    let mut valid = 0;
    let mut reader = BufReader::new(File::open("resources/input_4.txt")?);
    let mut buffer = String::new();

    while let Ok(n) = reader.read_line(&mut buffer) {
        if buffer.ends_with("\n\n") || n == 0 {
            // We should have a passport!
			let passport = Passport::from(&buffer);
            if passport.is_present() {
                present += 1;
            }

            if passport.is_valid() {
                valid += 1;
            }
            buffer.clear();
        }

        if n == 0 {
            break;
        }
    }

    println!("Day 4, Part 1: {}", present);
    println!("Day 4, Part 2: {}", valid);

    Ok(())
}

