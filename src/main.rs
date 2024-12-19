use color_eyre::Result;
use std::fs::read_to_string;


pub fn main() -> Result<()> {
    color_eyre::install()?;
    let suffixes = [
        "_example.in",
        // "_another.in",
        ".in"
    ];
    suffixes.iter().for_each(|&suffix| {
        use adventofcode::y22::d22::run as run;
        const YEAR: u32 = 22;
        const DAY: u32 = 22;
        let file_name = format!("./inputs/y{YEAR}/{DAY}{suffix}");
        let input = read_to_string(&file_name);
        if let Ok(input) = input {
            let ans = run(&input);
            println!("{file_name}: {:?}", ans);
        } else {
            println!("Skipped {file_name}");
        }
    });

    Ok(())
}
