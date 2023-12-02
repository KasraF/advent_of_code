use crate::utils;

struct Game {
    id: u32,
    rounds: Vec<(u32, u32, u32)>,
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
        .filter_map(|game| {
            // filter to possible games
            if game
                .rounds
                .into_iter()
                .all(|(r, g, b)| r <= 12 && g <= 13 && b <= 14)
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum();
    println!("Day 2, Answer 1: {rs}");
    Ok(())
}
