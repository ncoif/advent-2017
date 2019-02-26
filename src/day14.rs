use crate::day10;

pub fn title() -> &'static str {
    "Day 14: Disk Defragmentation"
}

fn from_char_count_ones(c: char) -> u32 {
    match c {
        '0' => 0, //0000
        '1' => 1, //0001
        '2' => 1, //0010
        '3' => 2, //0011
        '4' => 1, //0100
        '5' => 2, //0101
        '6' => 2, //0110
        '7' => 3, //0111
        '8' => 1, //1000
        '9' => 2, //1001
        'a' => 2, //1010
        'b' => 3, //1011
        'c' => 2, //1100
        'd' => 3, //1101
        'e' => 3, //1110
        'f' => 4, //1111
        _ => unreachable!(),
    }
}

pub fn answer1(input: &str) -> u32 {
    (0..128)
        .map(|i| {
            let s = format!("{}-{}", input, i);
            let hash = day10::hash(&s);
            hash.chars()
                .fold(0, |count, c| count + from_char_count_ones(c))
        })
        .sum()
}

#[test]
fn test_answer1() {
    let input = String::from("flqrgnkx");
    assert_eq!(answer1(&input), 8108);
}
