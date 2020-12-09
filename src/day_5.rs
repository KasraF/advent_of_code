use crate::utils::read;
use crate::Error;

enum Dir {
    High,
    Low,
}

impl Dir {
    fn from(s: &str) -> Self {
        match s {
            "B" | "R" => Dir::High,
            "F" | "L" => Dir::Low,
            _ => unreachable!(),
        }
    }
}

fn binary_space_partition(mut start: u8, mut stop: u8, part: impl Iterator<Item = Dir>) -> u8 {
    for d in part {
        match d {
            Dir::High => start = ((start as f32 + stop as f32) / 2.0).ceil() as u8,
            Dir::Low => stop = ((start as f32 + stop as f32) / 2.0).floor() as u8,
        }
    }

    stop
}

struct Pass {
    row: u8,
    col: u8,
    id: u16,
}

impl Pass {
    fn from(s: String) -> Self {
        let row = binary_space_partition(0, 127, (0usize..7).map(|i| &s[i..i + 1]).map(Dir::from));
        let col = binary_space_partition(0, 7, (7usize..10).map(|i| &s[i..i + 1]).map(Dir::from));
        let id = row as u16 * 8 + col as u16;
        Self { row, col, id }
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        self.id
    }
}

pub fn solve() -> Result<(), Error> {
    let mut data = read("resources/input_5.txt")?
        .map(Pass::from)
        .collect::<Vec<Pass>>();
    data.sort_by_key(Pass::id);

    let mut last = data[0].id;

    for pass in &data[1..] {
        if pass.id != last + 1 {
            last += 1;
            break;
        } else {
            last = pass.id;
        }
    }

    println!("Day 5, Part 1: {}", data.last().unwrap().id);
    println!("Day 5, Part 2: {}", last);

    Ok(())
}
