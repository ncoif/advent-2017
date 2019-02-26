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
    list: Vec<usize>,
    current_position: usize,
    skip_size: usize, // increment by 1 for everty .next()
}

impl Hash {
    fn next(&mut self, length: usize) {
        let size = self.list.len();

        // find the range of indices to reverse
        let range: Vec<usize> = (self.current_position..self.current_position + length)
            .map(|idx| idx % size)
            .collect();
        let reverse_range = range.clone().into_iter().rev();

        let mut new_list = self.list.clone();
        for (idx, idx_rev) in range.into_iter().zip(reverse_range) {
            new_list[idx] = self.list[idx_rev];
        }

        // increment and clean-up
        self.current_position = (self.current_position + length + self.skip_size) % size;
        self.list = new_list;
        self.skip_size += 1;
    }

    fn dense_hash(&self) -> String {
        let dense_size = self.list.len() / 16;

        (0..dense_size)
            .map(|i| {
                (0..16)
                    .map(|ii| i * 16 + ii)
                    .fold(0 as usize, |acc, index| acc ^ self.list[index])
            })
            .map(|c| format!("{:01$x}", c, 2))
            .collect()
    }
}

pub fn answer1(size: usize, input: &str) -> usize {
    let lengths = parse_input(&input);

    let mut hash = Hash {
        list: (0..size).collect(),
        current_position: 0,
        skip_size: 0,
    };

    for length in lengths {
        hash.next(length);
    }

    hash.list[0] * hash.list[1]
}

pub fn hash(input: &str) -> String {
    let mut lengths = input
        .trim()
        .chars()
        .map(|c| c as u8)
        .map(|c| c as usize)
        .collect::<Vec<usize>>();
    lengths.extend(&[17, 31, 73, 47, 23]);

    let mut hash = Hash {
        list: (0..256).collect(),
        current_position: 0,
        skip_size: 0,
    };

    for _ in 0..64 {
        for length in lengths.clone() {
            hash.next(length);
        }
    }

    hash.dense_hash()
}

pub fn answer2(input: &str) -> String {
    hash(input)
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

    let expected = Hash {
        list: vec![4, 3, 0, 1, 2],
        current_position: 1,
        skip_size: 3,
    };
    init.next(1);
    assert_eq!(init, expected);
}

#[test]
fn test_answer1() {
    assert_eq!(answer1(5, &"3,4,1,5".to_string()), 3 * 4);
}

#[test]
fn test_answer2() {
    assert_eq!(
        answer2(&"".to_string()),
        "a2582a3a0e66e6e86e3812dcb672a272".to_string()
    );
    assert_eq!(
        answer2(&"AoC 2017".to_string()),
        "33efeb34ea91902bb2f59c9920caa6cd".to_string()
    );
    assert_eq!(
        answer2(&"1,2,3".to_string()),
        "3efbe78a8d82f29979031a4aa0b16a9d".to_string()
    );
    assert_eq!(
        answer2(&"1,2,4".to_string()),
        "63960835bcdc130f0b66d7ff4f6a5a8e".to_string()
    );
}
