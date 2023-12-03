use crate::utils;

struct Schematic {
    grid: [[char; 142]; 142],
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

pub fn main() -> Result<(), utils::Error> {
    let schematic: Schematic = utils::read_lines(3)?.collect();
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
                '.' => {
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
    }

    println!("Day 3, Answer 1: {sum}");

    Ok(())
}
