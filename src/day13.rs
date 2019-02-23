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

#[derive(Debug, PartialEq, Clone)]
struct State {
    positions: Vec<(bool, u32)>, // (direction, position)
    sizes: Vec<u32>,
    max_layer: usize,
}

impl State {
    fn new(firewalls: &[Firewall]) -> Self {
        let max_layer = firewalls.last().unwrap().layer as usize;
        let mut layer_size: Vec<u32> = vec![0; max_layer + 1];
        for firewall in firewalls.iter() {
            layer_size[firewall.layer as usize] = firewall.depth;
        }

        State {
            positions: vec![(true, 0); max_layer + 1],
            sizes: layer_size,
            max_layer,
        }
    }

    fn next(&mut self) {
        (0..=self.max_layer).for_each(|layer| {
            if self.sizes[layer] > 0 {
                // do I need to change the direction?
                if self.positions[layer].0 && self.positions[layer].1 == self.sizes[layer] - 1 {
                    self.positions[layer].0 = false;
                } else if !self.positions[layer].0 && self.positions[layer].1 == 0 {
                    self.positions[layer].0 = true;
                }

                // move the security positions in the correct direction
                if self.positions[layer].0 {
                    self.positions[layer].1 += 1;
                } else {
                    self.positions[layer].1 -= 1;
                }
            }
        });
    }

    fn is_caught(&self, layer: usize) -> bool {
        self.sizes[layer] != 0 && self.positions[layer].1 == 0
    }

    fn layer_cost(&self, layer: usize) -> u32 {
        layer as u32 * self.sizes[layer]
    }
}

pub fn answer1(input: &str) -> u32 {
    let firewalls = parse_input(&input);
    let mut state = State::new(&firewalls);

    let mut severity = 0;
    (0..=state.max_layer).for_each(|layer_pos| {
        if state.is_caught(layer_pos) {
            severity += state.layer_cost(layer_pos);
        }
        state.next();
    });

    severity
}

pub fn answer2(input: &str) -> u32 {
    let firewalls = parse_input(&input);
    let firewalls: HashMap<u32, u32> = firewalls.iter().map(|f| (f.layer, f.depth)).collect();

    // no need for the expansive state structure and no need to compute the back-and-forth
    // we can compute the effective position of the scanner if it was always going straight
    // and compare it to the effective position of the packet if it was always going straight
    (0..)
        .filter(|delay| {
            for (layer, depth) in firewalls.iter() {
                let position = layer + delay;
                let scanner_position = 2 * depth - 2;
                if position % scanner_position == 0 {
                    return false;
                }
            }
            true
        })
        .next()
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