pub fn run(content: &str) -> u32 {
    // const N: usize = 1000 * 5 + 2; // Worst case
    // const N: usize = 2513; // Worst case
    const H: usize = 2513 + 1685 + 3 + 1000;
    let mut trie = [[0; 10]; H];
    let mut cnt = [0; H];
    let mut tsize = 3;
    let mut cur = [1, 2];
    let mut num = 0;

    let mut p = 0;

    let mut ans2 = 0;

    for c in content.bytes() {
        if c.is_ascii_digit() {
            let d = (c as usize) - ('0' as usize);
            if trie[cur[p]][d] == 0 {
                trie[cur[p]][d] = tsize;
                tsize += 1;
            }
            cur[p] = trie[cur[p]][d];
            cur[1 ^ p] = trie[cur[1 ^ p]][d];
            num = num * 10 + d;
        } else if num > 0 {
            cnt[cur[p]] += 1;

            ans2 += cnt[cur[1 ^ p]] * num as u32;

            // Reset and swap
            p ^= 1;
            num = 0;
            cur[0] = 1;
            cur[1] = 2;
        }
    }
    ans2 += cnt[cur[1 ^ p]] * num as u32;

    ans2
}