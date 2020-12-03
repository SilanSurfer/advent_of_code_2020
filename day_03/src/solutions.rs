const STEP: (usize, usize) = (3, 1);
const STARTING_POINT: (usize, usize) = (0, 0);
const STEPS: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

pub fn trees_in_path_with_default_step(input: &Vec<&str>) -> u32 {
    trees_in_path(input, &STEP) as u32
}

pub fn multiplication_of_tress_in_path_with_different_steps(input: &Vec<&str>) -> u64 {
    STEPS
        .iter()
        .fold(1, |acc: u64, item| acc * trees_in_path(input, item))
}

fn trees_in_path(input: &Vec<&str>, step: &(usize, usize)) -> u64 {
    let mut x_pos: usize = STARTING_POINT.1;
    let len = input[0].len();

    input
        .iter()
        .enumerate()
        .filter(|(no, _)| no % step.1 == 0)
        .fold(0, |acc, (_, line)| {
            let obj = line
                .chars()
                .nth(x_pos)
                .expect("Out of range in x position!");
            x_pos = (x_pos + step.0) % len;
            if obj == '#' {
                acc + 1
            } else {
                acc
            }
        })
}
