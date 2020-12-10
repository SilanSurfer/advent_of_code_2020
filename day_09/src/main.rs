use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_09.txt";
    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Couldn't open file {}", file_path));
    let input_vec = utils::get_numbers_from_input(&contents);
    println!(
        "First number that doesn't have property: {}",
        solutions::value_wihtout_sum(&input_vec, 25)
    );
}
