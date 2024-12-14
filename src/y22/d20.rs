pub fn run(content: &str) -> i32 {
    let mut vec: Vec<(usize, i32)> = content
        .lines()
        .map(|x| x.parse().unwrap())
        .enumerate()
        .collect();

    let mut i = 0;
    let n = vec.len() as i32;

    let add = |a: i32, d: i32| (a + d + n) % n;

    // n <= 5000
    for times in 0..n as usize {
        // We skip the ones we already swapped
        while i < n && vec[i as usize].0 < times {
            i += 1;
        }

        let d = add(vec[i as usize].1, 0);

        if d == 0 {
            continue;
        }

        let new_pos = if i + d < 0 {
            add(add(i, d), -1)
        } else if d > 0 {
            add(add(i, d), 1)
        } else {
            add(i, d)
        };

        let elem = vec.remove(i as usize);
        // After removing it, we need to shift the elements to the left, so decrement
        if i < new_pos {
            vec.insert(add(new_pos, -1) as usize, elem);
        } else {
            vec.insert(new_pos as usize, elem);
        }
        if vec.len() < 10 {
            println!("times={:} {:?}", times, vec);
        }
    }

    let pos0 = vec.iter().position(|(_, val)| *val == 0).unwrap();

    let ans1 = [1000, 2000, 3000]
        .into_iter()
        .map(|x| vec[add(pos0 as i32, x) as usize].1)
        .sum();
    ans1
}
