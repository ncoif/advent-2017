pub fn title() -> &'static str {
    "Day 15: Dueling Generators"
}

struct Generator {
    factor: u64,
    divisor: u64,
    value: u64,
}

impl Generator {
    fn new(factor: u64, divisor: u64, start: u64) -> Self {
        Generator {
            factor,
            divisor,
            value: start,
        }
    }

    fn next(&mut self) {
        self.value = self.value * self.factor % self.divisor
    }
}

pub fn answer1(a: u64, b: u64) -> u32 {
    let mut gen_a = Generator::new(16807, 2_147_483_647, a);
    let mut gen_b = Generator::new(48271, 2_147_483_647, b);

    let mask = 0b0000_0000_0000_0000_1111_1111_1111_1111u64;

    let mut count = 0;
    (0..40_000_000).for_each(|_| {
        gen_a.next();
        gen_b.next();
        if (gen_a.value & mask) == (gen_b.value & mask) {
            count += 1;
        }
    });

    count
}

#[test]
fn test_generator_a() {
    let mut gen_a = Generator::new(16807, 2_147_483_647, 65);
    gen_a.next();
    assert_eq!(gen_a.value, 1092455);
    gen_a.next();
    assert_eq!(gen_a.value, 1181022009);
    gen_a.next();
    assert_eq!(gen_a.value, 245556042);
    gen_a.next();
    assert_eq!(gen_a.value, 1744312007);
    gen_a.next();
    assert_eq!(gen_a.value, 1352636452);
}

#[test]
fn test_answer1() {
    assert_eq!(answer1(65, 8921), 588);
}
