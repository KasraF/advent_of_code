pub type Error = Box<dyn std::error::Error>;

mod day_1;
mod day_2;
mod utils;

fn main() -> Result<(), Error> {
    println!("Hello, advent of code 2020!");
    day_1::solve()?;
    day_2::solve()?;

    Ok(())
}
