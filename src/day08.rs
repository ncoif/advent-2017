use nom::types::CompleteStr;
use nom::{char, do_parse, map, map_res, named, opt, recognize, tag, take_until, tuple};
use std::str::FromStr;

pub fn title() -> &'static str {
    "Day 08: I Heard You Like Register"
}

#[derive(Debug, PartialEq)]
enum Direction {
    Inc,
    Dec,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Direction, ()> {
        match s {
            "inc" => Ok(Direction::Inc),
            "dec" => Ok(Direction::Dec),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Predicate {
    Eq,
    NotEq,
    Greater,
    Smaller,
    GreaterEq,
    SmallerEq,
}

impl FromStr for Predicate {
    type Err = ();

    fn from_str(s: &str) -> Result<Predicate, ()> {
        match s {
            "==" => Ok(Predicate::Eq),
            "!=" => Ok(Predicate::NotEq),
            ">" => Ok(Predicate::Greater),
            "<" => Ok(Predicate::Smaller),
            ">=" => Ok(Predicate::GreaterEq),
            "<=" => Ok(Predicate::SmallerEq),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    reg_inc: String,
    inc: Direction,
    inc_val: i32,
    reg_pred: String,
    pred: Predicate,
    pred_val: i32,
}

named!(
    reg_parser<CompleteStr, String>,
    map!(nom::alpha, |CompleteStr(s)| String::from(s))
);

named!(
    val_parser<CompleteStr, i32>,
    map_res!(recognize!(tuple!(opt!(char!('-')), nom::digit)), |CompleteStr(s)| i32::from_str_radix(s, 10))
);

named!(
    direction_parser<CompleteStr, Direction>,
    map_res!(take_until!(" "), |CompleteStr(s)| Direction::from_str(s))
);

named!(
    predicate_parser<CompleteStr, Predicate>,
    map_res!(take_until!(" "), |CompleteStr(s)| Predicate::from_str(s))
);

named!(
    instruction_parser<CompleteStr, Instruction>,
    do_parse!(
        reg_inc: reg_parser
            >> tag!(" ")
            >> inc: direction_parser
            >> tag!(" ")
            >> inc_val: val_parser
            >> tag!(" if ")
            >> reg_pred: reg_parser
            >> tag!(" ")
            >> pred: predicate_parser
            >> tag!(" ")
            >> pred_val: val_parser
            >> (Instruction {
                reg_inc: reg_inc.to_string(),
                inc: inc,
                inc_val: inc_val,
                reg_pred: reg_pred,
                pred: pred,
                pred_val: pred_val
            })
    )
);

pub fn answer1(input: &str) -> i32 {
    let instructions = parse_input(&input);

    dbg!(instructions);

    0
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let lines = input.split('\n');

    lines
        .filter(|l| *l != "")
        .map(|l| instruction_parser(CompleteStr(l)).unwrap().1)
        .collect()
}

#[test]
fn parse_simple_instruction() {
    assert_eq!(
        instruction_parser(CompleteStr::from("b inc 5 if a > 1")),
        Ok((
            CompleteStr::from(""),
            Instruction {
                reg_inc: "b".to_string(),
                inc: Direction::Inc,
                inc_val: 5,
                reg_pred: "a".to_string(),
                pred: Predicate::Greater,
                pred_val: 1
            }
        ))
    );

    assert_eq!(
        instruction_parser(CompleteStr::from("bad dec -5 if sdqa >= -111")),
        Ok((
            CompleteStr::from(""),
            Instruction {
                reg_inc: "bad".to_string(),
                inc: Direction::Dec,
                inc_val: -5,
                reg_pred: "sdqa".to_string(),
                pred: Predicate::GreaterEq,
                pred_val: -111
            }
        ))
    );
}
