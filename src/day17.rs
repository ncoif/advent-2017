pub fn title() -> &'static str {
    "Day 17: Spinlock"
}

fn iterate(iterations: usize, input: usize) -> Vec<usize> {
    let mut values: Vec<usize> = Vec::with_capacity(iterations);
    values.push(0);

    for i in 1..=iterations {
        let mut current = i - 1;
        // follow input jumps
        for _ in 1..=input {
            current = values[current];
        }

        // insert the value
        values.push(values[current]);

        // and update pointers for the neightbours
        values[current] = i;
    }

    values
}

pub fn answer1(input: usize) -> usize {
    let values = iterate(2017, input);
    values[2017]
}

pub fn answer2(input: usize) -> usize {
    let mut buflen = 1;
    let mut pos = 0;
    let mut result = 0;

    for i in 1..50_000_000 {
        pos = (pos + input) % buflen + 1;
        buflen += 1;
        if pos == 1 {
            result = i;
        }
    }
    result
}

#[test]
fn test_answer1() {
    assert_eq!(answer1(3), 638);
}
