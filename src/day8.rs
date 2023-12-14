use crate::utils;
use once_cell::unsync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use utils::LoopingIterable;

const NODE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").expect("Failed to parse node pattern")
});

enum Dir {
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Invalid direction: {value}"),
        }
    }
}

fn follow(nodes: &HashMap<String, (String, String)>, directions: &[Dir]) -> u32 {
    let mut curr = &nodes["AAA"];
    let mut count = 0;

    for dir in directions.looping_iter() {
        count += 1;
        let next = match dir {
            Dir::Left => &curr.0,
            Dir::Right => &curr.1,
        };

        if next == "ZZZ" {
            return count;
        } else {
            curr = &nodes[next];
        }
    }

    unreachable!()
}

fn ghost_follow(nodes: &HashMap<String, (String, String)>, directions: &[Dir]) -> u32 {
    let mut currs: Vec<_> = nodes.keys().filter(|k| k.ends_with("A")).collect();
    let mut count = 0;

    println!("Starting on {currs:?}");

    for dir in directions.looping_iter() {
        count += 1;

        // Step each of the nodes
        currs.par_iter_mut().for_each(|curr| {
            // Follow it!
            let (left, right) = &nodes[*curr];
            let next = match dir {
                Dir::Left => left,
                Dir::Right => right,
            };

            // update this node in-place
            *curr = next;
        });

        // See if it's done
        if currs.iter().all(|curr| curr.ends_with("Z")) {
            return count;
        }

        if currs.iter().any(|curr| curr.ends_with("Z")) {
            println!("Partial: {currs:?}");
        }
    }

    unreachable!()
}

fn parse_node(line: &str) -> Result<(String, (String, String)), utils::Error> {
    let matches = NODE_PATTERN
        .captures(line)
        .expect("Failed to parse line: {line}");
    let src = matches[1].to_string();
    let left = matches[2].to_string();
    let right = matches[3].to_string();
    Ok((src, (left, right)))
}

pub fn main() -> utils::ACResult {
    let mut lines = utils::read_lines(8)?;
    let directions: Vec<Dir> = lines
        .next()
        .expect("Failed to read first line")
        .trim()
        .chars()
        .map(|c| c.into())
        .collect();
    let nodes: HashMap<String, (String, String)> = lines
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|line| parse_node(line.as_str()).unwrap())
        .collect();

    println!("Day 8, Answer 1: {}", follow(&nodes, &directions));
    println!("Day 8, Answer 2: {}", ghost_follow(&nodes, &directions));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let dirs = &[Dir::Right, Dir::Left];
        let nodes = [
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
        .into_par_iter()
        .map(|line| parse_node(line).unwrap())
        .collect();

        assert_eq!(follow(&nodes, dirs), 2);
    }

    #[test]
    fn example_2() {
        let dirs = &[Dir::Left, Dir::Left, Dir::Right];
        let nodes = ["AAA = (BBB, BBB)", "BBB = (AAA, ZZZ)", "ZZZ = (ZZZ, ZZZ)"]
            .into_par_iter()
            .map(|line| parse_node(line).unwrap())
            .collect();

        assert_eq!(follow(&nodes, dirs), 6);
    }

    #[test]
    fn ghost_example() {
        let dirs = &[Dir::Left, Dir::Right];
        let nodes = [
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ]
        .into_iter()
        .map(|line| parse_node(line).unwrap())
        .collect();

        assert_eq!(ghost_follow(&nodes, dirs), 6);
    }
}
