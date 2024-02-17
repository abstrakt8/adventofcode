use std::fs::read_to_string;
use color_eyre::Result;
use adventofcode::y23::{p25_1};

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let str = read_to_string("20.in")?;
    println!("ANS {}", adventofcode::y23::p20_2::run(&str));

    Ok(())
}

