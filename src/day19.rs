pub fn title() -> &'static str {
    "Day 19: A Series of Tubes"
}

pub fn answer1(input: &str) -> String {
    let grid = parse_input(input);
    dbg!(&grid);
    "".to_string()
}

pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|s| s.to_string().as_bytes().to_vec())
        .collect();

    let x_max = grid.iter().map(|l| l.len()).max().unwrap() as usize;

    // padd all the lines with ' ' to get a square grid
    grid.iter_mut().for_each(|line| {
        let padd_length = x_max - line.len();
        (0..padd_length).for_each(|_| line.push(' ' as u8));
    });

    grid
}

#[test]
fn test_answer1() {
    let input = String::from(
        r#"        |
        |  +--+
        A  |  C
    F---|----E|--+
        |  |  |  D
        +B-+  +--+"#,
    );

    assert_eq!(answer1(&input), "ABCDEF".to_string());
}
