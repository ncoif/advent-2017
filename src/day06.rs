use std::collections::{HashMap, HashSet};

pub fn title() -> &'static str {
    "Day 6: Memory Reallocation"
}

pub fn answer1(input: &str) -> u32 {
    let mut banks: Vec<u32> = parse_input(input);
    let mut already_seen = HashSet::new();

    while !already_seen.contains(&banks) {
        already_seen.insert(banks.clone());
        next_banks(&mut banks);
    }

    already_seen.len() as u32
}

pub fn answer2(input: &str) -> u32 {
    let mut banks: Vec<u32> = parse_input(input);
    let mut already_seen = HashMap::new();

    let mut iteration: u32 = 0;
    while !already_seen.contains_key(&banks) {
        already_seen.insert(banks.clone(), iteration);
        next_banks(&mut banks);
        iteration += 1;
    }

    iteration - already_seen[&banks]
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(char::is_whitespace)
        .map(|n| n.parse::<u32>())
        .filter(|n| n.is_ok())
        .map(Result::unwrap) // safe because skip invalid elements
        .collect()
}

fn next_banks(bank: &mut Vec<u32>) {
    let idx_max = bank
        .iter()
        .enumerate()
        .rev() // reverse because in case of tie, we want the lowest idx
        .max_by(|(_, val_a), (_, val_b)| val_a.cmp(val_b))
        .map(|(idx, _val)| idx)
        .unwrap();

    // find the starting value
    let mut val_max = bank[idx_max];
    bank[idx_max] = 0;

    // find the starting point
    let mut idx_cur = idx_max + 1;
    if idx_cur >= bank.len() {
        idx_cur = 0;
    }

    // drain the starting value
    while val_max > 0 {
        bank[idx_cur] += 1;
        idx_cur += 1;
        if idx_cur >= bank.len() {
            idx_cur = 0;
        }
        val_max -= 1;
    }
}

#[test]
fn test_next_bank() {
    let mut banks: Vec<u32> = vec![0, 2, 7, 0];

    next_banks(&mut banks);
    assert_eq!(banks, vec![2, 4, 1, 2]);

    next_banks(&mut banks);
    assert_eq!(banks, vec![3, 1, 2, 3]);

    next_banks(&mut banks);
    assert_eq!(banks, vec![0, 2, 3, 4]);

    next_banks(&mut banks);
    assert_eq!(banks, vec![1, 3, 4, 1]);

    next_banks(&mut banks);
    assert_eq!(banks, vec![2, 4, 1, 2]);
}

#[test]
fn test_answer1() {
    let input = String::from(r#"0 2 7 0"#);

    assert_eq!(answer1(&input), 5);
}

#[test]
fn test_answer2() {
    let input = String::from(r#"0 2 7 0"#);

    assert_eq!(answer2(&input), 4);
}
