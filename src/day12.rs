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
                target: target
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
    target: Vec<u32>,
}

fn neighbours(pipes: &[Pipe]) -> Vec<Vec<u32>> {
    let mut neighbours = vec![vec![]];
    for pipe in pipes {
        let mut ns: HashSet<u32> = pipe.target.clone().drain(..).collect(); // dedup
        ns.insert(pipe.source);
        neighbours.push(ns.into_iter().collect());
    }

    neighbours
}

pub fn answer1(input: &str) -> usize {
    let pipes = parse_input(&input);
    let neighbours = neighbours(&pipes);

    // https://docs.rs/pathfinding/1.1.10/pathfinding/undirected/connected_components/fn.components.html
    let groups = pathfinding::undirected::connected_components::components(&neighbours);

    // find the group which contains 0, and return it's size
    for group in groups {
        if group.contains(&0) {
            return group.len();
        }
    }

    unreachable!();
}

pub fn answer2(input: &str) -> usize {
    let pipes = parse_input(&input);
    let neighbours = neighbours(&pipes);

    // https://docs.rs/pathfinding/1.1.10/pathfinding/undirected/connected_components/fn.components.html
    let groups = pathfinding::undirected::connected_components::components(&neighbours);

    groups.len()
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
                source: 0,
                target: vec![2]
            },
            Pipe {
                source: 2,
                target: vec![0, 3, 4]
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

#[test]
fn test_answer2() {
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

    assert_eq!(answer2(&input), 2);
}
