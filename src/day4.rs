use crate::utils::{readfile, Error};

struct Board {
    nums: [u16; 25],
    marks: [bool; 25],
    won: bool,
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        let won = false;
        let marks = [false; 25];
        let mut nums = [0; 25];

        for (row, numbers) in input
            .split('\n')
            .map(|row| {
                row.split_ascii_whitespace()
                    .map(|num| num.parse::<u16>().unwrap())
            })
            .enumerate()
        {
            for (col, num) in numbers.enumerate() {
                nums[Self::index(row, col)] = num;
            }
        }

        Board { nums, marks, won }
    }
}

impl Board {
    #[inline]
    fn index(row: usize, col: usize) -> usize {
        row * 5 + col
    }

    /// Marks the number (if it exists on the board), and
    /// returns `true` if the board has one the game.
    fn mark(&mut self, n: u16) -> bool {
        for row in 0..5 {
            for col in 0..5 {
                let index = Self::index(row, col);
                if self.nums[index] == n {
                    self.marks[index] = true;
                }
            }
        }

        for i in 0..5 {
            let mut won = true;

            for col in 0..5 {
                if !self.marks[Self::index(i, col)] {
                    won = false;
                    break;
                }
            }

            if won {
                self.won = true;
                return true;
            }

            won = true;
            for row in 0..5 {
                if !self.marks[Self::index(row, i)] {
                    won = false;
                    break;
                }
            }

            if won {
                self.won = true;
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> usize {
        self.nums
            .iter()
            .zip(self.marks)
            .filter(|(_, marked)| !marked)
            .map(|(num, _)| *num as usize)
            .sum()
    }
}

pub fn main(file: Option<String>) -> Result<(String, String), Error> {
    let input = readfile(&file.unwrap_or("resources/day4.txt".to_string()))?;
    let input: Vec<_> = input.split("\n\n").collect();
    let (nums, boards) = input.split_first().unwrap();

    let nums: Vec<_> = nums.split(',').map(|n| n.parse::<u16>().unwrap()).collect();
    let mut boards: Vec<_> = boards.iter().map(|board| Board::from(*board)).collect();

    let mut first: Option<usize> = None;
    let mut last: Option<usize> = None;

    for num in nums {
        for board in boards.iter_mut().filter(|b| !b.won) {
            if board.mark(num) {
                // Found a winner!
                let score = board.sum_unmarked() * num as usize;
                last = Some(score);

                // Found a winner!
                if first.is_none() {
                    first = Some(score);
                }
            }
        }
    }

    // Part 1: 39421 is too high
    Ok((first.unwrap().to_string(), last.unwrap().to_string()))
}
