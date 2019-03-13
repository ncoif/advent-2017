use crate::common;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

pub fn title() -> &'static str {
    "Day 18: Duet"
}

#[derive(Debug, PartialEq)]
enum Register {
    Var(char),
    Val(i64),
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Register, ()> {
        let val = i64::from_str_radix(s, 10);
        match val {
            Ok(val) => Ok(Register::Val(val)),
            _ => Ok(Register::Var(common::to_char(s))),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Send(Register),
    Set(char, Register),
    Add(char, Register),
    Mul(char, Register),
    Mod(char, Register),
    Recover(char),
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
                common::to_char(words.next().unwrap()),
                Register::from_str(words.next().unwrap()).unwrap(),
            )),
            Some("add") => Ok(Instruction::Add(
                common::to_char(words.next().unwrap()),
                Register::from_str(words.next().unwrap()).unwrap(),
            )),
            Some("mul") => Ok(Instruction::Mul(
                common::to_char(words.next().unwrap()),
                Register::from_str(words.next().unwrap()).unwrap(),
            )),
            Some("mod") => Ok(Instruction::Mod(
                common::to_char(words.next().unwrap()),
                Register::from_str(words.next().unwrap()).unwrap(),
            )),
            Some("rcv") => Ok(Instruction::Recover(common::to_char(words.next().unwrap()))),
            Some("jgz") => Ok(Instruction::Jump(
                Register::from_str(words.next().unwrap()).unwrap(),
                Register::from_str(words.next().unwrap()).unwrap(),
            )),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Program<'a> {
    regs: HashMap<char, i64>,
    pc: i64,
    instructions: &'a [Instruction],
}

impl<'a> Program<'a> {
    fn new(instructions: &[Instruction], id: i64) -> Program {
        Program {
            regs: (97u8..123u8)
                .map(|x| (x as char, if x == 112 { id } else { 0 }))
                .collect(),
            pc: 0,
            instructions,
        }
    }

    fn execute(&mut self, my_queue: &mut VecDeque<i64>, other_queue: &mut VecDeque<i64>) -> bool {
        while let Some(instruction) = self.instructions.get(if self.pc < 0 {
            usize::max_value()
        } else {
            self.pc as usize
        }) {
            self.pc += 1;
            match instruction {
                Instruction::Send(rx) => {
                    other_queue.push_back(self.get_val(&rx));
                }
                Instruction::Set(rx, ry) => {
                    self.regs.insert(*rx, self.get_val(&ry));
                }
                Instruction::Add(rx, ry) => {
                    self.regs.insert(*rx, self.regs[rx] + self.get_val(&ry));
                }
                Instruction::Mul(rx, ry) => {
                    self.regs.insert(*rx, self.regs[rx] * self.get_val(&ry));
                }
                Instruction::Mod(rx, ry) => {
                    self.regs.insert(*rx, self.regs[rx] % self.get_val(&ry));
                }
                Instruction::Recover(rx) => match my_queue.pop_front() {
                    Some(val) => {
                        self.regs.insert(*rx, val);
                    }
                    None => {
                        self.pc -= 1;
                        return false;
                    }
                },
                Instruction::Jump(rx, ry) => {
                    if self.get_val(&rx) > 0 {
                        self.pc += self.get_val(&ry) - 1;
                    }
                }
            }
        }
        true
    }

    fn get_val(&self, r: &Register) -> i64 {
        match r {
            Register::Var(v) => self.regs[&v],
            Register::Val(v) => *v,
        }
    }
}

pub fn answer1(input: &str) -> i64 {
    let instructions = parse_input(input);

    let mut pc = 0;
    let mut regs: HashMap<char, i64> = (97u8..123u8).map(|x| (x as char, 0)).collect();
    let mut freq = 0;

    while let Some(instruction) = instructions.get(if pc < 0 {
        usize::max_value()
    } else {
        pc as usize
    }) {
        pc += 1;
        match instruction {
            Instruction::Send(rx) => {
                freq = get_val(&regs, rx);
            }
            Instruction::Set(rx, ry) => {
                regs.insert(*rx, get_val(&regs, ry));
            }
            Instruction::Add(rx, ry) => {
                regs.insert(*rx, regs[rx] + get_val(&regs, ry));
            }
            Instruction::Mul(rx, ry) => {
                regs.insert(*rx, regs[rx] * get_val(&regs, ry));
            }
            Instruction::Mod(rx, ry) => {
                regs.insert(*rx, regs[rx] % get_val(&regs, ry));
            }
            Instruction::Recover(_rx) => {
                return freq;
            }
            Instruction::Jump(rx, ry) => {
                if get_val(&regs, rx) > 0 {
                    pc += get_val(&regs, ry) - 1;
                }
            }
        }
    }

    freq
}

fn get_val(regs: &HashMap<char, i64>, r: &Register) -> i64 {
    match r {
        Register::Var(v) => regs[&v],
        Register::Val(v) => *v,
    }
}

pub fn answer2(input: &str) -> usize {
    let instructions = parse_input(input);

    let mut program0 = Program::new(&instructions, 0);
    let mut program1 = Program::new(&instructions, 1);
    let mut queue0 = VecDeque::new();
    let mut queue1 = VecDeque::new();

    let mut counter = 0;
    loop {
        let done = program0.execute(&mut queue1, &mut queue0);
        let len_before = queue1.len();

        let done = done || program1.execute(&mut queue0, &mut queue1);

        counter += queue1.len() - len_before;
        if done || queue1.is_empty() {
            break;
        }
    }

    counter
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
        Register::Var('a')
    );
    assert_eq!(
        Register::from_str(&"-1".to_string()).unwrap(),
        Register::Val(-1)
    );
}

#[test]
fn test_instruction_from_str() {
    let input = String::from(r#"jgz a -1"#);

    assert_eq!(
        Instruction::from_str(&input).unwrap(),
        Instruction::Jump(Register::Var('a'), Register::Val(-1))
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

#[test]
fn test_answer2() {
    let input = String::from(
        r#"
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d"#,
    );

    assert_eq!(answer2(&input), 3);
}
