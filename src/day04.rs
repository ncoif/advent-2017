use std::collections::HashSet;
use std::iter::FromIterator;

pub fn title() -> &'static str {
    "Day 4: High-Entropy Passphrases"
}

pub fn answer1(input: &str) -> u32 {
    let passphrases: Vec<Vec<String>> = parse_input(input);

    passphrases
        .iter()
        .fold(0, |count, p| if is_valid(&p) { count + 1 } else { count })
}

pub fn answer2(input: &str) -> u32 {
    let passphrases: Vec<Vec<String>> = parse_input(input);

    passphrases
        .iter()
        .fold(0, |count, p| if is_valid_2(&p) { count + 1 } else { count })
}

fn parse_input(input: &str) -> Vec<Vec<String>> {
    let passphrases = input.split('\n');
    passphrases
        .filter(|l| *l != "")
        .map(|l| l.split(char::is_whitespace).map(String::from).collect())
        .collect()
}

fn is_valid(passphrase: &[String]) -> bool {
    let deduplicates: HashSet<_> = passphrase.iter().collect();
    deduplicates.len() == passphrase.len()
}

fn is_valid_2(passphrase: &[String]) -> bool {
    let deduplicates: HashSet<String> = passphrase
        .iter()
        .map(|w| {
            let mut chars: Vec<char> = w.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            String::from_iter(chars)
        })
        .collect();
    deduplicates.len() == passphrase.len()
}

#[test]
fn test_answer1() {
    let input = String::from(
        r#"
aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa"#,
    );

    assert_eq!(answer1(&input), 2);
}

#[test]
fn test_is_valid() {
    assert_eq!(
        is_valid(&vec![
            "aa".to_string(),
            "bb".to_string(),
            "cc".to_string(),
            "dd".to_string(),
            "ee".to_string()
        ]),
        true
    );
    assert_eq!(
        is_valid(&vec![
            "aa".to_string(),
            "bb".to_string(),
            "cc".to_string(),
            "dd".to_string(),
            "aa".to_string()
        ]),
        false
    );
    assert_eq!(
        is_valid(&vec![
            "aa".to_string(),
            "bb".to_string(),
            "cc".to_string(),
            "dd".to_string(),
            "aaa".to_string()
        ]),
        true
    );
}

#[test]
fn test_answer2() {
    let input = String::from(
        r#"
abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio"#,
    );

    assert_eq!(answer2(&input), 3);
}

#[test]
fn test_is_valid_2() {
    assert_eq!(
        is_valid_2(&vec!["abcde".to_string(), "fghij".to_string()]),
        true
    );
    assert_eq!(
        is_valid_2(&vec![
            "abcde".to_string(),
            "xyz".to_string(),
            "ecdab".to_string()
        ]),
        false
    );
    assert_eq!(
        is_valid_2(&vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "abd".to_string(),
            "abf".to_string(),
            "abj".to_string()
        ]),
        true
    );
    assert_eq!(
        is_valid_2(&vec![
            "iiii".to_string(),
            "oiii".to_string(),
            "ooii".to_string(),
            "oooi".to_string(),
            "oooo".to_string()
        ]),
        true
    );
    assert_eq!(
        is_valid_2(&vec![
            "oiii".to_string(),
            "ioii".to_string(),
            "iioi".to_string(),
            "iiio".to_string()
        ]),
        false
    );
}
