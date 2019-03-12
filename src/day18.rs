use crate::common;
use std::str::FromStr;

pub fn title() -> &'static str {
    "Day 18: Duet"
}

#[derive(Debug, PartialEq)]
struct Register {
    var: Option<char>,
    val: Option<i64>,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Register, ()> {
        let val = i64::from_str_radix(s, 10);
        return if val.is_ok() {
            Ok(Register {
                var: None,
                val: Some(val.unwrap()),
            })
        } else {
            let var = common::to_char(s);
            Ok(Register {
                var: Some(var),
                val: None,
            })
        };
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Send(Register),
    Set(Register, Register),
    Add(Register, Register),
    Mul(Register, Register),
    Mod(Register, Register),
    Recover(Register),
    Jump(Register, Register),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        dbg!(&s);
        let mut words = s.split(' ');
        match words.next() {
            Some("snd") => Ok(Instruction::Send(
                Register::from_str(words.next().unwrap()).unwrap(),
            )),
            Some("set") => Ok(Instruction::Set(
                Register::from_str(words.next().unwrap()).unwrap(),
                Register::from_str(words.next().unwrap()).unwrap(),
            )),
            Some("add") => Ok(Instruction::Add(
                Register::from_str(words.next().unwrap()).unwrap(),
                Register::from_str(words.next().unwrap()).unwrap(),
            )),
            Some("mul") => Ok(Instruction::Mul(
                Register::from_str(words.next().unwrap()).unwrap(),
                Register::from_str(words.next().unwrap()).unwrap(),
            )),
            Some("mod") => Ok(Instruction::Mod(
                Register::from_str(words.next().unwrap()).unwrap(),
                Register::from_str(words.next().unwrap()).unwrap(),
            )),
            Some("rcv") => Ok(Instruction::Recover(
                Register::from_str(words.next().unwrap()).unwrap(),
            )),
            Some("jgz") => Ok(Instruction::Jump(
                Register::from_str(words.next().unwrap()).unwrap(),
                Register::from_str(words.next().unwrap()).unwrap(),
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
fn test_register_from_str() {
    assert_eq!(
        Register::from_str(&"a".to_string()).unwrap(),
        Register {
            var: Some('a'),
            val: None
        }
    );
    assert_eq!(
        Register::from_str(&"-1".to_string()).unwrap(),
        Register {
            var: None,
            val: Some(-1)
        }
    );
}

#[test]
fn test_instruction_from_str() {
    let input = String::from(r#"jgz a -1"#);
    let a = Register {
        var: Some('a'),
        val: None,
    };
    let minus_one = Register {
        var: None,
        val: Some(-1),
    };

    assert_eq!(
        Instruction::from_str(&input).unwrap(),
        Instruction::Jump(a, minus_one)
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
jgz a p
set a 1
jgz a -2"#,
    );

    assert_eq!(answer1(&input), 4);
}
