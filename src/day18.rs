use crate::common;
use std::collections::HashMap;
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

        if val.is_ok() {
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
        }
    }
}

impl Register {
    fn get(&self, s: &State) -> i64 {
        if self.val.is_some() {
            self.val.unwrap()
        } else {
            s.regs[&self.var.unwrap()]
        }
    }

    fn var(&self) -> char {
        self.var.unwrap()
    }

    fn initialize(&self, regs: &mut HashMap<char, i64>) {
        if self.var.is_some() {
            regs.insert(self.var.unwrap(), 0);
        }
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

impl Instruction {
    fn apply(&self, s: &mut State) -> Option<i64> {
        match self {
            Instruction::Send(rx) => {
                s.last_freq = Some(rx.get(s));
                s.cur += 1;
                None
            }
            Instruction::Set(rx, ry) => {
                s.regs.insert(rx.var(), ry.get(s));
                s.cur += 1;
                None
            }
            Instruction::Add(rx, ry) => {
                s.regs.insert(rx.var(), rx.get(s) + ry.get(s));
                s.cur += 1;
                None
            }
            Instruction::Mul(rx, ry) => {
                s.regs.insert(rx.var(), rx.get(s) * ry.get(s));
                s.cur += 1;
                None
            }
            Instruction::Mod(rx, ry) => {
                s.regs.insert(rx.var(), rx.get(s) % ry.get(s));
                s.cur += 1;
                None
            }
            Instruction::Recover(rx) => {
                s.cur += 1;
                if rx.get(s) != 0 && s.last_freq.is_some() {
                    Some(s.last_freq.unwrap())
                } else {
                    None
                }
            }
            Instruction::Jump(rx, ry) => {
                if rx.get(s) != 0 {
                    s.cur += ry.get(s) as isize;
                } else {
                    s.cur += 1;
                }
                None
            }
        }
    }
}

#[derive(Debug)]
struct State {
    last_freq: Option<i64>,
    regs: HashMap<char, i64>,
    cur: isize,
}

pub fn answer1(input: &str) -> i64 {
    let instructions = parse_input(input);
    let registers = initialize(&instructions);

    let mut state = State {
        last_freq: None,
        regs: registers,
        cur: 0,
    };

    let mut result = None;
    while state.cur >= 0 && state.cur <= instructions.len() as isize && result.is_none() {
        let instruction = &instructions[state.cur as usize];
        result = instruction.apply(&mut state);
    }

    result.unwrap()
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let lines = input.split('\n');
    let instructions: Vec<Instruction> = lines
        .filter(|l| *l != "")
        .map(|l| Instruction::from_str(l).unwrap())
        .collect();

    instructions
}

fn initialize(instructions: &[Instruction]) -> HashMap<char, i64> {
    let mut registers = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Send(rx) => {
                rx.initialize(&mut registers);
            }
            Instruction::Set(rx, ry) => {
                rx.initialize(&mut registers);
                ry.initialize(&mut registers);
            }
            Instruction::Add(rx, ry) => {
                rx.initialize(&mut registers);
                ry.initialize(&mut registers);
            }
            Instruction::Mul(rx, ry) => {
                rx.initialize(&mut registers);
                ry.initialize(&mut registers);
            }
            Instruction::Mod(rx, ry) => {
                rx.initialize(&mut registers);
                ry.initialize(&mut registers);
            }
            Instruction::Recover(rx) => {
                rx.initialize(&mut registers);
            }
            Instruction::Jump(rx, ry) => {
                rx.initialize(&mut registers);
                ry.initialize(&mut registers);
            }
        }
    }
    registers
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
set a 1
jgz a -2"#,
    );

    assert_eq!(answer1(&input), 4);
}
