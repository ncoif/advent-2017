use aoc::day01;
use aoc::day02;
use aoc::day03;
use aoc::day04;
use aoc::day05;

fn read_file(day: u32) -> String {
    let filename = format!("input/input{:02}.txt", day);
    std::fs::read_to_string(filename).unwrap()
}

fn main() {
    let input1 = read_file(1);
    println!("{} (1/2): {}", day01::title(), day01::answer1(&input1));
    println!("{} (2/2): {}", day01::title(), day01::answer2(&input1));

    let input2 = read_file(2);
    println!("{} (1/2): {}", day02::title(), day02::answer1(&input2));
    println!("{} (2/2): {}", day02::title(), day02::answer2(&input2));

    println!("{} (1/2): {}", day03::title(), day03::answer1(265_149));
    println!("{} (2/2): {}", day03::title(), day03::answer2(265_149));

    let input4 = read_file(4);
    println!("{} (1/2): {}", day04::title(), day04::answer1(&input4));
    println!("{} (2/2): {}", day04::title(), day04::answer2(&input4));

    let input5 = read_file(5);
    println!("{} (1/2): {}", day05::title(), day05::answer1(&input5));
    println!("{} (2/2): {}", day05::title(), day05::answer2(&input5));
}
