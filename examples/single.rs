use std::fs::read_to_string;
use color_eyre::Result;
use adventofcode::y23::{p19_1};

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let str = read_to_string("19.in")?;
    println!("Day1 {}", p19_1::run(&str));

    Ok(())
}

