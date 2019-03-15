pub fn title() -> &'static str {
    "Day 19: A Series of Tubes"
}

pub fn answer1(input: &str) -> String {
    let grid = parse_input(input);

    // find starting x
    let start_idx: usize = grid[0]
        .iter()
        .position(|&x| x == b'|')
        .expect("Not starting x found");

    let mut state = State::new(grid);
    state.move_cell(start_idx, 0, Dir::Down);
    while !state.is_end(state.x, state.y, state.dir) {
        step(&mut state);
    }

    //https://stackoverflow.com/questions/41034635/idiomatic-transformations-for-string-str-vecu8-and-u8
    String::from_utf8(state.current).unwrap()
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug)]
struct State {
    grid: Vec<Vec<u8>>,
    x: usize,
    y: usize,
    dir: Dir,
    current: Vec<u8>,
}

impl State {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        State {
            grid,
            x: 0,
            y: 0,
            dir: Dir::Down,
            current: vec![],
        }
    }

    fn move_cell(&mut self, x: usize, y: usize, direction: Dir) {
        let cur = self.grid[y][x];
        if cur != b' ' && cur != b'+' && cur != b'-' && cur != b'|' {
            self.current.push(cur);
        }

        self.x = x;
        self.y = y;
        self.dir = direction;
    }

    fn is_valid(&self, x: usize, y: usize) -> bool {
        x > 0 && x < self.grid[0].len() && y > 0 && y < self.grid.len()
    }

    fn is_blank(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == b' '
    }

    fn is_valid_blank(&self, x: usize, y: usize) -> bool {
        self.is_valid(x, y) && self.is_blank(x, y)
    }

    fn is_valid_not_blank(&self, x: usize, y: usize) -> bool {
        self.is_valid(x, y) && !self.is_blank(x, y)
    }

    fn is_end(&self, x: usize, y: usize, direction: Dir) -> bool {
        if self.grid[y][x] == b'+' {
            match direction {
                Dir::Up => self.is_valid_blank(x - 1, y) && self.is_valid_blank(x + 1, y),
                Dir::Down => self.is_valid_blank(x - 1, y) && self.is_valid_blank(x + 1, y),
                Dir::Left => self.is_valid_blank(x, y - 1) && self.is_valid_blank(x, y + 1),
                Dir::Right => self.is_valid_blank(x, y - 1) && self.is_valid_blank(x, y + 1),
            }
        } else {
            match direction {
                Dir::Up => self.is_valid_blank(x, y - 1),
                Dir::Down => self.is_valid_blank(x, y + 1),
                Dir::Left => self.is_valid_blank(x - 1, y),
                Dir::Right => self.is_valid_blank(x + 1, y),
            }
        }
    }
}

fn step(s: &mut State) {
    match (s.grid[s.y][s.x], s.dir) {
        (b'+', Dir::Up) => {
            if s.is_valid_not_blank(s.x - 1, s.y) {
                s.move_cell(s.x - 1, s.y, Dir::Left)
            } else if s.is_valid_not_blank(s.x + 1, s.y) {
                s.move_cell(s.x + 1, s.y, Dir::Right)
            } else {
                unreachable!()
            }
        }
        (b'+', Dir::Down) => {
            if s.is_valid_not_blank(s.x - 1, s.y) {
                s.move_cell(s.x - 1, s.y, Dir::Left)
            } else if s.is_valid_not_blank(s.x + 1, s.y) {
                s.move_cell(s.x + 1, s.y, Dir::Right)
            } else {
                unreachable!()
            }
        }
        (b'+', Dir::Left) => {
            if s.is_valid_not_blank(s.x, s.y - 1) {
                s.move_cell(s.x, s.y - 1, Dir::Up)
            } else if s.is_valid_not_blank(s.x, s.y + 1) {
                s.move_cell(s.x, s.y + 1, Dir::Down)
            } else {
                unreachable!()
            }
        }
        (b'+', Dir::Right) => {
            if s.is_valid_not_blank(s.x, s.y - 1) {
                s.move_cell(s.x, s.y - 1, Dir::Up)
            } else if s.is_valid_not_blank(s.x, s.y + 1) {
                s.move_cell(s.x, s.y + 1, Dir::Down)
            } else {
                unreachable!()
            }
        }
        (_, Dir::Up) => s.move_cell(s.x, s.y - 1, Dir::Up),
        (_, Dir::Down) => s.move_cell(s.x, s.y + 1, Dir::Down),
        (_, Dir::Left) => s.move_cell(s.x - 1, s.y, Dir::Left),
        (_, Dir::Right) => s.move_cell(s.x + 1, s.y, Dir::Right),
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
        (0..padd_length).for_each(|_| line.push(b' '));
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
