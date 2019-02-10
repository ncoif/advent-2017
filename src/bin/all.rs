use aoc::day01;

fn run_day(day: u32) {
    let filename = format!("input/input{:02}.txt", day);
    let input = std::fs::read_to_string(filename).unwrap();

    let answer1 = day01::answer1(&input);
    println!("Day {:02}: Inverse Captcha ({}/2): {}", day, 1, answer1)
}

fn main() {
    run_day(1);
}
