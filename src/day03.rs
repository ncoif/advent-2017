pub fn title() -> &'static str {
    "Day 3: Spiral Memory"
}

#[derive(Debug)]
struct Spiral {
    dx: isize,
    dy: isize,
    leg: u8,
    layer: isize,
}

impl Spiral {
    fn new(size: usize) -> Self {
        let mut spiral = Spiral::default();
        (1..size).for_each(|_| spiral.next());
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
}

impl Default for Spiral {
    fn default() -> Self {
        Spiral {
            dx: 0,
            dy: 0,
            leg: 0,
            layer: 1,
        }
    }
}

pub fn answer1(input: usize) -> u32 {
    let spiral = Spiral::new(input);

    (spiral.dx.abs() + spiral.dy.abs()) as u32
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
