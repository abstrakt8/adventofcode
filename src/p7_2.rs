use std::collections::HashMap;

fn to_num(s: char) -> u32 {
    if let Some(d) = s.to_digit(10) {
        d
    } else {
        match s {
            'T' => 10,
            'J' => 1, // special caes here
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => { 0 }
        }
    }
}

fn highest_kind(exists: [usize; 6], jokers: usize) -> u32 {
    if jokers >= 5 { return 6; }
    for i in 0..=jokers { if 5 >= i && exists[5 - i] >= 1 { return 6; } }

    for i in 0..=jokers { if 4 >= i && exists[4 - i] >= 1 { return 5; } }

    // jokers should be <= 2 now otherwise four of a kind can be formed
    assert!(jokers <= 2);

    // Full house
    if exists[3] >= 1 {
        if exists[2] >= 1 { return 4; }
        if jokers >= 1 { return 4; }
    }
    if exists[2] >= 2 && jokers >= 1 { return 4; }
    if exists[2] >= 1 && jokers >= 2 { return 4; }


    // Three of a kind
    for i in 0..=jokers { if 3 >= i && exists[3 - i] >= 1 { return 3; } }

    // Two pairs
    if exists[2] >= 2 { return 2; }
    if exists[2] >= 1 && jokers >= 1 { return 2; }
    if jokers >= 2 { return 2; }

    // One pair
    if exists[2] >= 1 { return 1; }
    if jokers >= 1 { return 1; }

    // None
    return 0;
}

fn encode(s: &str) -> Vec<u32> {
    let mut h: HashMap<u32, usize> = HashMap::new();
    let mut exists = [0usize; 5 + 1];

    let mut ints: Vec<_> = s.chars().map(to_num).collect();

    let J = to_num('J');

    ints.iter().for_each(|c| {
        *h.entry(*c).or_insert(0) += 1
    });

    let mut jokers = 0;
    h.iter().for_each(|(c, cnt)| {
        if *c != J {
            exists[*cnt] += 1;
        } else {
            jokers = *cnt;
        }
    });

    // 252156913 (too low)
    // 254287290 (too high)
    // 253680676 ?
    // 253718286
    let ans = highest_kind(exists, jokers);
    let mut out = vec![ans];
    out.append(&mut ints);
    return out;
}


pub fn run(input: &str) -> u64 {
    let mut cards: Vec<_> = input.lines().filter_map(|s| {
        if !s.is_empty() {
            let t: Vec<&str> = s.split_whitespace().collect();
            return Some((encode(t[0]), t[1].parse::<u64>().unwrap()));
        } else {
            None
        }
    }).collect();
    cards.sort_by(|(a, _), (b, _)| a.cmp(b));
    cards.iter().enumerate().map(|(idx, (_, bid))| (idx as u64 + 1) * bid).sum()
}
//
// mod tests {
//     use crate::p7_2::run;
//
//     #[test]
//     fn test() {
//         let input = "32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483";
//         assert_eq!(5905, run(input));
//     }
// }