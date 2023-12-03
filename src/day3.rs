use crate::utils;

struct Schematic {
    grid: [[char; 142]; 142],
}

impl Schematic {
    fn parse_number(&self, mut i: usize, j: usize) -> Option<u32> {
        if !self.grid[j][i].is_ascii_digit() {
            None
        } else {
            // First, see if we need to move i to the left.
            while self.grid[j][i - 1].is_ascii_digit() {
                i -= 1;
            }

            // Then start reading the number.
            let mut num = self.grid[j][i].to_digit(10).unwrap();
            i += 1;
            while self.grid[j][i].is_ascii_digit() {
                num = num * 10 + self.grid[j][i].to_digit(10).unwrap();
                i += 1;
            }
            Some(num)
        }
    }
}

impl FromIterator<String> for Schematic {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut grid = [['.'; 142]; 142];
        let mut j = 0; // Skip the first row

        for line in iter {
            for (i, c) in line.chars().enumerate() {
                grid[j + 1][i + 1] = c;
            }
            j += 1;
        }

        Schematic { grid }
    }
}

fn part_one(schematic: &Schematic) {
    let mut sum = 0;
    for (j, line) in schematic.grid[1..141].iter().enumerate() {
        let j = j + 1;
        let mut i = 1;

        while i < line.len() - 1 {
            match line[i] {
                '0'..='9' => {
                    let start = i;
                    let mut num = line[i].to_digit(10).unwrap();
                    i += 1;
                    while i < line.len() && line[i].is_ascii_digit() {
                        num = num * 10 + line[i].to_digit(10).unwrap();
                        i += 1;
                    }

                    // start..end is the width of the number.
                    // We need to check that range above and
                    // below for any symbols.
                    let mut touches_symbol = line[start - 1] != '.' || line[i] != '.';

                    // above
                    for k in (start - 1)..=i {
                        if touches_symbol {
                            break;
                        }
                        let char = schematic.grid[j - 1][k];
                        touches_symbol = char != '.' && !char.is_ascii_digit();
                    }

                    // below
                    for k in (start - 1)..=i {
                        if touches_symbol {
                            break;
                        }
                        let char = schematic.grid[j + 1][k];
                        touches_symbol = char != '.' && !char.is_ascii_digit();
                    }

                    if touches_symbol {
                        sum += num;
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }
    }

    println!("Day 3, Answer 1: {sum}");
}

fn part_two(schematic: &Schematic) {
    let mut sum: u128 = 0;

    for (j, line) in schematic.grid[1..141].iter().enumerate() {
        let j = j + 1;

        for i in 1..(line.len() - 1) {
            match line[i] {
                '*' => {
                    // Found a gear. Time to look for numbers.
                    let right = schematic.parse_number(i + 1, j);
                    let left = schematic.parse_number(i - 1, j);
                    // Now above.
                    let (top, tl, tr) = if let Some(t) = schematic.parse_number(i, j - 1) {
                        // There is exactly one number above
                        (Some(t), None, None)
                    } else {
                        // Check left and right
                        let tl = schematic.parse_number(i - 1, j - 1);
                        let tr = schematic.parse_number(i + 1, j - 1);
                        (None, tl, tr)
                    };

                    // As above, so below.
                    let (bot, bl, br) = if let Some(t) = schematic.parse_number(i, j + 1) {
                        // There is exactly one number above
                        (Some(t), None, None)
                    } else {
                        // Check left and right
                        let tl = schematic.parse_number(i - 1, j + 1);
                        let tr = schematic.parse_number(i + 1, j + 1);
                        (None, tl, tr)
                    };

                    // Now, we need to check that there's *exactly* two:
                    let nums = [left, right, top, tl, tr, bot, bl, br];
                    let mut count = 0;
                    let mut mul = 1;
                    for n in nums {
                        if let Some(n) = n {
                            count += 1;
                            mul *= n;
                        }

                        if count > 2 {
                            break;
                        }
                    }

                    if count == 2 {
                        sum += mul as u128;
                    }
                }
                _ => (),
            }
        }
    }

    println!("Day 3, Answer 2: {sum}");
}

pub fn main() -> Result<(), utils::Error> {
    let schematic: Schematic = utils::read_lines(3)?.collect();
    part_one(&schematic);
    part_two(&schematic);

    Ok(())
}
