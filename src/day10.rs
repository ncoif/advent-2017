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
    list: Vec<(usize, usize)>, // (prev, next)
    list_size: usize,
    list_start: usize,
    current_position: usize,
    skip_size: usize,
}

impl Hash {
    fn next(&mut self, next_length: usize) {
        dbg!("start_next");

        let first = self.list[self.list_start].0;
        let mut last = self.list[self.list_start].1;
        for _ in 1..next_length {
            last = self.list[last].1
        }
        dbg!((first, last));

        // reverse the sub-list
        let mut cur_idx = self.list_start;
        for idx in 0..next_length {
            let cur = self.list[cur_idx];
            // dbg!(&cur);
            // dbg!(&self.list);

            if idx == 0 {
                // if first element, also update the edge
                self.list[cur_idx].1 = last;
                self.list[cur_idx].0 = cur.1;
                self.list[last].0 = cur_idx;
                dbg!(("first", &cur, "cur_idx", cur_idx));
            } else if idx == next_length - 1 {
                // if last element, also update the edge
                self.list[cur_idx].1 = cur.0;
                self.list[cur_idx].0 = first;
                self.list[first].1 = cur_idx;
                self.list_start = cur_idx + self.skip_size;
                dbg!(("last", &cur, "cur_idx", cur_idx));
            } else {
                self.list[cur_idx].1 = cur.0;
                self.list[cur_idx].0 = cur.1;
            }

            cur_idx = cur.1;
        }

        // and adjust the counters
        self.current_position = (self.current_position + next_length + self.skip_size) % self.list_size;
        self.skip_size += 1;
    }
}

pub fn answer1(size: usize, input: &str) -> usize {
    let lengths = parse_input(&input);

    let list = (0..size)
        .map(|idx| {
            let prev = (size + idx -1) % size;
            let next = (idx + 1) % size;
            (prev, next)
        })
        .collect();

    dbg!(&list);

    let mut hash = Hash {
        list: list,
        list_size: size,
        list_start: 0,
        current_position: 0,
        skip_size: 0,
    };

    for length in lengths {
        hash.next(length);
    }

    hash.list_start * hash.list[hash.list_start].1
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
        list: vec![(4, 1), (0, 2), (1, 3), (2, 4), (3, 0)],
        list_size: 5,
        list_start: 0,
        current_position: 0,
        skip_size: 0,
    };
    let expected = Hash {
        list: vec![(1, 3), (2, 0), (4, 1), (0, 4), (3, 2)],
        list_size: 5,
        list_start: 2,
        current_position: 3,
        skip_size: 1,
    };

    init.next(3);
    assert_eq!(init, expected);

    let expected = Hash {
        list: vec![(3, 1), (0, 2), (1, 4), (4, 0), (2, 3)],
        list_size: 5,
        list_start: 4,
        current_position: 3,
        skip_size: 2,
    };
    init.next(4);
    assert_eq!(init, expected);

    let expected = Hash {
        list: vec![(3, 1), (0, 2), (1, 4), (4, 0), (2, 3)],
        list_size: 5,
        list_start: 3,
        current_position: 1,
        skip_size: 3,
    };
    init.next(1);
    assert_eq!(init, expected);

    let expected = Hash {
        list: vec![(1,3),(2,0),(4,1),(0,4),(3,2)],
        list_size: 5,
        list_start: 3,
        current_position: 1,
        skip_size: 3,
    };

}

#[test]
fn test_next_length_list_size() {
    let mut init = Hash {
        list: vec![(3,1),(0,2),(1,4),(4,0),(2,3)],
        list_size: 5,
        list_start: 4,
        current_position: 2,
        skip_size: 3,
    };
    let expected = Hash {
        list: vec![(1,3),(2,0),(4,1),(0,4),(3,2)],
        list_size: 5,
        list_start: 3,
        current_position: 4,
        skip_size: 4,
    };

    init.next(5);
    assert_eq!(init, expected);
}

#[test]
fn test_answer1() {
    // assert_eq!(answer1(5, &"3,4,1,5".to_string()), 3 * 4);
}
