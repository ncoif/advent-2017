use nom::types::CompleteStr;
use nom::{map_res, named, separated_nonempty_list, tag};

pub fn title() -> &'static str {
    "Day 10: Knot Hash"
}

named!(
    u32_parser<CompleteStr, u32>,
    map_res!(nom::digit, |CompleteStr(s)| u32::from_str_radix(s, 10))
);

named!(
    list_parser<CompleteStr, Vec<u32>>,
    separated_nonempty_list!(tag!(","), u32_parser)
);

pub fn answer1(size: usize, input: &str) -> u64 {
    let lengths = parse_input(&input);

    0
}

fn parse_input(input: &str) -> Vec<u32> {
    list_parser(CompleteStr(input)).unwrap().1
}

#[test]
fn test_parse_input() {
    assert_eq!(parse_input(&"3,4,1,5".to_string()), vec![3, 4, 1, 5]);
}
