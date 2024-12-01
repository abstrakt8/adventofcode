pub fn run(content: &str) -> u32 {
    // const N: usize = 1000 * 5 + 2; // Worst case
    const N: usize = 2513; // Worst case
    let mut trie = [[[0; 10]; N]; 2];
    let mut cnt = [[0; N]; 2];
    let mut tsize = [1; 2];
    let mut cur = [1; 2];
    let mut num = 0;

    let mut p = 0;

    let mut ans2 = 0;

    for c in content.bytes() {
        if c.is_ascii_digit() {
            let d = (c as usize) - ('0' as usize);
            if trie[p][cur[p]][d] == 0 {
                trie[p][cur[p]][d] = tsize[p];
                tsize[p] += 1;
            }
            cur[p] = trie[p][cur[p]][d];
            cur[1^p] = trie[1^p][cur[1^p]][d];
            num = num * 10 + d;
        } else if num > 0 {
            cnt[p][cur[p]] += 1;

            ans2 += cnt[1^p][cur[1^p]] * num as u32;

            // Reset and swap
            p ^= 1;
            num = 0;
            cur[0] = 1;
            cur[1] = 1;
        }
    }
    ans2 += cnt[1^p][cur[1^p]] * num as u32;

    ans2
}
