pub fn title() -> &'static str {
    "Day 19: A Series of Tubes"
}

pub fn answer1(input: &str) -> String {
    let grid = parse_input(input);
    //dbg!(&grid);

    // find starting x
    let start_idx: usize = grid[0]
        .iter()
        .position(|&x| x == '|' as u8)
        .expect("Not starting x found");
    dbg!(start_idx);

    let mut result: Vec<u8> = vec![];
    step(&grid, start_idx, 0, &mut result, Dir::Down);
    dbg!(&result);

    //https://stackoverflow.com/questions/41034635/idiomatic-transformations-for-string-str-vecu8-and-u8
    std::str::from_utf8(&result).unwrap().trim().to_string()
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

fn step(grid: &Vec<Vec<u8>>, x: usize, y: usize, current: &mut Vec<u8>, direction: Dir) {
    dbg!((grid[y][x], &direction));
    match (grid[y][x], direction) {
        (b'|', Dir::Up) => step(grid, x, y - 1, current, Dir::Up),
        (b'|', Dir::Down) => step(grid, x, y + 1, current, Dir::Down),
        (b'-', Dir::Up) => step(grid, x, y - 1, current, Dir::Up),
        (b'-', Dir::Down) => step(grid, x, y + 1, current, Dir::Down),
        (b'-', Dir::Left) => {
            if x == 0 {
                return;
            } else {
                step(grid, x - 1, y, current, Dir::Left)
            }
        }
        (b'-', Dir::Right) => step(grid, x + 1, y, current, Dir::Right),
        (b'|', Dir::Left) => {
            if x == 0 {
                return;
            } else {
                step(grid, x - 1, y, current, Dir::Left)
            }
        }
        (b'|', Dir::Right) => step(grid, x + 1, y, current, Dir::Right),
        (b'+', Dir::Up) => {
            if x - 1 > 0 && grid[y][x - 1] != ' ' as u8 {
                step(grid, x - 1, y, current, Dir::Left)
            } else if x + 1 < grid[0].len() && grid[y][x + 1] != ' ' as u8 {
                step(grid, x + 1, y, current, Dir::Right)
            } else {
                unreachable!()
            }
        }
        (b'+', Dir::Down) => {
            if x - 1 > 0 && grid[y][x - 1] != ' ' as u8 {
                step(grid, x - 1, y, current, Dir::Left)
            } else if x + 1 < grid[0].len() && grid[y][x + 1] != ' ' as u8 {
                step(grid, x + 1, y, current, Dir::Right)
            } else {
                unreachable!()
            }
        }
        (b'+', Dir::Left) => {
            if y - 1 > 0 && grid[y - 1][x] != ' ' as u8 {
                step(grid, x, y - 1, current, Dir::Up)
            } else if y + 1 < grid.len() && grid[y + 1][x] != ' ' as u8 {
                step(grid, x, y + 1, current, Dir::Down)
            } else {
                unreachable!()
            }
        }
        (b'+', Dir::Right) => {
            if y - 1 > 0 && grid[y - 1][x] != ' ' as u8 {
                step(grid, x, y - 1, current, Dir::Up)
            } else if y + 1 < grid.len() && grid[y + 1][x] != ' ' as u8 {
                step(grid, x, y + 1, current, Dir::Down)
            } else {
                unreachable!()
            }
        }
        (v, Dir::Up) => {
            // that's a letter
            current.push(v);
            step(grid, x, y - 1, current, Dir::Up)
        }
        (v, Dir::Down) => {
            // that's a letter
            current.push(v);
            step(grid, x, y + 1, current, Dir::Down)
        }
        (v, Dir::Left) => {
            // that's a letter
            current.push(v);
            if x == 0 {
                return;
            } else {
                step(grid, x - 1, y, current, Dir::Left)
            }
        }
        (v, Dir::Right) => {
            // that's a letter
            current.push(v);
            step(grid, x + 1, y, current, Dir::Right)
        }
    };
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
        r#"
        |
        |  +--+
        A  |  C
    F---|----E|--+
        |  |  |  D
        +B-+  +--+"#,
    );

    assert_eq!(answer1(&input), "ABCDEF".to_string());
}
