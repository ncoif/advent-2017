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
        is_valid(&vec_of_strings!["aa", "bb", "cc", "dd", "ee"]),
        true
    );
    assert_eq!(
        is_valid(&vec_of_strings!["aa", "bb", "cc", "dd", "aa"]),
        false
    );
    assert_eq!(
        is_valid(&vec_of_strings!["aa", "bb", "cc", "dd", "aaa"]),
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
    assert_eq!(is_valid_2(&vec_of_strings!["abcde", "fghij"]), true);
    assert_eq!(is_valid_2(&vec_of_strings!["abcde", "xyz", "ecdab"]), false);
    assert_eq!(
        is_valid_2(&vec_of_strings!["a", "ab", "abc", "abd", "abf", "abj"]),
        true
    );
    assert_eq!(
        is_valid_2(&vec_of_strings!["iiii", "oiii", "ooii", "oooi", "oooo"]),
        true
    );
    assert_eq!(
        is_valid_2(&vec_of_strings!["oiii", "ioii", "iioi", "iiio"]),
        false
    );
}
