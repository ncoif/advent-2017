pub fn title() -> &'static str {
    "Day 17: Spinlock"
}

pub fn answer1(input: usize) -> usize {
    // the index of the vector is the value number, which contains a tuple (cw, ccw) to link to left and right
    let mut values: Vec<(usize, usize)> = Vec::with_capacity(2017);
    values.push((0, 0));
    let mut current = 0;

    for i in 1..=2017 {
        // follow 3 jumps
        for _ in 1..=input {
            current = values[current].1;
        }

        // insert the value
        let (_left, right) = values[current];
        values.push((current, right));

        // and update pointers for the neightbours
        values[current].1 = i;
        values[right].0 = i;
        current = i;
    }

    values[current].1
}

#[test]
fn test_answer1() {
    assert_eq!(answer1(3), 638);
}
