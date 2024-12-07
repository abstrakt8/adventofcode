pub fn run(content: &str) -> u32 {
    const H: usize = 2513 + 1685 + 3 + 1000;
    const TRIE_SIZE: usize = H * 10;

    let mut trie = [0u16; TRIE_SIZE];
    let mut cnt = [0u32; H];
    let mut tsize = 3u16;
    let mut cur = [1u16, 2];
    let mut num = 0u32;
    let mut p = 0usize;
    let mut ans2 = 0u64;  // Use u64 to avoid potential overflow

    let bytes = content.as_bytes();
    let len = bytes.len();

    unsafe {
        let trie_ptr = trie.as_mut_ptr();
        let cnt_ptr = cnt.as_mut_ptr();
        let bytes_ptr = bytes.as_ptr();
        let mut i = 0;

        while i < len {
            let c = *bytes_ptr.add(i);
            if c.is_ascii_digit() {
                let d = (c - b'0') as usize;
                let cur_p = *cur.get_unchecked(p) as usize;
                let trie_index = cur_p * 10 + d;
                if *trie_ptr.add(trie_index) == 0 {
                    *trie_ptr.add(trie_index) = tsize;
                    tsize = tsize.wrapping_add(1);
                }
                *cur.get_unchecked_mut(p) = *trie_ptr.add(trie_index);
                *cur.get_unchecked_mut(p ^ 1) = *trie_ptr.add(*cur.get_unchecked(p ^ 1) as usize * 10 + d);
                num = num.wrapping_mul(10).wrapping_add(d as u32);
            } else if num != 0 {
                let cur_p = *cur.get_unchecked(p) as usize;
                let cur_p_xor = *cur.get_unchecked(p ^ 1) as usize;
                *cnt_ptr.add(cur_p) = (*cnt_ptr.add(cur_p)).wrapping_add(1);
                ans2 = ans2.wrapping_add((*cnt_ptr.add(cur_p_xor) as u64).wrapping_mul(num as u64));
                p ^= 1;
                num = 0;
                cur = [1, 2];
            }
            i = i.wrapping_add(1);
        }
        if num != 0 {
            let cur_p_xor = *cur.get_unchecked(p ^ 1) as usize;
            ans2 = ans2.wrapping_add((*cnt_ptr.add(cur_p_xor) as u64).wrapping_mul(num as u64));
        }
    }

    ans2 as u32
}
