pub fn title() -> &'static str {
    "Day 3: Spiral Memory"
}

#[derive(Debug)]
struct Spiral {
    dx: isize,
    dy: isize,
    leg: u8,
    layer: isize,
    size: usize,
    grid: Vec<Vec<u32>>,
}

impl Spiral {
    fn new1(size: usize) -> Self {
        let mut spiral = Spiral::default();
        (1..size).for_each(|_| spiral.next());
        spiral
    }

    fn new2(size: usize) -> Self {
        let mut spiral = Spiral::default();

        // overshooting here, by creating a grid much larger than necessary
        let mut grid: Vec<Vec<u32>> = vec![vec![]; size];
        (0..size).for_each(|x| grid[x] = vec![0; size]);
        spiral.grid = grid;

        let center = ((size) / 2) as isize;
        spiral.grid[center as usize][center as usize] = 1;

        (1..size).for_each(|_| {
            spiral.next();
            let center_d = ((center + spiral.dx) as usize, (center + spiral.dy) as usize);
            println!("{:?}", center_d);
            spiral.grid[center_d.0][center_d.1] = spiral.neighbourd_sum(center_d);
        });
        spiral
    }

    fn next(&mut self) {
        match self.leg {
            0 => {
                self.dx += 1;
                if self.dx == self.layer {
                    self.leg += 1
                };
            }
            1 => {
                self.dy += 1;
                if self.dy == self.layer {
                    self.leg += 1
                };
            }
            2 => {
                self.dx -= 1;
                if -self.dx == self.layer {
                    self.leg += 1
                };
            }
            3 => {
                self.dy -= 1;
                if -self.dy == self.layer {
                    self.leg = 0;
                    self.layer += 1
                };
            }
            _ => unreachable!(),
        };
    }

    fn neighbourd_sum(&self, center: (usize, usize)) -> u32 {
        self.grid[center.0][center.1]
            + self.grid[center.0 + 1][center.1]
            + self.grid[center.0 - 1][center.1]
            + self.grid[center.0][center.1 + 1]
            + self.grid[center.0 + 1][center.1 + 1]
            + self.grid[center.0 - 1][center.1 + 1]
            + self.grid[center.0][center.1 - 1]
            + self.grid[center.0 + 1][center.1 - 1]
            + self.grid[center.0 - 1][center.1 - 1]
    }
}

impl Default for Spiral {
    fn default() -> Self {
        Spiral {
            dx: 0,
            dy: 0,
            leg: 0,
            layer: 1,
            size: 0,
            grid: vec![],
        }
    }
}

pub fn answer1(input: usize) -> u32 {
    let spiral = Spiral::new1(input);

    (spiral.dx.abs() + spiral.dy.abs()) as u32
}

pub fn answer2(input: usize) -> u32 {
    0
}

#[test]
fn test_spiral_next() {
    let mut spiral = Spiral::default();

    assert_eq!((spiral.dx, spiral.dy), (0, 0));
    spiral.next();
    assert_eq!((spiral.dx, spiral.dy), (1, 0));
    spiral.next();
    assert_eq!((spiral.dx, spiral.dy), (1, 1));
    spiral.next();
    assert_eq!((spiral.dx, spiral.dy), (0, 1));
    spiral.next();
    assert_eq!((spiral.dx, spiral.dy), (-1, 1));
    spiral.next();
    assert_eq!((spiral.dx, spiral.dy), (-1, 0));
    spiral.next();
    assert_eq!((spiral.dx, spiral.dy), (-1, -1));
    spiral.next();
    assert_eq!((spiral.dx, spiral.dy), (0, -1));
    spiral.next();
    assert_eq!((spiral.dx, spiral.dy), (1, -1));
}

#[test]
fn test_spiral_grid() {
    let spiral = Spiral::new2(9);

    println!("{:?}", spiral);

    assert_eq!(spiral.grid[4 + 0][4 + 0], 1);
    assert_eq!(spiral.grid[4 + 1][4 + 0], 1);
    assert_eq!(spiral.grid[4 + 1][4 + 1], 2);
    assert_eq!(spiral.grid[4 + 0][4 + 1], 4);
    assert_eq!(spiral.grid[4 - 1][4 + 1], 5);
    assert_eq!(spiral.grid[4 - 1][4 + 0], 10);
    assert_eq!(spiral.grid[4 - 1][4 - 1], 11);
    assert_eq!(spiral.grid[4 + 0][4 - 1], 23);
    assert_eq!(spiral.grid[4 + 1][4 - 1], 25);
}

#[test]
fn answer1_square_1() {
    assert_eq!(answer1(1), 0);
}

#[test]
fn answer1_square_12() {
    assert_eq!(answer1(12), 3);
}

#[test]
fn answer1_square_23() {
    assert_eq!(answer1(23), 2);
}

#[test]
fn answer1_square_1024() {
    assert_eq!(answer1(1024), 31);
}
