use color_eyre::eyre::anyhow;
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Or,
    And,
    Xor,
}

impl Operation {
    fn from(i: &str) -> Self {
        match i {
            "XOR" => Self::Xor,
            "AND" => Self::And,
            "OR" => Self::Or,
            _ => unreachable!(),
        }
    }

    fn apply(&self, a: u8, b: u8) -> u8 {
        match self {
            Operation::Or => a | b,
            Operation::And => a & b,
            Operation::Xor => a ^ b,
        }
    }
}

pub fn run(content: &str) -> u64 {
    let (initial_values, gates) = content.split_once("\n\n").unwrap();

    let mut values: HashMap<&str, u8> = initial_values
        .lines()
        .map(|line| {
            let (id, value) = line.split_once(": ").unwrap();
            (id, value.parse::<u8>().unwrap())
        })
        .collect();

    let mut inputs: HashMap<&str, (Operation, &str, &str)> = Default::default();
    let mut outputs: HashMap<&str, Vec<&str>> = Default::default();
    let mut in_degrees: HashMap<&str, usize> = Default::default();

    for line in gates.lines() {
        let (connection, id) = line.split_once(" -> ").unwrap();

        let Some((id1, op, id2)) = connection.split(" ").collect_tuple() else {
            unreachable!()
        };

        inputs.insert(id, (Operation::from(op), id1, id2));
        *in_degrees.entry(id).or_default() += 2;
        outputs.entry(id1).or_default().push(id);
        outputs.entry(id2).or_default().push(id);
    }

    let mut queue: VecDeque<&str> = Default::default();
    for (id, _) in &values {
        queue.push_back(id);
    }

    let mut ans1 = 0u64;

    while let Some(front) = queue.pop_front() {
        if let Some((op, id1, id2)) = inputs.get(front) {
            values.insert(
                front,
                op.apply(*values.get(id1).unwrap(), *values.get(id2).unwrap()),
            );
        }

        if let Some(others) = outputs.get(front) {
            for out in others {
                let Entry::Occupied(mut vac) = in_degrees.entry(out) else {
                    continue;
                };
                *vac.get_mut() -= 1;
                if *vac.get() == 0 {
                    queue.push_back(out);
                }
            }
        }

        if let Some(rest) = front.strip_prefix("z") {
            let bit = rest.parse::<u8>().unwrap();
            ans1 |= (*values.get(front).unwrap() as u64) << bit;
        }
    }

    ans1
}

#[test]
pub fn part1_example1() {
    let content = r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    assert_eq!(2024, run(content));
}
