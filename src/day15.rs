pub fn title() -> &'static str {
    "Day 15: Dueling Generators"
}

struct Generator {
    factor: u64,
    divisor: u64,
    acceptable: u64,
    value: u64,
}

impl Generator {
    fn new(factor: u64, divisor: u64, acceptable: u64, start: u64) -> Self {
        Generator {
            factor,
            divisor,
            acceptable,
            value: start,
        }
    }

    fn next(&mut self) {
        self.value = self.value * self.factor % self.divisor
    }

    fn next_2(&mut self) {
        self.next();
        while self.value % self.acceptable != 0 {
            self.next();
        }
    }
}

pub fn answer1(a: u64, b: u64) -> u32 {
    let mut gen_a = Generator::new(16807, 2_147_483_647, 4, a);
    let mut gen_b = Generator::new(48271, 2_147_483_647, 8, b);

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

pub fn answer2(a: u64, b: u64) -> u32 {
    let mut gen_a = Generator::new(16807, 2_147_483_647, 4, a);
    let mut gen_b = Generator::new(48271, 2_147_483_647, 8, b);

    let mask = 0b0000_0000_0000_0000_1111_1111_1111_1111u64;

    let mut count = 0;
    (0..5_000_000).for_each(|_| {
        gen_a.next_2();
        gen_b.next_2();
        if (gen_a.value & mask) == (gen_b.value & mask) {
            count += 1;
        }
    });

    count
}

#[test]
fn test_generator_a() {
    let mut gen_a = Generator::new(16807, 2_147_483_647, 4, 65);
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
fn test_generator_a_next_2() {
    let mut gen_a = Generator::new(16807, 2_147_483_647, 4, 65);
    gen_a.next_2();
    assert_eq!(gen_a.value, 1352636452);
    gen_a.next_2();
    assert_eq!(gen_a.value, 1992081072);
    gen_a.next_2();
    assert_eq!(gen_a.value, 530830436);
    gen_a.next_2();
    assert_eq!(gen_a.value, 1980017072);
    gen_a.next_2();
    assert_eq!(gen_a.value, 740335192);
}

#[test]
fn test_answer1() {
    assert_eq!(answer1(65, 8921), 588);
}

#[test]
fn test_answer2() {
    assert_eq!(answer2(65, 8921), 309);
}
