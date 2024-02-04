use itertools::Itertools;

pub fn hash(s: &str) -> u32 {
    s.chars()
        .map(|c| c as u32)
        .fold(0, |acc, e| {
            ((acc + e) * 17) & 255
        })
}

pub fn run(content: &str) -> u32 {
    let mut boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];
    let find = |new_label: &str| {

    };

    content.split(',').for_each(|instruction| {
        let tokens: Vec<&str> = instruction.split_terminator(&['=', '-']).collect();

        let label = tokens[0];

        let box_id = hash(label) as usize;
        let f = boxes[box_id].iter()
            .find_position(|(l, _)| *l == label);

        let maybe_idx = f.map(|(pos, _item)| pos);

        if tokens.len() == 2 {
            let focus = str::parse(tokens[1]).unwrap();
            if let Some(i) = maybe_idx {
                boxes[box_id][i].1 = focus
            } else {
                boxes[box_id].push((label, focus));
            }
        }
        if tokens.len() == 1 {
            if let Some(i) = maybe_idx {
                boxes[box_id].remove(i);
            }
        }
    });

    let mut ans = 0;
    for (b_id, b) in boxes.iter().enumerate() {
        for (i, (_, f)) in b.iter().enumerate() {
            ans += (b_id + 1) * (i + 1) * (*f as usize);
        }
    }

    ans as u32
}

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

#[test]
pub fn test_seq() {
    assert_eq!(run("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"), 145);
}