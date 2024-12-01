use std::collections::HashMap;

pub fn run_slow(content: &str) -> i32 {

    let mut vecs: [Vec<i32>; 2] = Default::default();
    let mut count: HashMap<i32, i32> = Default::default();

    content.lines().for_each(|line| {
        line.split_whitespace().enumerate().for_each(|(i, s)| {
            let x: i32 = s.parse().unwrap();
            vecs[i].push(x);
            if i == 1 {
                *count.entry(x).or_insert(0) += 1;
            }
        });
    });

    vecs[0].sort();
    vecs[1].sort();

    let mut ans1 = 0;
    let mut ans2 = 0;

    for i in 0..vecs[0].len() {
        ans1 += (vecs[0][i] - vecs[1][i]).abs();
        ans2 += count.get(&vecs[0][i]).unwrap_or(&0) * vecs[0][i];
    }
    //3574690
    //22565391
    ans2
}