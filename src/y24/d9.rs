use std::cmp::min;
use std::collections::BTreeSet;

type Int = u64;

pub fn gauss(n: Int) -> Int {
    n * (n + 1) / 2
}

pub fn gauss2(a: Int, b: Int) -> Int {
    gauss(b as Int) - if a > 0 { gauss((a - 1) as Int) } else { 0 }
}

pub fn check_sum_range(a: Int, size: Int, id: Int) -> Int {
    gauss2(a, a + size - 1) * id
}

pub fn run1(content: &str) -> Int {
    let mut d = 0;

    let mut check_sum: Int = 0;

    let bytes = content.as_bytes();

    let mut i = 0;
    let mut j = bytes.len() - 1;

    if j % 2 == 1 {
        j -= 1;
    }

    let file_id = |i: usize| -> Int { (i / 2) as Int };

    let space = |i: usize| -> Int { (bytes[i] - b'0') as Int };

    let mut cj = space(j);
    while i < j {
        check_sum += check_sum_range(d, space(i), file_id(i));
        d += space(i);
        i += 1;
        // i cant be j here because j must be two apart from i
        let mut ci = space(i);
        while ci > 0 && i < j {
            let mn = min(ci, cj);
            check_sum += check_sum_range(d, mn, file_id(j));
            d += mn;

            ci -= mn;
            cj -= mn;

            if ci == 0 {
                break;
            }
            j -= 2;
            // Otherwise we might overcount
            if i < j {
                cj = space(j);
            }
        }

        i += 1;
    }

    if cj > 0 {
        check_sum += check_sum_range(d, cj, file_id(j));
    }

    check_sum
}

pub fn run2(content: &str) -> Int {
    let bytes = content.as_bytes();

    let n = bytes.len();

    let space = |i: usize| -> Int { (bytes[i] - b'0') as Int };
    let file_id = |i: usize| -> Int { (i / 2) as Int };

    let mut pos: Vec<BTreeSet<Int>> = vec![BTreeSet::new(); 10];
    let mut pre = vec![0; n];

    for i in 0..n {
        if i % 2 == 1 {
            pos[space(i) as usize].insert(pre[i - 1]);
        }
        if i > 0 {
            pre[i] = pre[i - 1];
        }
        pre[i] += space(i);
    }

    let mut check_sum = 0;
    for j in (0..n).rev().step_by(2) {
        if j % 2 == 1 {
            continue;
        }
        let supposed = if j > 0 { pre[j - 1] } else { 0 };
        let cj = space(j);
        
        let smallest_available = (cj..=9)
            .filter_map(|s| {
                if let Some(i) = pos[s as usize].first() {
                    if *i < supposed {
                        return Some((*i, s));
                    }
                }
                None
            })
            .min();
        if let Some((i, ci)) = smallest_available {
            pos[ci as usize].remove(&i);

            check_sum += check_sum_range(i, cj, file_id(j));

            let ck = ci - cj;
            let k = i + cj;
            if ck > 0 {
                pos[ck as usize].insert(k);
            }
        } else {
            check_sum += check_sum_range(supposed, cj, file_id(j));
        }
    }

    check_sum
}

pub fn run(content: &str) -> Int {
    // run1(content)
    // 6398096697992
    run2(content)
}
