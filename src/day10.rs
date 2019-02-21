use nom::types::CompleteStr;
use nom::{map_res, named, separated_nonempty_list, tag};

pub fn title() -> &'static str {
    "Day 10: Knot Hash"
}

named!(
    usize_parser<CompleteStr, usize>,
    map_res!(nom::digit, |CompleteStr(s)| usize::from_str_radix(s, 10))
);

named!(
    list_parser<CompleteStr, Vec<usize>>,
    separated_nonempty_list!(tag!(","), usize_parser)
);

#[derive(Debug, PartialEq)]
struct Hash {
    list: Vec<u32>,
    current_position: usize,
    skip_size: usize,
}

impl Hash {
    fn next(&mut self, next_length: usize) {
        let slice = &mut self.list[self.current_position..next_length];
        slice.reverse();
    }
}

pub fn answer1(size: usize, input: &str) -> u32 {
    let lengths = parse_input(&input);

    let mut hash = Hash {
        list: (0..=size).map(|idx| idx as u32).collect(),
        current_position: 0,
        skip_size: 0,
    };

    for length in lengths {
        hash.next(length);
    }

    hash.list[0] * hash.list[1]
}

fn parse_input(input: &str) -> Vec<usize> {
    list_parser(CompleteStr(input)).unwrap().1
}

#[test]
fn test_parse_input() {
    assert_eq!(parse_input(&"3,4,1,5".to_string()), vec![3, 4, 1, 5]);
}

#[test]
fn test_next() {
    let mut init = Hash {
        list: vec![0, 1, 2, 3, 4],
        current_position: 0,
        skip_size: 0,
    };
    let expected = Hash {
        list: vec![2, 1, 0, 3, 4],
        current_position: 3,
        skip_size: 1,
    };

    init.next(3);
    assert_eq!(init, expected);

    let expected = Hash {
        list: vec![4, 3, 0, 1, 2],
        current_position: 3,
        skip_size: 2,
    };
    init.next(4);
    assert_eq!(init, expected);
}

#[test]
fn test_answer1() {
    assert_eq!(answer1(5, &"3,4,1,5".to_string()), 3 * 4);
}
