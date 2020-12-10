use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_10.txt";
    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Couldn't open file {}", file_path));
    let input_vec = utils::get_numbers_from_input(&contents);
    let (one, three) = solutions::get_jolt_differences(&input_vec);
    println!(
        "Number of 1-jolt differences multiplied by the number of 3-jolt differences: {}",
        one * three
    );
}
