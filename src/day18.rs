use crate::common;
use std::str::FromStr;

pub fn title() -> &'static str {
    "Day 18: Duet"
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Send(char),
    Set(char, i32),
    Add(char, i32),
    Mul(char, char),
    Mod(char, char),
    Recover(char),
    Jump(char, i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        let mut words = s.split(' ');
        match words.next() {
            Some("snd") => Ok(Instruction::Send(common::to_char(words.next().unwrap()))),
            Some("set") => Ok(Instruction::Set(
                common::to_char(words.next().unwrap()),
                i32::from_str_radix(words.next().unwrap(), 10).unwrap(),
            )),
            Some("add") => Ok(Instruction::Add(
                common::to_char(words.next().unwrap()),
                i32::from_str_radix(words.next().unwrap(), 10).unwrap(),
            )),
            Some("mul") => Ok(Instruction::Mul(
                common::to_char(words.next().unwrap()),
                common::to_char(words.next().unwrap()),
            )),
            Some("mod") => Ok(Instruction::Mod(
                common::to_char(words.next().unwrap()),
                common::to_char(words.next().unwrap()),
            )),
            Some("rcv") => Ok(Instruction::Recover(common::to_char(words.next().unwrap()))),
            Some("jgz") => Ok(Instruction::Jump(
                common::to_char(words.next().unwrap()),
                i32::from_str_radix(words.next().unwrap(), 10).unwrap(),
            )),
            _ => Err(()),
        }
    }
}

pub fn answer1(input: &str) -> u32 {
    let instructions = parse_input(input);
    dbg!(&instructions);
    0
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let lines = input.split('\n');
    let instructions: Vec<Instruction> = lines
        .filter(|l| *l != "")
        .map(|l| Instruction::from_str(l).unwrap())
        .collect();

    instructions
}

#[test]
fn test_instruction_from_str() {
    let input = String::from(r#"jgz a -1"#);

    assert_eq!(
        Instruction::from_str(&input).unwrap(),
        Instruction::Jump('a', -1)
    );
}

#[test]
fn test_answer1() {
    let input = String::from(
        r#"
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"#,
    );

    assert_eq!(answer1(&input), 4);
}
