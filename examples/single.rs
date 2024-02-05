use std::fs::read_to_string;
use color_eyre::Result;
use adventofcode::y23::{p25_1};

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let str = read_to_string("25.in")?;
    println!("ANS {}", p25_1::run(&str));

    Ok(())
}

