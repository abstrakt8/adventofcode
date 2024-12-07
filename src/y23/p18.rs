use crate::utils::grid2d::{Direction, GridCell};
use color_eyre::owo_colors::OwoColorize;
use color_eyre::{Report, Result};
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

struct Operation {
    dir: Direction,
    amount: i32,
    color: String,
}

impl Operation {}


impl FromStr for Operation {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(' ');

        let dir = it.next().ok_or_else(|| Report::msg("No direction given"))?;
        let dir = dir.chars().nth(0).ok_or_else(|| Report::msg("Empty first token"))?;
        let dir = Direction::from_letter(dir)?;

        let amount = it.next().ok_or_else(|| Report::msg("No amount given"))?;
        let amount = amount.parse::<i32>().map_err(Report::new)?;

        let color = it.next().ok_or_else(|| Report::msg("No color given"))?;
        let color = color[1..color.len() - 1].to_owned();

        Ok(Operation { dir, amount, color })
    }
}

pub fn run(content: &str) -> Result<i32> {
    let operations: Result<Vec<Operation>> = content
        .lines()
        .map(str::parse::<Operation>)
        .collect();
    // I don't know why I can't just specify .collect()?; and define the type as Vec<Operation>
    let operations = operations?;
    let mut current_cell = GridCell::new(0, 0);

    let mut grid: HashMap<GridCell, i32> = HashMap::new();
    let mut generation = 0;
    let (mut min_cell, mut max_cell) = (current_cell, current_cell);
    for Operation { amount, dir, .. } in &operations {
        for i in 0..*amount {
            grid.insert(current_cell, generation);
            current_cell = current_cell.step_once(*dir);

            min_cell = GridCell::min_components(min_cell, current_cell);
            max_cell = GridCell::max_components(max_cell, current_cell);
        }
    }

    generation += 1;
    print(&grid, &min_cell, &max_cell);

    // The ones on the edge
    let mut ans = grid.len() as i32;
    println!("ans={ans}");

    let mut inside_generation = 0;

    // A cell is inside, if it doesnt touch the boundaries of the imaginary grid when flood filling
    for x in (min_cell.x..=max_cell.x) {
        for y in (min_cell.y..=max_cell.y) {
            let cell = GridCell::new(x, y);
            if grid.contains_key(&cell) {
                continue;
            }
            let mut q = VecDeque::new();
            let mut inside = true;
            let mut count = 0;

            grid.insert(cell, generation);
            q.push_back(cell);

            while let Some(cell) = q.pop_front() {
                count += 1;
                for neighbor in cell.neighbors4() {
                    if !neighbor.inside_rectangle(min_cell, max_cell) {
                        inside = false;
                    } else if !grid.contains_key(&neighbor) {
                        grid.insert(neighbor, generation);
                        q.push_back(neighbor);
                        print(&grid, &min_cell, &max_cell);
                    }
                }
            }
            if inside {
                inside_generation = generation;
                ans += count;
                print(&grid, &min_cell, &max_cell);
                println!("Inside: {inside_generation} {count} {ans}");
            }
            generation += 1;
        }
    }

    print(&grid, &min_cell, &max_cell);

    Ok(ans)
}

fn print(grid: &HashMap<GridCell, i32>, min_cell: &GridCell, max_cell: &GridCell) {
    for x in (min_cell.x..=max_cell.x) {
        for y in (min_cell.y..=max_cell.y) {
            let c = grid.get(&GridCell::new(x, y));
            let c = c.map(|x| x.to_string()).unwrap_or_else(|| ".".to_string());
            print!("{c}")
        }
        println!()
    }
    println!("--------------")
}

#[cfg(test)]
mod test {
    use crate::utils::grid2d::Direction;
    use crate::y23::p18::{run, Operation};
    use color_eyre::*;

    pub fn setup() {
        color_eyre::install().unwrap();
    }

    #[test]
    pub fn test_parse_err() {
        setup();
        let err = "R X (#70c710)".parse::<Operation>();
        if let Err(ref e) = err {
            println!("{}", e);
        }
        assert!(err.is_err());
    }

    #[test]
    pub fn test_parse() -> Result<()> {
        let Operation { amount, color, dir } = "R 6 (#70c710)".parse::<Operation>()?;
        assert_eq!(dir, Direction::RIGHT);
        assert_eq!(amount, 6);
        assert_eq!(color, "#70c710");
        Ok(())
    }

    #[test]
    pub fn test_example() -> Result<()> {
        let content = r##"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"##;
        assert_eq!(run(content)?, 62);
        Ok(())
    }
}
