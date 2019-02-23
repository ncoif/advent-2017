use aoc::day01;
use aoc::day02;
use aoc::day03;
use aoc::day04;
use aoc::day05;
use aoc::day06;
use aoc::day07;
use aoc::day08;
use aoc::day09;
use aoc::day10;
use aoc::day11;
use aoc::day12;
use aoc::day13;

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

    let input6 = read_file(6);
    println!("{} (1/2): {}", day06::title(), day06::answer1(&input6));
    println!("{} (2/2): {}", day06::title(), day06::answer2(&input6));

    let input7 = read_file(7);
    println!("{} (1/2): {}", day07::title(), day07::answer1(&input7));
    println!("{} (2/2): {}", day07::title(), day07::answer2(&input7));

    let input8 = read_file(8);
    println!("{} (1/2): {}", day08::title(), day08::answer1(&input8));
    println!("{} (2/2): {}", day08::title(), day08::answer2(&input8));

    let input9 = read_file(9);
    println!("{} (1/2): {}", day09::title(), day09::answer1(&input9));
    println!("{} (2/2): {}", day09::title(), day09::answer2(&input9));

    let input10 = read_file(10);
    println!(
        "{} (1/2): {}",
        day10::title(),
        day10::answer1(256, &input10)
    );
    println!(
        "{} (2/2): {}",
        day10::title(),
        day10::answer2(256, &input10)
    );

    let input11 = read_file(11);
    println!("{} (1/2): {}", day11::title(), day11::answer1(&input11));
    println!("{} (2/2): {}", day11::title(), day11::answer2(&input11));

    let input12 = read_file(12);
    println!("{} (1/2): {}", day12::title(), day12::answer1(&input12));
    println!("{} (2/2): {}", day12::title(), day12::answer2(&input12));

    let input13 = read_file(13);
    println!("{} (1/2): {}", day13::title(), day13::answer1(&input13));
}
