use itertools::Itertools;
use serde_json::Value;
use std::cmp::Ordering;

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Array(a), Value::Array(b)) => a
            .iter()
            .zip(b)
            .map(|(a, b)| compare(a, b))
            .find(|&ord| ord != Ordering::Equal)
            .unwrap_or_else(|| a.len().cmp(&b.len())),
        (Value::Number(a), Value::Number(b)) => a.as_u64().unwrap().cmp(&b.as_u64().unwrap()),
        (Value::Number(_), Value::Array(_)) => compare(&Value::Array(vec![a.clone()]), b),
        (Value::Array(_), Value::Number(_)) => compare(a, &Value::Array(vec![b.clone()])),
        _ => unreachable!(),
    }
}

pub fn run(content: &str) -> (usize, usize) {
    let input: Vec<Vec<Value>> = content
        .split("\n\n")
        .map(|lines| {
            lines
                .split("\n")
                .map(|line| serde_json::from_str(line).unwrap())
                .collect()
        })
        .collect();

    let ans1 = input
        .iter()
        .enumerate()
        .filter_map(|(i, data)| {
            compare(&data[0], &data[1])
                .eq(&Ordering::Less)
                .then_some(i + 1)
        })
        .sum();

    let divider1: Value = serde_json::from_str("[[2]]").unwrap();
    let divider2: Value = serde_json::from_str("[[6]]").unwrap();
    let dividers = [divider1, divider2];

    let mut all: Vec<Value> = input
        .into_iter()
        .flatten()
        .chain(dividers.iter().cloned())
        .collect::<Vec<_>>();
    all.sort_by(compare);

    let ans2 = all
        .iter()
        .positions(|v| dividers.contains(v))
        .map(|i| i + 1)
        .product();

    /*
    ./inputs/y22/13_example.in: (13, 140)
    ./inputs/y22/13.in: (5503, 20952)
     */
    (ans1, ans2)
}
