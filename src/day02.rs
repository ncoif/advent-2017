pub fn title() -> &'static str {
    "Day 02: Corruption Checksum"
}

pub fn answer1(input: &str) -> u32 {
    let spreadsheet = parse_input(input);

    spreadsheet.iter().fold(0, |checksum, line| {
        checksum + (line.iter().max().unwrap() - line.iter().min().unwrap())
    })
}

pub fn answer2(input: &str) -> u32 {
    let spreadsheet = parse_input(input);

    spreadsheet.iter().fold(0, |checksum, line| {
        let divisible = evenly_divisible(line);
        checksum + (divisible.0 / divisible.1)
    })
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let lines = input.split('\n');
    let spreadsheet: Vec<Vec<u32>> = lines
        .filter(|l| *l != "")
        .map(|l| {
            l.split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    spreadsheet
}

fn evenly_divisible(line: &[u32]) -> (u32, u32) {
    for x in line {
        for y in line {
            if x != y && x % y == 0 {
                return (*x, *y);
            }
        }
    }
    unreachable!()
}

#[test]
fn test_answer1() {
    let input = String::from(
        r#"
5 1 9 5
7 5 3
2 4 6 8"#,
    );

    assert_eq!(answer1(&input), 18);
}

#[test]
fn test_answer2() {
    let input = String::from(
        r#"
5 9 2 8
9 4 7 3
3 8 6 5"#,
    );

    assert_eq!(answer2(&input), 9);
}

#[test]
fn test_evenly_divisible() {
    let input = vec![5, 9, 2, 8];
    assert_eq!(evenly_divisible(&input), (8, 2));
}
