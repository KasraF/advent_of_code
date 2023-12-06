use std::collections::HashMap;
use std::str::FromStr;

use crate::utils;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Item {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temp,
    Humidity,
    Location,
}

impl FromStr for Item {
    type Err = utils::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "seed" => Ok(Item::Seed),
            "soil" => Ok(Item::Soil),
            "fertilizer" => Ok(Item::Fertilizer),
            "water" => Ok(Item::Water),
            "light" => Ok(Item::Light),
            "temperature" => Ok(Item::Temp),
            "humidity" => Ok(Item::Humidity),
            "location" => Ok(Item::Location),
            _ => Err(format!("Invalid Item type: {s}").into()),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct AElem {
    src: u32,
    dst: u32,
    len: u32,
}

impl FromStr for AElem {
    type Err = utils::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_whitespace();
        let dst = nums
            .next()
            .ok_or(format!("Destination not found: {s}"))?
            .parse()?;
        let src = nums
            .next()
            .ok_or(format!("Source not found: {s}"))?
            .parse()?;
        let len = nums.next().ok_or(format!("nums not found"))?.parse()?;

        if let Some(rest) = nums.next() {
            Err(format!("Line not empty after last number parsed: {s} ({rest})").into())
        } else {
            Ok(AElem { src, dst, len })
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Default)]
struct Almanac {
    /// Invariant: This vector must always be
    /// sorted by the AoCElem source. Since
    /// the lookup algorithm relies on binary search.
    inner: Vec<AElem>,
}

impl Almanac {
    fn insert(&mut self, elem: AElem) {
        let idx = self.inner.binary_search_by_key(&elem.src, |x| x.src);

        match idx {
            Ok(_) => panic!("Index already exists: {elem:?}"),
            Err(idx) => self.inner.insert(idx, elem),
        }
    }

    fn lookup(&self, s: u32) -> u32 {
        match self.inner.binary_search_by_key(&s, |x| x.src) {
            Ok(idx) => {
                // We found it!
                self.inner[idx].dst
            }
            Err(idx) => {
                // Two cases are possible:
                // 1. It belongs to the range at idx.
                // 2. It does not fit at all
                if idx == 0 {
                    s
                } else {
                    let AElem { src, dst, len } = &self.inner[idx - 1];
                    debug_assert!(*src < s, "{src} !< {s}");
                    if *len >= s - *src {
                        dst + (s - *src)
                    } else {
                        s
                    }
                }
            }
        }
    }
}

pub fn main() -> Result<(), utils::Error> {
    let mut input = utils::read_lines(5)?;

    let seeds: Vec<u32> = input
        .next()
        .unwrap()
        .as_str()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();

    let mut maps: HashMap<Item, (Item, Almanac)> = HashMap::with_capacity(7);
    let mut curr = None;

    for line in input {
        if line.is_empty() {
            continue;
        }

        if let Some(rest) = line.strip_suffix(" map:") {
            // Time to reset the curr
            if let Some((src, dst, alm)) = curr {
                maps.insert(src, (dst, alm));
            }

            let (src, dst) = rest
                .split_once("-to-")
                .ok_or(format!("-to- not found in: {rest}"))?;
            curr = Some((src.parse()?, dst.parse()?, Almanac::default()))
        } else if let Some((_, _, alm)) = &mut curr {
            // parse the numbers!
            alm.insert(line.parse()?);
        }
    }

    if let Some((src, dst, alm)) = curr {
        maps.insert(src, (dst, alm));
    } else {
        panic!("Last almanac map was None?!");
    }

    let mut min_loc = u32::MAX;
    for seed in seeds {
        let (mut item, mut src) = (Item::Seed, seed);
        while item != Item::Location {
            let (dst, map) = maps.get(&item).unwrap();
            print!("{src} ({item:?}) -> ");
            item = *dst;
            src = map.lookup(src);
            println!("{src} ({item:?})");
        }
        if src < min_loc {
            min_loc = src;
        }
    }
    println!("min loc: {min_loc}");

    Ok(())
}
