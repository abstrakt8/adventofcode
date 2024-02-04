use std::collections::HashMap;

fn to_num(s: char) -> u32 {
    if let Some(d) = s.to_digit(10) {
        d
    } else {
        match s {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => { 0 }
        }
    }
}

fn encode(s: &str) -> Vec<u32> {
    let mut h: HashMap<u32, usize> = HashMap::new();
    let mut exists = [0usize; 5 + 1];

    let mut ints: Vec<_> = s.chars().map(to_num).collect();

    ints.iter().for_each(|c| { *h.entry(*c).or_insert(0) += 1 });
    h.iter().for_each(|(c, cnt)| { exists[*cnt] += 1; });

    let ans = {
        if exists[5] >= 1 { 6 }  //
        else if exists[4] >= 1 { 5 } //
        else if exists[3] >= 1 && exists[2] >= 1 { 4 } //
        else if exists[3] >= 1 { 3 } //
        else if exists[2] >= 2 { 2 } //
        else if exists[2] >= 1 { 1 }  //
        else { 0 }
    };
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

    // let mut sum = 0;
    // for i in 0..cards.len() {
    //     sum += cards[i].1 * (i + 1) as u64;
    // }
    // return sum;
    cards.iter().enumerate().map(|(idx, (_, bid))| (idx as u64 + 1) * bid).sum()
}

// mod tests {
//     use crate::p7_1::run;
//
//     #[test]
//     fn test() {
//         let input = "32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483";
//         assert_eq!(6440, run(input));
//     }
// }