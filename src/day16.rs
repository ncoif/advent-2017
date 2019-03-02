use std::mem;
use std::str::FromStr;

pub fn title() -> &'static str {
    "Day 16: Permutation Promenade"
}

#[derive(Debug, PartialEq)]
enum Dance {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Dance {
    type Err = ();

    fn from_str(s: &str) -> Result<Dance, ()> {
        let bytes = s.as_bytes();
        match bytes[0] {
            b's' => Ok(Dance::parse_spin(s)),
            b'x' => Ok(Dance::parse_exchange(s)),
            b'p' => Ok(Dance::parse_partner(s)),
            _ => Err(()),
        }
    }
}

impl Dance {
    fn parse_spin(s: &str) -> Dance {
        let parse = s.get(1..).unwrap();
        Dance::Spin(usize::from_str_radix(parse, 10).unwrap())
    }

    fn parse_exchange(s: &str) -> Dance {
        let mut parse = s.get(1..).unwrap().split('/');
        let idx1 = parse.next().unwrap();
        let idx2 = parse.next().unwrap();
        Dance::Exchange(
            usize::from_str_radix(idx1, 10).unwrap(),
            usize::from_str_radix(idx2, 10).unwrap(),
        )
    }

    fn parse_partner(s: &str) -> Dance {
        let mut parse = s.get(1..).unwrap().split('/');
        let p1 = parse.next().unwrap();
        let p2 = parse.next().unwrap();
        Dance::Partner(p1.as_bytes()[0] as char, p2.as_bytes()[0] as char)
    }

    fn apply(&self, programs: &mut Vec<char>) {
        match self {
            Dance::Spin(idx) => {
                let (start, end) = programs.split_at(programs.len() - *idx);
                let mut new = end.to_vec();
                new.extend_from_slice(start);
                mem::replace(programs, new);
            }
            Dance::Exchange(idx1, idx2) => {
                programs.swap(*idx1, *idx2);
            }
            Dance::Partner(p1, p2) => {
                let idx1 = programs.iter().position(|c| c == p1).unwrap();
                let idx2 = programs.iter().position(|c| c == p2).unwrap();
                programs.swap(idx1, idx2);
            }
        }
    }
}

pub fn answer1(input: &str, size: usize) -> String {
    let dances = parse_input(&input);
    let mut programs = vec_chars(size);

    for dance in dances {
        dance.apply(&mut programs);
    }
    programs.iter().collect()
}

fn parse_input(input: &str) -> Vec<Dance> {
    let mut lines = input.split('\n').filter(|l| *l != "");
    let line = lines.next().unwrap();
    let moves = line.split(',');

    moves.map(|m| Dance::from_str(m).unwrap()).collect()
}

fn vec_chars(size: usize) -> Vec<char> {
    (0..size)
        .map(|idx| b'a' + idx as u8)
        .map(|e| e as char)
        .collect()
}

#[test]
fn test_parse_input() {
    assert_eq!(
        parse_input("s1,x3/4,pe/b"),
        vec![
            Dance::Spin(1),
            Dance::Exchange(3, 4),
            Dance::Partner('e', 'b')
        ]
    );
}

#[test]
fn test_vec_chars() {
    assert_eq!(vec_chars(5), vec!['a', 'b', 'c', 'd', 'e']);
}

#[test]
fn test_answer1() {
    assert_eq!(answer1("s1,x3/4,pe/b", 5), "baedc");
}
