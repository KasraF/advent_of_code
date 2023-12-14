use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

fn main() -> Result<(), utils::Error> {
    match env::args().skip(1).next() {
        Some(arg) => match arg.as_str() {
            "1" => day1::main(),
            "2" => day2::main(),
            "3" => day3::main(),
            "4" => day4::main(),
            "5" => day5::main(),
            "6" => day6::main(),
            "7" => day7::main(),
            n => Err(format!("Day not yet implemented: {n}").into()),
        },
        None => Err("Please provide the AoC day to run.".into()),
    }
}
