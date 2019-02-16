pub fn title() -> &'static str {
    "Day 05: A Maze of Twisty Trampolines, All Alike"
}

pub fn answer1(input: &str) -> u32 {
    let mut instructions: Vec<isize> = parse_input(input);

    let mut current_instruction: isize = 0;
    let mut jumps = 0;

    while 0 <= current_instruction && current_instruction < instructions.len() as isize {
        let keep: isize = current_instruction;
        current_instruction = current_instruction + instructions[current_instruction as usize];
        instructions[keep as usize] += 1;

        jumps += 1;
    }

    jumps
}

pub fn answer2(input: &str) -> u32 {
    let mut instructions: Vec<isize> = parse_input(input);

    let mut current_instruction: isize = 0;
    let mut jumps = 0;

    while 0 <= current_instruction && current_instruction < instructions.len() as isize {
        let keep: isize = current_instruction;
        current_instruction = current_instruction + instructions[current_instruction as usize];

        if instructions[keep as usize] >= 3 {
            instructions[keep as usize] -= 1
        } else {
            instructions[keep as usize] += 1
        }

        jumps += 1;
    }

    jumps
}

fn parse_input(input: &str) -> Vec<isize> {
    let lines = input.split('\n');
    lines
        .filter(|l| *l != "")
        .map(|l| l.parse::<isize>())
        .filter(|n| n.is_ok())
        .map(Result::unwrap) // safe because skip invalid elements
        .collect()
}

#[test]
fn test_answer1() {
    let input = String::from(
        r#"
0
3
0
1
-3"#,
    );

    assert_eq!(answer1(&input), 5);
}

#[test]
fn test_answer2() {
    let input = String::from(
        r#"
0
3
0
1
-3"#,
    );

    assert_eq!(answer2(&input), 10);
}
