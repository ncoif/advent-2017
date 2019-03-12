use crate::common;
use std::collections::{HashMap, VecDeque};
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
    fn var(&self) -> char {
        self.var.unwrap()
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
                    other_queue.push_back(self.get(&rx));
                }
                Instruction::Set(rx, ry) => {
                    self.regs.insert(rx.var(), self.get(&ry));
                }
                Instruction::Add(rx, ry) => {
                    self.regs.insert(rx.var(), self.get(&rx) + self.get(&ry));
                }
                Instruction::Mul(rx, ry) => {
                    self.regs.insert(rx.var(), self.get(&rx) * self.get(&ry));
                }
                Instruction::Mod(rx, ry) => {
                    self.regs.insert(rx.var(), self.get(&rx) % self.get(&ry));
                }
                Instruction::Recover(rx) => match my_queue.pop_front() {
                    Some(val) => {
                        self.regs.insert(rx.var(), val);
                    }
                    None => {
                        self.pc -= 1;
                        return false;
                    }
                },
                Instruction::Jump(rx, ry) => {
                    if self.get(&rx) > 0 {
                        self.pc += self.get(&ry) - 1;
                    }
                }
            }
        }
        true
    }

    fn get(&self, r: &Register) -> i64 {
        if r.val.is_some() {
            r.val.unwrap()
        } else {
            self.regs[&r.var.unwrap()]
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
                freq = get(&regs, rx);
            }
            Instruction::Set(rx, ry) => {
                regs.insert(rx.var(), get(&regs, ry));
            }
            Instruction::Add(rx, ry) => {
                regs.insert(rx.var(), get(&regs, rx) + get(&regs, ry));
            }
            Instruction::Mul(rx, ry) => {
                regs.insert(rx.var(), get(&regs, rx) * get(&regs, ry));
            }
            Instruction::Mod(rx, ry) => {
                regs.insert(rx.var(), get(&regs, rx) % get(&regs, ry));
            }
            Instruction::Recover(_rx) => {
                return freq;
            }
            Instruction::Jump(rx, ry) => {
                if get(&regs, rx) > 0 {
                    pc += get(&regs, ry) - 1;
                }
            }
        }
    }

    freq
}

fn get(regs: &HashMap<char, i64>, r: &Register) -> i64 {
    if r.val.is_some() {
        r.val.unwrap()
    } else {
        regs[&r.var.unwrap()]
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
