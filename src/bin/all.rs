use aoc::day01;

fn read_file(day: u32) -> String {
    let filename = format!("input/input{:02}.txt", day);
    std::fs::read_to_string(filename).unwrap()
}

fn main() {
    let input1 = read_file(1);
    println!("{} (1/2): {}", day01::title(), day01::answer1(&input1));
    println!("{} (2/2): {}", day01::title(), day01::answer2(&input1));
}
