use std::cmp::max;
use std::fs;

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn parse_game_id(input: &str) -> i32 {
    input.split(' ').last().unwrap().parse::<i32>().unwrap()
}

fn parse_rgb(input: &str) -> [i32; 3] {
    let mut out = [0, 0, 0];
    let tokens: Vec<&str> = input.split(", ").collect();
    for c in tokens {
        let t: Vec<&str> = c.split(' ').collect();
        let id = match t[1] {
            "red" => 0,
            "green" => 1,
            "blue" => 2,
            &_ => { panic!() }
        };
        out[id] = t[0].parse::<i32>().unwrap();
    }
    return out;
}

fn is_solvable(input: &str) -> bool {
    let sets: Vec<&str> = input.split("; ").collect();
    let [R, G, B] = [0, 0, 0];
    for game in sets {
        let [r, g, b] = parse_rgb(game);
        if !(r <= RED && g <= GREEN && b <= BLUE) {
            return false;
        }
    }
    return true;
}

fn game_id_value(input: &str) -> i32 {
    let tokens: Vec<&str> = input.split(": ").collect();
    let id = parse_game_id(tokens[0]);
    let possible = is_solvable(tokens[1]);
    if possible { id } else { 0 }
}

fn power_value(input: &str) -> i32 {
    let sets: Vec<&str> = input.split("; ").collect();
    let (mut R, mut G, mut B) = (0, 0, 0);
    for game in sets {
        let [r, g, b] = parse_rgb(game);
        (R, G, B) = (max(R, r), max(G, g), max(B, b));
    }
    return R * G * B;
}

fn game_power_value(input: &str) -> i32 {
    let tokens: Vec<&str> = input.split(": ").collect();
    power_value(tokens[1])
}

pub fn solve() {
    let contents = fs::read_to_string("2.in")
        .expect("Should have been able to read the file");

    let ans1: i32 = contents.lines().map(|l| game_id_value(&l)).sum();
    println!("{:}", ans1);

    let ans2: i32 = contents.lines().map(|l| game_power_value(&l)).sum();
    println!("{:}", ans2);
}


mod tests {
    use crate::p2::{is_solvable, parse_game_id, parse_rgb, game_id_value, power_value};

    #[test]
    fn test() {
        assert_eq!(0, game_id_value("Game 1: 19 blue, 12 red; 19 blue, 2 green, 1 red; 13 red, 11 blue"))
    }

    #[test]
    fn test_game_solvable() {
        assert_eq!(true, is_solvable("1 red, 3 green; 3 blue, 6 green; 5 blue, 1 red, 11 green; 1 red; 3 green, 13 blue"))
    }

    #[test]
    fn test_parse_rgb() {
        assert_eq!([1, 3, 0], parse_rgb("1 red, 3 green"));
        assert_eq!([0, 6, 3], parse_rgb("3 blue, 6 green"));
        assert_eq!([1, 11, 5], parse_rgb("5 blue, 1 red, 11 green"));
        assert_eq!([0, 3, 13], parse_rgb("3 green, 13 blue"));
    }

    #[test]
    fn test_game_id() {
        assert_eq!(1, parse_game_id("Game 1"));
        assert_eq!(42, parse_game_id("Game 42"));
        assert_eq!(100, parse_game_id("Game 100"));
    }

    #[test]
    fn test_power_value() {
        assert_eq!(48, power_value("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"))
    }
}

