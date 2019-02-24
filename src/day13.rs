use nom::types::CompleteStr;
use nom::{do_parse, map_res, named, tag};
use std::collections::HashMap;

pub fn title() -> &'static str {
    "Day 13: Packet Scanners"
}

named!(
    id_parser<CompleteStr, u32>,
    map_res!(nom::digit, |CompleteStr(s)| u32::from_str_radix(s, 10))
);

named!(
    firewall_parser<CompleteStr, Firewall>,
    do_parse!(
        layer: id_parser
            >> tag!(": ")
            >> depth: id_parser
            >> (Firewall {
                layer: layer,
                depth: depth
            })
    )
);

fn parse_input(input: &str) -> Vec<Firewall> {
    let lines = input.split('\n');

    lines
        .filter(|l| *l != "")
        .map(|l| firewall_parser(CompleteStr(l)).unwrap().1)
        .collect()
}

#[derive(Debug, PartialEq)]
struct Firewall {
    layer: u32,
    depth: u32,
}

pub fn answer1(input: &str) -> u32 {
    let firewalls = parse_input(&input);

    let firewalls: HashMap<u32, u32> = firewalls.iter().map(|f| (f.layer, f.depth)).collect();

    let mut severity = 0;

    for (layer, depth) in firewalls.iter() {
        let scanner_position = 2 * depth - 2;
        if layer % scanner_position == 0 {
            severity += layer * depth;
        }
    }
    severity
}

pub fn answer2(input: &str) -> u32 {
    let firewalls = parse_input(&input);
    let firewalls: HashMap<u32, u32> = firewalls.iter().map(|f| (f.layer, f.depth)).collect();

    // no need for the expansive state structure and no need to compute the back-and-forth
    // we can compute the effective position of the scanner if it was always going straight
    // and compare it to the effective position of the packet if it was always going straight
    (0..)
        .find(|delay| {
            for (layer, depth) in firewalls.iter() {
                let position = layer + delay;
                let scanner_position = 2 * depth - 2;
                if position % scanner_position == 0 {
                    return false;
                }
            }
            true
        })
        .unwrap()
}

#[test]
fn test_answer1() {
    let input = String::from(
        r#"
0: 3
1: 2
4: 4
6: 4"#,
    );

    assert_eq!(answer1(&input), 0 * 3 + 6 * 4);
}

#[test]
fn test_answer2() {
    let input = String::from(
        r#"
0: 3
1: 2
4: 4
6: 4"#,
    );

    assert_eq!(answer2(&input), 10);
}
