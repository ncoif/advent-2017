use crate::day10;

pub fn title() -> &'static str {
    "Day 14: Disk Defragmentation"
}

fn from_char_count_ones(c: char) -> u32 {
    match c {
        '0' => 0, //0000
        '1' => 1, //0001
        '2' => 1, //0010
        '3' => 2, //0011
        '4' => 1, //0100
        '5' => 2, //0101
        '6' => 2, //0110
        '7' => 3, //0111
        '8' => 1, //1000
        '9' => 2, //1001
        'a' => 2, //1010
        'b' => 3, //1011
        'c' => 2, //1100
        'd' => 3, //1101
        'e' => 3, //1110
        'f' => 4, //1111
        _ => unreachable!(),
    }
}

fn from_char_to_bytes(c: char) -> Vec<bool> {
    match c {
        '0' => vec![false, false, false, false], //0000
        '1' => vec![false, false, false, true],  //0001
        '2' => vec![false, false, true, false],  //0010
        '3' => vec![false, false, true, true],   //0011
        '4' => vec![false, true, false, false],  //0100
        '5' => vec![false, true, false, true],   //0101
        '6' => vec![false, true, true, false],   //0110
        '7' => vec![false, true, true, true],    //0111
        '8' => vec![true, false, false, false],  //1000
        '9' => vec![true, false, false, true],   //1001
        'a' => vec![true, false, true, false],   //1010
        'b' => vec![true, false, true, true],    //1011
        'c' => vec![true, true, false, false],   //1100
        'd' => vec![true, true, false, true],    //1101
        'e' => vec![true, true, true, false],    //1110
        'f' => vec![true, true, true, true],     //1111
        _ => unreachable!(),
    }
}

fn neighbours(grid: &[Vec<bool>]) -> Vec<Vec<usize>> {
    let mut neighbours = vec![vec![]];
    for x in 0..128 {
        for y in 0..128 {
            if grid[x][y] {
                let mut ns = vec![];
                ns.push(x * 128 + y);
                if x > 0 && grid[x - 1][y] {
                    ns.push((x - 1) * 128 + y);
                }
                if x < 126 && grid[x + 1][y] {
                    ns.push((x + 1) * 128 + y);
                }
                if y > 0 && grid[x][y - 1] {
                    ns.push(x * 128 + y - 1);
                }
                if y < 126 && grid[x][y + 1] {
                    ns.push(x * 128 + y + 1);
                }
                neighbours.push(ns);
            }
        }
    }

    neighbours
}

pub fn answer1(input: &str) -> u32 {
    (0..128)
        .map(|i| {
            let s = format!("{}-{}", input, i);
            day10::hash(&s)
                .chars()
                .fold(0, |count, c| count + from_char_count_ones(c))
        })
        .sum()
}

pub fn answer2(input: &str) -> usize {
    let mut grid: Vec<Vec<bool>> = vec![vec![]; 128];
    (0..128).for_each(|i| {
        let s = format!("{}-{}", input, i);
        grid[i].extend(
            day10::hash(&s)
                .chars()
                .flat_map(from_char_to_bytes)
                .collect::<Vec<bool>>(),
        );
    });
    let neighbours = neighbours(&grid);

    // https://docs.rs/pathfinding/1.1.10/pathfinding/undirected/connected_components/fn.components.html
    let groups = pathfinding::undirected::connected_components::components(&neighbours);
    groups.len()
}

#[test]
fn test_answer1() {
    let input = String::from("flqrgnkx");
    assert_eq!(answer1(&input), 8108);
}

#[test]
fn test_answer2() {
    let input = String::from("flqrgnkx");
    assert_eq!(answer2(&input), 1242);
}
