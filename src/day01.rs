pub fn title() -> &'static str {
    "Day 01: Inverse Captcha"
}

pub fn answer1(input: &str) -> u32 {
    let digits: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10))
        .filter(Option::is_some)
        .map(Option::unwrap) // safe because of the .is_some()
        .collect();

    let digits_cycled_iter = digits.iter().cycle().skip(1);
    digits
        .iter()
        .zip(digits_cycled_iter)
        .fold(0, |sum, (a, b)| if a == b { sum + a } else { sum })
}

pub fn answer2(input: &str) -> u32 {
    let digits: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10))
        .filter(Option::is_some)
        .map(Option::unwrap) // safe because of the .is_some()
        .collect();

    let digits_cycled_iter = digits.iter().cycle().skip(digits.len() / 2);
    digits
        .iter()
        .zip(digits_cycled_iter)
        .fold(0, |sum, (a, b)| if a == b { sum + a } else { sum })
}

#[test]
fn test_answer1() {
    assert_eq!(answer1("1122"), 3);
    assert_eq!(answer1("1111"), 4);
    assert_eq!(answer1("1234"), 0);
    assert_eq!(answer1("91212129"), 9);
}

#[test]
fn test_answer2() {
    assert_eq!(answer2("1212"), 6);
    assert_eq!(answer2("1221"), 0);
    assert_eq!(answer2("123425"), 4);
    assert_eq!(answer2("123123"), 12);
    assert_eq!(answer2("12131415"), 4);
}
