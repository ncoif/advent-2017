pub fn title() -> &'static str {
    "Day 02: Corruption Checksum"
}

pub fn answer1(input: &str) -> u32 {
    let spreadsheet = parse_input(input);

    spreadsheet.iter().fold(0, |checksum, line| {
        checksum + (line.iter().max().unwrap() - line.iter().min().unwrap())
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
