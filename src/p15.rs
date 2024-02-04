pub fn hash(s: &str) -> u32 {
    s.chars()
        .map(|c| c as u32)
        .fold(0, |acc, e| {
            ((acc + e) * 17) & 255
        })
}

pub fn run(content: &str) -> u32 { content.split(',').map(hash).sum() }

#[test]
pub fn test_examples() {
    assert_eq!(hash("HASH"), 52);

    assert_eq!(hash("rn=1"), 30);
    assert_eq!(hash("cm-"), 253);
    assert_eq!(hash("qp=3"), 97);
    assert_eq!(hash("cm=2"), 47);
    assert_eq!(hash("qp-"), 14);
    assert_eq!(hash("pc=4"), 180);
    assert_eq!(hash("ot=9"), 9);
    assert_eq!(hash("ab=5"), 197);
    assert_eq!(hash("pc-"), 48);
    assert_eq!(hash("pc=6"), 214);
    assert_eq!(hash("ot=7"), 231);

    assert_eq!(hash("rn"), 0);
}