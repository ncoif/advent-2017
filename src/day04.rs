use std::collections::HashSet;

pub fn title() -> &'static str {
    "Day 4: High-Entropy Passphrases"
}

pub fn answer1(input: &str) -> u32 {
    let passphrases: Vec<Vec<String>> = parse_input(input);

    passphrases
        .iter()
        .fold(0, |count, p| if is_valid(&p) { count + 1 } else { count })
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
