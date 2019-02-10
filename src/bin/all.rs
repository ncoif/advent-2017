use aoc::day01;

fn run_day(day: u32) {
    let filename = format!("input/input{:02}.txt", day);
    let input = std::fs::read_to_string(filename).unwrap();

    let answer1 = day01::answer1(&input);
    println!("Day {:02}: Inverse Captcha (1/2): {}", day, answer1);

    let answer2 = day01::answer2(&input);
    println!("Day {:02}: Inverse Captcha (2/2): {}", day, answer2);
}

fn main() {
    run_day(1);
}
