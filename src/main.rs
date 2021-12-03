use crate::utils::Error;

mod day1;
mod day2;
mod day3;
mod utils;

fn main() -> Result<(), Error> {
    let solutions = [crate::day1::main, crate::day2::main, crate::day3::main];
    let mut args = std::env::args();
    args.next(); // Skip the program name
    let day = args
        .next()
        .expect("Please specify the day")
        .parse::<usize>()
        .expect("Failed to parse day number");
    let file = args.next();

    if day > solutions.len() {
        if day <= 31 {
            eprintln!("Day {} not yet implemented.", day);
        } else {
            eprintln!("What calendar are you even using?!");
        }
        std::process::exit(1);
    }

    let (part1, part2) = solutions[day - 1](file)?;
    println!("Day {}: {}, {}", day, part1, part2);
    Ok(())
}
