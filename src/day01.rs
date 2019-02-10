use itertools::zip;

pub fn answer1(input: &str) -> u32 {
    let digits: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();

    let mut digits_cycled: Vec<u32> = digits.iter().skip(1).cloned().collect();
    digits_cycled.push(digits[0]);

    zip(&digits, &digits_cycled).fold(0, |sum, (a, b)| if a == b { sum + a } else { sum })
}

#[test]
fn test_answer1_1122() {
    assert_eq!(answer1("1122"), 3);
}

#[test]
fn test_answer1_1111() {
    assert_eq!(answer1("1111"), 4);
}

#[test]
fn test_answer1_1234() {
    assert_eq!(answer1("1234"), 0);
}

#[test]
fn test_answer1_91212129() {
    assert_eq!(answer1("91212129"), 9);
}
