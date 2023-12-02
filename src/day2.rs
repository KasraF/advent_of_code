use crate::utils;

struct Game {
    id: u32,
    rounds: Vec<(u32, u32, u32)>,
}

impl Game {
    fn min_cubes(&self) -> (u32, u32, u32) {
        self.rounds
            .iter()
            .fold((0, 0, 0), |(acc_r, acc_g, acc_b), (r, g, b)| {
                (
                    u32::max(acc_r, *r),
                    u32::max(acc_g, *g),
                    u32::max(acc_b, *b),
                )
            })
    }
}

impl From<&str> for Game {
    fn from(input: &str) -> Self {
        let input = input.strip_prefix("Game ").expect("Invalid line");
        let (id, rounds) = input.split_once(':').expect("Invalid game format");

        let id = id.parse().expect("Invalid id format");
        let rounds = rounds
            .split(';')
            .map(|round| -> (u32, u32, u32) {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                for cube in round.split(',') {
                    match cube
                        .trim_start()
                        .split_once(' ')
                        .expect("Invalid cube format")
                    {
                        (n, "blue") => b = n.parse().expect("failed to parse blue"),
                        (n, "green") => g = n.parse().expect("failed to parse green"),
                        (n, "red") => r = n.parse().expect("failed to parse red"),
                        (n, color) => eprintln!("color format not supported: {n} {color}"),
                    }
                }

                (r, g, b)
            })
            .collect();

        Game { id, rounds }
    }
}

pub fn main() -> Result<(), utils::Error> {
    let rs: u32 = utils::read_lines(2)?
        .map(|line| -> Game { line.as_str().into() })
        .map(|game| {
            let (r, g, b) = game.min_cubes();
            r * g * b
        })
        .sum();
    println!("Day 2, Answer 2: {rs}");
    Ok(())
}
