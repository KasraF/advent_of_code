use crate::utils::{readlines, Error};

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Default)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn move_part1(mut self, dir: &Direction) -> Self {
        match dir {
            Direction::Forward(amount) => self.horizontal += amount,
            Direction::Down(amount) => self.depth += amount,
            Direction::Up(amount) => self.depth -= amount,
        }

        self
    }

    fn move_part2(mut self, dir: &Direction) -> Self {
        match dir {
            Direction::Down(amount) => self.aim += amount,
            Direction::Up(amount) => self.aim -= amount,
            Direction::Forward(amount) => {
                self.horizontal += amount;
                self.depth += self.aim * amount;
            }
        }

        self
    }
}

fn parse(s: &str) -> Direction {
    match s.split_at(s.find(" ").unwrap()) {
        ("forward", val) => Direction::Forward(val.trim().parse().unwrap()),
        ("down", val) => Direction::Down(val.trim().parse().unwrap()),
        ("up", val) => Direction::Up(val.trim().parse().unwrap()),
        (_, _) => unreachable!(),
    }
}

pub fn main(file: Option<String>) -> Result<(String, String), Error> {
    let path = &file.unwrap_or("resources/day2.txt".to_string());
    let directions: Vec<Direction> = readlines(path)?.map(|l| parse(&l.unwrap())).collect();
    let position = directions
        .iter()
        .fold(Position::default(), |pos, dir| pos.move_part1(dir));
    let part1 = format!("{}", position.depth * position.horizontal);

    let position = directions
        .iter()
        .fold(Position::default(), |pos, dir| pos.move_part2(dir));
    let part2 = format!("{}", position.depth * position.horizontal);

    Ok((part1, part2))
}
