use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_03.txt";
    let contents =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Couldn't open file {}", file_path));
    let input_vec = utils::get_lines_from_input(&contents);
    println!(
        "No of trees in path: {}",
        solutions::trees_in_path_with_default_step(&input_vec)
    );
    println!(
        "Multiplication of no of trees in many paths: {}",
        solutions::multiplication_of_tress_in_path_with_different_steps(&input_vec)
    );
}
