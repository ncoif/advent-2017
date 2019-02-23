use nom::types::CompleteStr;
use nom::{map_res, named, separated_nonempty_list, tag};
use std::cmp::max;
use std::str::FromStr;

pub fn title() -> &'static str {
    "Day 11: Hex Ed"
}

named!(
    hex_parser<CompleteStr, Hex>,
    map_res!(nom::alpha, |CompleteStr(s)| Hex::from_str(s))
);

named!(
    hexes_parser<CompleteStr, Vec<Hex>>,
    separated_nonempty_list!(tag!(","), hex_parser)
);

#[derive(Debug, PartialEq)]
enum Hex {
    N,
    NE,
    NW,
    S,
    SE,
    SW,
}

impl FromStr for Hex {
    type Err = ();

    fn from_str(s: &str) -> Result<Hex, ()> {
        match s {
            "n" => Ok(Hex::N),
            "ne" => Ok(Hex::NE),
            "nw" => Ok(Hex::NW),
            "s" => Ok(Hex::S),
            "se" => Ok(Hex::SE),
            "sw" => Ok(Hex::SW),
            _ => Err(()),
        }
    }
}

// inspired from https://www.redblobgames.com/grids/hexagons/#neighbors
#[derive(Debug)]
struct CubeCoords {
    x: i32,
    y: i32,
    z: i32,
}

impl CubeCoords {
    fn distance_from_center(&self) -> i32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) / 2
    }

    fn move_hex(&mut self, direction: Hex) {
        match direction {
            Hex::N => {
                self.y += 1;
                self.z -= 1;
            }
            Hex::NE => {
                self.x += 1;
                self.z -= 1;
            }
            Hex::NW => {
                self.x -= 1;
                self.y += 1;
            }
            Hex::S => {
                self.y -= 1;
                self.z += 1;
            }
            Hex::SE => {
                self.x += 1;
                self.y -= 1;
            }
            Hex::SW => {
                self.x -= 1;
                self.z += 1;
            }
        }
    }
}

pub fn answer1(input: &str) -> i32 {
    let hexes = parse_input(input);

    let mut cur_coords = CubeCoords { x: 0, y: 0, z: 0 };
    for hex in hexes {
        cur_coords.move_hex(hex);
    }

    cur_coords.distance_from_center()
}

pub fn answer2(input: &str) -> i32 {
    let hexes = parse_input(input);

    let mut cur_coords = CubeCoords { x: 0, y: 0, z: 0 };
    let mut max_distance = 0;
    for hex in hexes {
        cur_coords.move_hex(hex);
        max_distance = max(cur_coords.distance_from_center(), max_distance);
    }

    max_distance
}

fn parse_input(input: &str) -> Vec<Hex> {
    hexes_parser(CompleteStr(input)).unwrap().1
}

#[test]
fn test_parse_input() {
    assert_eq!(
        parse_input(&"se,sw,s,nw,ne,n".to_string()),
        vec![Hex::SE, Hex::SW, Hex::S, Hex::NW, Hex::NE, Hex::N]
    );
}

#[test]
fn test_answer1() {
    assert_eq!(answer1(&"ne,ne,ne".to_string()), 3);
    assert_eq!(answer1(&"ne,ne,sw,sw".to_string()), 0);
    assert_eq!(answer1(&"ne,ne,s,s".to_string()), 2);
    assert_eq!(answer1(&"se,sw,se,sw,sw".to_string()), 3);
}
