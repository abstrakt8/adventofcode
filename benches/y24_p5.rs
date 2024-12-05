use adventofcode::y24;

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/y24/5.in");

#[divan::bench(sample_count = 10000)]
fn bitsets() {
    y24::p5::run(INPUT);
}

#[divan::bench(sample_count = 1000)]
fn naive() {
    y24::p5_naive::run(INPUT);
}

// #[divan::bench(sample_count = 1000)]
// fn four_threads() {
//     y24::p4_rayon_lookup_table::run2_4threads(INPUT);
// }
