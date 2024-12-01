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

pub fn run(content: &str) -> u32 {
    // const N: usize = 1000 * 5 + 2; // Worst case
    const N: usize = 2513; // Worst case
    let mut trie = [[[0; 10]; N]; 2];
    let mut cnt = [[0; N]; 2];
    let mut tsize = [1; 2];
    let mut cur = [1; 2];
    let mut num = 0;

    let mut p = 0;

    let mut ans2 = 0;

    for c in content.bytes() {
        if c.is_ascii_digit() {
            let d = (c as usize) - ('0' as usize);
            if trie[p][cur[p]][d] == 0 {
                trie[p][cur[p]][d] = tsize[p];
                tsize[p] += 1;
            }
            cur[p] = trie[p][cur[p]][d];
            cur[1^p] = trie[1^p][cur[1^p]][d];
            num = num * 10 + d;
        } else if num > 0 {
            cnt[p][cur[p]] += 1;

            ans2 += cnt[1^p][cur[1^p]] * num as u32;

            // Reset and swap
            p ^= 1;
            num = 0;
            cur[0] = 1;
            cur[1] = 1;
        }
    }
    ans2 += cnt[1^p][cur[1^p]] * num as u32;

    ans2
}