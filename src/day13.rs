use nom::types::CompleteStr;
use nom::{do_parse, map_res, named, tag};

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

fn max_layer(firewalls: &[Firewall]) -> usize {
    // quick and dirty, assuming the layer are sorted in the input
    firewalls.last().unwrap().layer as usize
}

fn move_security_positions(
    layer_security_position: &mut [(bool, u32)],
    layer_size: &[u32],
    max_layer: usize,
) {
    (0..=max_layer)
        .filter(|&layer| layer_size[layer] > 0)
        .for_each(|layer| {
            // do I need to change the direction?
            if layer_security_position[layer].0
                && layer_security_position[layer].1 == layer_size[layer] - 1
            {
                layer_security_position[layer].0 = false;
            } else if !layer_security_position[layer].0 && layer_security_position[layer].1 == 0 {
                layer_security_position[layer].0 = true;
            }

            // move the security positions in the correct direction
            if layer_security_position[layer].0 {
                layer_security_position[layer].1 += 1;
            } else {
                layer_security_position[layer].1 -= 1;
            }
        });
}

fn run_with_delay(firewalls: &[Firewall], delay: u32) -> bool {
    if delay % 1000 == 0 {
        dbg!(delay);
    }

    let max_layer = max_layer(&firewalls);

    let mut layer_size: Vec<u32> = vec![0; max_layer + 1];
    let mut layer_security_position: Vec<(bool, u32)> = vec![(true, 0); max_layer + 1]; // (direction, position)
    for firewall in firewalls.iter() {
        layer_size[firewall.layer as usize] = firewall.depth;
    }

    // wait delay
    (0..delay).for_each(|_delay| {
        // move the security positions
        move_security_positions(&mut layer_security_position, &layer_size, max_layer);
    });

    let mut is_caught = false;
    (0..=max_layer).for_each(|layer_pos| {
        // check if I'm caught
        if layer_security_position[layer_pos].1 == 0 && layer_size[layer_pos] != 0 {
            is_caught = true;
        }

        // move the security positions
        move_security_positions(&mut layer_security_position, &layer_size, max_layer);
    });

    is_caught
}

pub fn answer1(input: &str) -> u32 {
    let firewalls = parse_input(&input);
    let max_layer = max_layer(&firewalls);

    let mut layer_size: Vec<u32> = vec![0; max_layer + 1];
    let mut layer_security_position: Vec<(bool, u32)> = vec![(true, 0); max_layer + 1]; // (direction, position)
    for firewall in firewalls.iter() {
        layer_size[firewall.layer as usize] = firewall.depth;
    }

    let mut severity = 0;
    (0..=max_layer).for_each(|layer_pos| {
        // check if I'm caught
        if layer_security_position[layer_pos].1 == 0 && layer_size[layer_pos] != 0 {
            severity += layer_pos as u32 * layer_size[layer_pos]; // caught
        }

        // move the security positions
        move_security_positions(&mut layer_security_position, &layer_size, max_layer);
    });

    severity
}

pub fn answer2(input: &str) -> u32 {
    let firewalls = parse_input(&input);

    let mut delay = 0;
    let mut is_caught = run_with_delay(&firewalls, delay);
    while is_caught {
        delay += 1;
        is_caught = run_with_delay(&firewalls, delay);
    }

    delay
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
