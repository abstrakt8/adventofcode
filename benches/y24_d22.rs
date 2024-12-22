use adventofcode::y24;

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/y24/22.in");

#[divan::bench(sample_count = 300)]
fn p1() {
    assert_eq!(17163502021, y24::d22::run1(INPUT));
}

#[divan::bench(sample_count = 10)]
fn p2() {
    assert_eq!(1938, y24::d22::run2(INPUT));
}

#[divan::bench(sample_count = 10)]
fn p2_fast() {
    assert_eq!(1938, y24::d22::run2_fast(INPUT));
}
