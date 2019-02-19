use nom::types::CompleteStr;
use nom::{char, do_parse, map, map_res, named, opt, recognize, tag, take_until, tuple};
use std::collections::HashMap;
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

impl Direction {
    fn apply(&self, reg: i32, val: i32) -> i32 {
        match self {
            Direction::Inc => reg + val,
            Direction::Dec => reg - val,
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

impl Predicate {
    fn validate(&self, reg: i32, val: i32) -> bool {
        match self {
            Predicate::Eq => reg == val,
            Predicate::NotEq => reg != val,
            Predicate::Greater => reg > val,
            Predicate::Smaller => reg < val,
            Predicate::GreaterEq => reg >= val,
            Predicate::SmallerEq => reg <= val,
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

    // initialise all the registers
    let mut registers: HashMap<String, i32> = HashMap::new();
    for instruction in &instructions {
        registers.insert(instruction.reg_inc.clone(), 0);
        registers.insert(instruction.reg_pred.clone(), 0);
    }

    // run the program
    for instruction in &instructions {
        if instruction
            .pred
            .validate(registers[&instruction.reg_pred], instruction.pred_val)
        {
            registers.insert(
                instruction.reg_inc.clone(),
                instruction
                    .inc
                    .apply(registers[&instruction.reg_inc], instruction.inc_val),
            );
        }
    }

    // find the max value
    registers
        .iter()
        .max_by_key(|(_k, v)| *v)
        .map(|(_k, v)| *v)
        .unwrap()
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

#[test]
fn test_answer1() {
    let input = String::from(
        r#"
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"#,
    );

    assert_eq!(answer1(&input), 1);
}
