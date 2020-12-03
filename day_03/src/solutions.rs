
//use std::convert::TryInto;

const STEP: (usize, usize) = (3, 1);
const STARTING_POINT: (usize, usize) = (0, 0);

pub fn trees_in_path(input: &Vec<&str>) -> u32 {
    let mut x_pos: usize = STARTING_POINT.1;
    let mut counter = 0;
    let len = input[0].len();
    for line in input {
        let obj = line.chars().nth(x_pos).expect("Out of range in x position!");
        x_pos = (x_pos + STEP.0) % len;
        if obj == '#' {
            counter += 1;
        }

    }
    counter
}