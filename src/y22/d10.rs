pub fn run(content: &str) -> i32 {
    let mut cycle: usize = 0;
    let mut ans1 = 0;
    let mut x = 1;
    let mut screen = [[b'.'; 40]; 6];

    let mut run_cycle = |cycle: &mut usize, ans1: &mut i32, x: i32| {
        let i = *cycle / 40;
        let j = *cycle % 40;
        if x.abs_diff(j as i32) <= 1 {
            screen[i][j] = b'#';
        }

        *cycle += 1;
        if *cycle >= 20 && (*cycle - 20) % 40 == 0 {
            *ans1 += *cycle as i32 * x;
        }
    };

    for line in content.lines() {
        if line.starts_with("noop") {
            run_cycle(&mut cycle, &mut ans1, x);
        } else {
            let (_, v) = line.split_once(" ").unwrap();
            let v: i32 = v.parse().unwrap();
            run_cycle(&mut cycle, &mut ans1, x);
            run_cycle(&mut cycle, &mut ans1, x);
            x += v;
        }
    }

    // Ans2: REHPRLUB
    for i in 0..screen.len() {
        println!("{:?}", String::from_utf8(screen[i].to_vec()));
    }
    
    ans1
}
