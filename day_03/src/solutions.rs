const STEP: (usize, usize) = (3, 1);
const STARTING_POINT: (usize, usize) = (0, 0);
const STEPS: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

pub fn trees_in_path_with_default_step(input: &Vec<&str>) -> u32 {
    trees_in_path(input, STEP) as u32
}

pub fn multiplication_of_tress_in_path_with_different_steps(input: &Vec<&str>) -> u64 {
    let mut results: Vec<u64> = Vec::new();
    for step in STEPS.iter() {
        results.push(trees_in_path(input, *step));
    }
    results.iter().fold(1, |acc: u64, item| acc * item as &u64)
}

fn trees_in_path(input: &Vec<&str>, step: (usize, usize)) -> u64 {
    let mut x_pos: usize = STARTING_POINT.1;
    let mut counter = 0;
    let len = input[0].len();
    for (no, line) in input.iter().enumerate() {
        if no % step.1 != 0 {
            continue;
        }
        let obj = line
            .chars()
            .nth(x_pos)
            .expect("Out of range in x position!");
        x_pos = (x_pos + step.0) % len;
        if obj == '#' {
            counter += 1;
        }
    }
    counter
}
