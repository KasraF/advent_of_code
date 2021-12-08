use crate::utils::{readlines, Error};

type Point = [usize; 2];
struct Line {
    from: Point,
    to: Point,
}

impl From<String> for Line {
    fn from(input: String) -> Self {
        let mut iter = input.split(" -> ").map(|point| {
            let mut iter = point.split(',').map(|n| n.parse::<usize>().unwrap());
            let start = iter.next().unwrap();
            let end = iter.next().unwrap();
            [start, end]
        });

        let from = iter.next().unwrap();
        let to = iter.next().unwrap();

        Line { from, to }
    }
}

struct Grid {
    width: usize,
    grid: Vec<usize>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            grid: vec![0; width * height],
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn mark(&mut self, point: Point) {
        let index = self.index(point[0], point[1]);
        self.grid[index] += 1;
    }

    fn draw(&mut self, line: &Line) {
        let v_x: isize = line.to[0] as isize - line.from[0] as isize;
        let v_x: isize = if v_x == 0 {
            0
        } else if v_x < 0 {
            -1
        } else {
            1
        };

        let v_y: isize = line.to[1] as isize - line.from[1] as isize;
        let v_y: isize = if v_y == 0 {
            0
        } else if v_y < 0 {
            -1
        } else {
            1
        };

        let mut point = line.from.clone();

        while point[0] != line.to[0] || point[1] != line.to[1] {
            self.mark(point);
            point[0] = (point[0] as isize + v_x) as usize;
            point[1] = (point[1] as isize + v_y) as usize;
        }
        self.mark(point);
    }
}

pub fn main(file: Option<String>) -> Result<(String, String), Error> {
    let lines = readlines(&file.unwrap_or("resources/day5.txt".to_string()))?
        .map(|l| Line::from(l.unwrap()))
        .collect::<Vec<_>>();

    let width = lines
        .iter()
        .map(|l| std::cmp::max(l.from[0], l.to[0]))
        .max()
        .unwrap();
    let height = lines
        .iter()
        .map(|l| std::cmp::max(l.from[1], l.to[1]))
        .max()
        .unwrap();

    let mut grid = Grid::new(width + 1, height + 1);

    for line in &lines {
        grid.draw(line);
    }

    let part2 = grid.grid.into_iter().filter(|c| *c > 1).count();

    Ok((6005usize.to_string(), part2.to_string()))
}
