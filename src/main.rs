
use std::fs::read_to_string;
use color_eyre::Result;
use adventofcode::y24::p6::run;

pub fn main() -> Result<()> {
    color_eyre::install()?;
    // println!("ANS {}", y24::p2_dp::run(&str));
    let str = read_to_string("./inputs/y24/6_example.in")?;
    println!("Example {}", run(&str));

    let str = read_to_string("./inputs/y24/6.in")?;
    println!("Output {}", run(&str));

    // Day 3
    // Example 161
    // Output 184511516


    Ok(())
}

