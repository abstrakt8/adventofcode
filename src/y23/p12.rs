pub fn solve(line: &str) -> i32 {
    let mut it = line.split(' ');
    let s = it.next().unwrap().chars().collect::<Vec<char>>();
    let c = it.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let ids = s.iter()
        .enumerate()
        .flat_map(|(i, &c)| if c == '?' { Some(i) } else { None })
        .collect::<Vec<usize>>();
    let k = ids.len();

    let mut ans = 0;
    for mask in 0..(1 << k) {
        let mut t = s.clone();
        for i in 0..k {
            t[ids[i]] = if ((mask >> i) & 1) > 0 { '#' } else { '.' };
        }
        let mut a = vec![];
        let mut cur = 0;
        for c in &t {
            if *c == '#' {
                cur += 1;
            } else {
                if cur > 0 {
                    a.push(cur);
                }
                cur = 0;
            }
        }
        (cur > 0).then(|| a.push(cur));
        if a == c {
            ans += 1;
        }
    }
    ans
}

pub fn run(content: &str) -> i32 {
    content.lines().map(solve).sum()
}

#[test]
pub fn example() {
    assert_eq!(solve("?###???????? 3,2,1"), 10);
}