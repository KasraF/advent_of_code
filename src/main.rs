pub type Error = Box<dyn std::error::Error>;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod utils;

fn main() -> Result<(), Error> {
    println!("Hello, advent of code 2020!");
    day_1::solve()?;
    day_2::solve()?;
    day_3::solve()?;
    day_4::solve()?;
    day_5::solve()?;

    Ok(())
}
