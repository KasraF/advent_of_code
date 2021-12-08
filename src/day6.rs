use crate::utils::{readfile, Error};

fn simulate(fish: &mut [usize; 9]) {
    fish.rotate_left(1);
    fish[6] += fish[8];
}

pub fn main(file: Option<String>) -> Result<(String, String), Error> {
    let input = readfile(&file.unwrap_or("resources/day6.txt".to_string()))?;
    let fish_times: Vec<_> = input
        .split(',')
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect();
    let mut fish = [0; 9];
    for time in fish_times {
        fish[time] += 1;
    }

    for _ in 0..80 {
        simulate(&mut fish);
    }
    let part1 = fish.iter().sum::<usize>().to_string();

    for _ in 0..(256 - 80) {
        simulate(&mut fish);
    }

    Ok((part1, fish.iter().sum::<usize>().to_string()))
}
