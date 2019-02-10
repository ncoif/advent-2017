use std::fmt;

pub fn title() -> &'static str {
    "Day 3: Spiral Memory"
}

#[derive(Debug)]
struct Spiral {
    dx: isize,
    dy: isize,
    leg: u8,
    layer: isize,
    size: isize,
    grid: Vec<u32>,
}

impl Spiral {
    fn new(size: isize) -> Self {
        let mut spiral = Spiral::default();
        (1..size).for_each(|_| spiral.next());
        spiral
    }

    fn get(&self, p: (isize, isize)) -> u32 {
        self.grid[(p.0 + self.size * p.1) as usize]
    }

    fn set(&mut self, p: (isize, isize), val: u32) {
        self.grid[(p.0 + self.size * p.1) as usize] = val;
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

        if 2 * (self.dx + 1) > self.size {
            self.size = 2 * (self.dx + 1);
        }
        if 2 * (self.dy + 1) > self.size {
            self.size = 2 * (self.dy + 1);
        }
    }

    fn neighbourd_sum(&self, center: (isize, isize)) -> u32 {
        let mut ret = 0;
        for y in center.0.saturating_sub(1)..=center.0.saturating_add(1) {
            for x in center.1.saturating_sub(1)..=center.1.saturating_add(1) {
                if x >= self.size || y >= self.size {
                    continue;
                }
                ret += self.get((y, x));
            }
        }
        ret
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

impl fmt::Display for Spiral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.size {
            for y in 0..self.size {
                write!(f, "{:03} ", self.get((x, y)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn answer1(input: isize) -> u32 {
    let spiral = Spiral::new(input);

    (spiral.dx.abs() + spiral.dy.abs()) as u32
}

pub fn answer2(input: isize) -> u32 {
    // first pass to compute the required size for the grid here
    let spiral = Spiral::new(input);
    let required_grid_size = spiral.size;

    // second pass with the correct size
    let mut spiral = Spiral::default();
    spiral.size = required_grid_size;
    spiral.grid = vec![0; (required_grid_size * required_grid_size) as usize];

    let center = (required_grid_size / 2, required_grid_size / 2);
    spiral.set(center, 1);

    loop {
        spiral.next();
        let center_d = (center.0 + spiral.dx, center.1 + spiral.dy);
        let sum = spiral.neighbourd_sum(center_d);
        if sum as isize <= input {
            spiral.set(center_d, sum);
        } else {
            return sum;
        }
    }
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

#[test]
fn test_answer2() {
    assert_eq!(answer2(23), 25);
    assert_eq!(answer2(747), 806);
}
