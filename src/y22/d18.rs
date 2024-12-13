use std::collections::{HashMap, HashSet};

const DIRECTIONS: [[i32; 3]; 6] = [
    [-1, 0, 0],
    [1, 0, 0],
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
];

pub fn add(p: &[i32], q: &[i32]) -> Vec<i32> {
   p.into_iter().zip(q).map(|(a, b)| a + b).collect()
}

pub fn run(content: &str) -> (u32, u32) {
    let set: HashSet<Vec<i32>> = content
        .lines()
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut outside: HashMap<Vec<i32>, bool> = Default::default();

    fn is_outside(outside: &HashMap<Vec<i32>, bool>, set: &HashSet<Vec<i32>>, pos: Vec<i32>, vis: &mut HashSet<Vec<i32>>) -> bool {
        if pos.iter().any(|x| !(0..100).contains(x)) {
            return true;
        }
        if set.contains(&pos) {
            return false;
        }
        if let Some(ans) = outside.get(&pos) {
            return *ans;
        }
        if vis.contains(&pos) {
            return false;
        }
        vis.insert(pos.clone());
        // dbg!(&pos);

        let ans = {
            let mut flag = false;
            for d in DIRECTIONS {
                let q = add(&pos, &d);
                if is_outside(outside, set, q, vis) {
                    flag = true;
                    break;
                }
            }
            flag
        };

        ans
    }

    let mut ans1 = 0;
    let mut ans2 = 0;
    for point in &set {
        for dir in DIRECTIONS {
            let p: Vec<i32> = point.into_iter().zip(dir).map(|(&a, b)| a + b).collect();
            if set.contains(&p) {
                continue;
            }
            ans1 += 1;
            
            let mut vis : HashSet<Vec<i32>>= HashSet::new();
            let out = is_outside(&outside, &set, p.clone(), &mut vis);
            
            if out {
                ans2 += 1;
            } else {
                // dbg!(&p);
                // 2033 too low
            }
            for v in vis {
                outside.insert(v,out);
            }
        }
    }

    (ans1, ans2)
}
