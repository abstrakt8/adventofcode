use std::fs::read_to_string;
use color_eyre::Result;
use adventofcode::y23::{p18, p18_2};

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let str = read_to_string("18.in")?;
    // println!("Day1 {}", p18::run(&str)?);
    println!("Day2 {}", p18_2::run(&str)?);

    Ok(())
}

