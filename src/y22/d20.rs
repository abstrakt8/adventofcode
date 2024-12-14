use num_traits::{PrimInt, ToPrimitive};
use std::ops::{Add, AddAssign, Index};

#[derive(Clone, Copy, Debug)]
struct WrappedIndex {
    index: usize,
    len: usize,
}

impl WrappedIndex {
    pub fn new(index: usize, len: usize) -> Self {
        assert!(len > 0);
        Self { index, len }
    }

    fn wrap<T, K>(index: T, n: K) -> usize
    where
        T: PrimInt,
        K: PrimInt,
    {
        let n = n.to_i64().unwrap();
        let index = index.to_i64().unwrap();
        (((index % n) + n) % n).to_usize().unwrap()
    }

    fn add_offset<T: PrimInt>(&mut self, offset: T) {
        let offset = Self::wrap(offset, self.len);
        self.index = Self::wrap(self.index + offset, self.len);
    }
}

impl<T: PrimInt> Add<T> for WrappedIndex {
    type Output = Self;

    fn add(mut self, rhs: T) -> Self::Output {
        self.add_offset(rhs);
        self
    }
}

impl<T> Index<WrappedIndex> for [T] {
    type Output = T;
    fn index(&self, idx: WrappedIndex) -> &Self::Output {
        &self[idx.index]
    }
}

impl<T: PrimInt> AddAssign<T> for WrappedIndex {
    fn add_assign(&mut self, rhs: T) {
        self.add_offset(rhs);
    }
}

// There might be 4 cases

// 1. Go to the left, stay on the left side
// 0, 1, 2, 3, 4, 5, 6
// 0, 1, 2, ., 4, 5, 6
// 0, 1, 3, 2, 4, 5, 6
// Delta = -1 Insert pos = 2

// 2. Go to the left wrap around to the other side
// 0, 1, 2, 3, 4, 5, 6
// 0, 1, 2, ., 4, 5, 6
// 0, 1, 2, 4, 5, 3, 6
// Delta = -4 Insert pos = 5  !!! Special case

// 3. Go to the right stay on the right side
// 0, 1, 2, 3, 4, 5, 6
// 0, 1, 2, ., 4, 5, 6
// 0, 1, 2, 4, 3, 5, 6
// Delta = +1 Insert pos = 4

// 4. Go to the right wrap to the left side
// 0, 1, 2, 3, 4, 5, 6
// 0, 1, 2, ., 4, 5, 6
// 0, 3, 1, 2, 4, 5, 6
// Delta = +4 Insert pos = 1 !!! Special case

// Fazit: when wrapping around, insert pos will never be first or last element
// Also the number of times to wrap around to the same position is (n-1), consider 1 2 3

// Going to the left L times should be equivalent to going to the right N-1-L times
// Now only handle the going to the right case, which is just normal if it stays, +1 if I wrap around

pub fn score(vec: &Vec<i64>) -> i64 {
    let pos0 = vec.iter().position(|val| *val == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| vec[WrappedIndex::wrap(pos0 + i, vec.len())])
        .sum()
}

pub fn simulate_wrapping(vec: Vec<i64>, times: usize) -> Vec<i64> {
    let mut vec: Vec<(usize, i64)> = vec.into_iter().enumerate().collect();
    let n = vec.len();
    
    for _ in 0..times {
        for inner_times in 0..n {
            let i = vec.iter().position(|&x| x.0 == inner_times).unwrap();
            let d = vec[i].1;
            if d == 0 {
                continue;
            }
            let removed = vec.remove(i);
            let d = WrappedIndex::wrap(d, n - 1);
            let mut j = WrappedIndex::wrap(i + d, n);
            if i + d >= n {
                j = WrappedIndex::wrap(j + 1, n);
            }
            vec.insert(j, removed);
        }
    }

    vec.into_iter().map(|(_, val)| val).collect()
}

pub fn solve2(vec: Vec<i64>) -> i64 {
    let decryption_key = 811589153;
    let vec = vec.into_iter().map(|x| x * decryption_key).collect();
    let vec = simulate_wrapping(vec, 10);

    score(&vec)
}

pub fn solve1(vec: Vec<i64>) -> i64 {
    score(&simulate_wrapping(vec, 1))
}

pub fn run(content: &str) -> (i64, i64) {
    let vec: Vec<i64> = content
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    let ans1 = solve1(vec.clone());
    let ans2 = solve2(vec);
    (ans1, ans2)
}
