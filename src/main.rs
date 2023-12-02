use std::env;

mod day1;
mod day2;
mod utils;

fn main() -> Result<(), utils::Error> {
    match env::args().skip(1).next() {
        Some(arg) => match arg.as_str() {
            "1" => day1::main(),
            "2" => day2::main(),
            n => Err(format!("Day not yet implemented: {n}").into()),
        },
        None => Err("Please provide the AoC day to run.".into()),
    }
}
