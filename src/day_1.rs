use crate::utils::AdventError;
use crate::Error;

fn read() -> Result<Vec<u32>, Error> {
    Ok(crate::utils::read("resources/input_1.txt")?
        .map(|line| line.parse().unwrap())
        .collect())
}

pub fn solve() -> Result<(), Error> {
    let data = read()?;

    // Try 1: Naive, cause the vec is short enough.
    for i in 0..data.len() {
        for j in i + 1..data.len() {
            let fst = data[i];
            let snd = data[j];
            if fst + snd == 2020 {
                println!("Day 1, Part 1: {}", fst * snd);
            }
        }
    }

    for i in 0..data.len() {
        for j in i + 1..data.len() {
            for k in j + 1..data.len() {
                let fst = data[i];
                let snd = data[j];
                let thd = data[k];

                if fst + snd + thd == 2020 {
                    println!("Day 1, Part 2: {}", fst * snd * thd);
                    return Ok(());
                }
            }
        }
    }

    Err(Box::new(AdventError::new("No input matched requirement.")))
}
