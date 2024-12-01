use std::cell::UnsafeCell;

const N: usize = 2513; // Worst case

thread_local! {
    static TRIE: UnsafeCell<[[[u16; 10]; N]; 2]> = UnsafeCell::new([[[0; 10]; N]; 2]);
    static CNT: UnsafeCell<[[u32; N]; 2]> = UnsafeCell::new([[0; N]; 2]);
    static TSIZE: UnsafeCell<[u16; 2]> = UnsafeCell::new([1; 2]);
    static CUR: UnsafeCell<[u16; 2]> = UnsafeCell::new([1; 2]);
}

pub fn run(content: &str) -> u32 {
    TRIE.with(|trie| {
        CNT.with(|cnt| {
            TSIZE.with(|tsize| {
                CUR.with(|cur| {
                    unsafe {
                        let trie = &mut *trie.get();
                        let cnt = &mut *cnt.get();
                        let tsize = &mut *tsize.get();
                        let cur = &mut *cur.get();

                        let mut num = 0;
                        let mut p = 0;
                        let mut ans2 = 0;

                        for c in content.bytes() {
                            if c.is_ascii_digit() {
                                let d = (c as usize) - ('0' as usize);
                                if trie[p][cur[p] as usize][d] == 0 {
                                    trie[p][cur[p] as usize][d] = tsize[p];
                                    tsize[p] += 1;
                                }
                                cur[p] = trie[p][cur[p] as usize][d];
                                cur[1^p] = trie[1^p][cur[1^p] as usize][d];
                                num = num * 10 + d;
                            } else if num > 0 {
                                cnt[p][cur[p] as usize] += 1;

                                ans2 += cnt[1^p][cur[1^p] as usize] * num as u32;

                                // Reset and swap
                                p ^= 1;
                                num = 0;
                                cur[0] = 1;
                                cur[1] = 1;
                            }
                        }
                        ans2 += cnt[1^p][cur[1^p] as usize] * num as u32;

                        // Reset thread-local variables for next use
                        *trie = [[[0; 10]; N]; 2];
                        *cnt = [[0; N]; 2];
                        *tsize = [1; 2];
                        *cur = [1; 2];

                        ans2
                    }
                })
            })
        })
    })
}
