mod day1;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    day1::main()
}
