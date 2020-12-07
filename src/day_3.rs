use crate::utils::AdventError;
use crate::Error;

enum Tile {
    Square,
    Tree,
}

struct Map {
    width: usize,
    height: usize,
    data: Vec<Tile>,
}

impl Map {
    fn from(data: Vec<Tile>, width: usize, height: usize) -> Result<Self, AdventError> {
        if width * height != data.len() {
            Err(AdventError::new(&format!(
                "Expected vec of size {}x{}={}, found vec of size {}.",
                width,
                height,
                width * height,
                data.len()
            )))
        } else {
            Ok(Map {
                width,
                height,
                data,
            })
        }
    }

    fn trees(&self, slope: &(usize, usize)) -> usize {
        let mut idx: (usize, usize) = (0, 0);
        let mut trees = 0;

        while idx.1 < self.height {
            match &self[idx] {
                Tile::Tree => trees += 1,
                _ => (),
            }
            idx.0 += slope.0;
            idx.1 += slope.1;
        }

        trees
    }
}

impl std::ops::Index<(usize, usize)> for Map {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        // The x index wraps. y does not.
        let x = index.0 % self.width;
        let y = index.1;
        &self.data[y * self.width + x]
    }
}

fn read() -> Result<Map, Error> {
    let data = crate::utils::read("resources/input_3.txt")?
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Square,
                    '#' => Tile::Tree,
                    _ => {
                        eprintln!("Failed to match tile: {}", c);
                        unreachable!()
                    }
                })
                .collect::<Vec<Tile>>()
                .into_iter()
        })
        .collect();
    Ok(Map::from(data, 31, 323)?)
}

pub fn solve() -> Result<(), Error> {
    let map = read()?;

    // Part 1
    println!("Day 3, Part 1: {}", map.trees(&(3, 1)));

    // Part 2
    println!(
        "Day 3, Part 2: {}",
        vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            // .map(|slope| map.trees(slope))
            .map(|slope| map.trees(slope))
            .product::<usize>()
    );

    Ok(())
}
