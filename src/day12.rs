use maplit::hashset;
use nom::types::CompleteStr;
use nom::{do_parse, map_res, named, separated_nonempty_list, tag};
use std::collections::HashSet;

pub fn title() -> &'static str {
    "Day 12: Digital Plumber"
}

named!(
    id_parser<CompleteStr, u32>,
    map_res!(nom::digit, |CompleteStr(s)| u32::from_str_radix(s, 10))
);

named!(
    target_parser<CompleteStr, Vec<u32>>,
    separated_nonempty_list!(tag!(", "), id_parser)
);

named!(
    pipe_parser<CompleteStr, Pipe>,
    do_parse!(
        source: id_parser
            >> tag!(" <-> ")
            >> target: target_parser
            >> (Pipe {
                source: source,
                target: target.into_iter().collect()
            })
    )
);

fn parse_input(input: &str) -> Vec<Pipe> {
    let lines = input.split('\n');

    lines
        .filter(|l| *l != "")
        .map(|l| pipe_parser(CompleteStr(l)).unwrap().1)
        .collect()
}

#[derive(Debug, PartialEq)]
struct Pipe {
    source: u32,
    target: HashSet<u32>,
}

pub fn answer1(input: &str) -> i32 {
    let pipes = parse_input(&input);
    dbg!(&pipes);
    0
}

#[test]
fn test_parse_input() {
    let input = String::from(
        r#"
0 <-> 2
2 <-> 0, 3, 4"#,
    );

    assert_eq!(
        parse_input(&input),
        vec![
            Pipe {
                source: 1,
                target: hashset!(2)
            },
            Pipe {
                source: 2,
                target: hashset!(0, 3, 4)
            }
        ]
    );
}

#[test]
fn test_answer1() {
    let input = String::from(
        r#"
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"#,
    );

    assert_eq!(answer1(&input), 6);
}
