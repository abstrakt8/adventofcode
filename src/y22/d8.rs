use std::cmp::max;

const DIRS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

pub fn solve1(vec: &Vec<&[u8]>) -> u32 {
    let n = vec.len();
    let m = vec[0].len();

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            let f = DIRS.iter().any(|d: &[i32; 2]| {
                let mut x = i as i32;
                let mut y = j as i32;
                loop {
                    let nx = x + d[0];
                    let ny = y + d[1];
                    if !(0..n as i32).contains(&nx) || !(0..m as i32).contains(&ny) {
                        return true;
                    }

                    if vec[nx as usize][ny as usize] >= vec[i][j] {
                        return false;
                    }

                    (x, y) = (nx, ny);
                }
            });
            ans += f as u32;
        }
    }
    ans
}

pub fn solve2(vec: &Vec<&[u8]>) -> u32 {
    let n = vec.len();
    let m = vec[0].len();

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            let f = DIRS
                .iter()
                .map(|d: &[i32; 2]| {
                    let mut x = i as i32;
                    let mut y = j as i32;
                    let mut v = 0;
                    loop {
                        let nx = x + d[0];
                        let ny = y + d[1];
                        if !(0..n as i32).contains(&nx) || !(0..m as i32).contains(&ny) {
                            break;
                        }
                        v += 1;

                        if vec[nx as usize][ny as usize] >= vec[i][j] {
                            break;
                        }

                        (x, y) = (nx, ny);
                    }
                    v
                })
                .product();

            ans = max(ans, f);
        }
    }
    ans
}

pub fn run(content: &str) -> (u32, u32) {
    let vec: Vec<&[u8]> = content.lines().map(|x| x.as_bytes()).collect();

    (solve1(&vec), solve2(&vec))
}
