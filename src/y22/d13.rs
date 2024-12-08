use itertools::Itertools;
use serde_json::Value;
use std::cmp::{min, Ordering};

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Array(a), Value::Array(b)) => {
            for i in 0..min(a.len(), b.len()) {
                let c = compare(&a[i], &b[i]);
                if c != Ordering::Equal {
                    return c;
                }
            }
            a.len().cmp(&b.len())
        }
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
    let dividers = [divider1.clone(), divider2.clone()];

    let mut all: Vec<Value> = input
        .into_iter()
        .flatten()
        .chain(dividers.into_iter())
        .collect::<Vec<_>>();
    all.sort_by(compare);

    let ans2 = all
        .iter()
        .positions(|v| v.eq(&divider1) || v.eq(&divider2))
        .map(|i| i + 1)
        .product();

    (ans1, ans2)
}
