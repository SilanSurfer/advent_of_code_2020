use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_08.txt";
    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Couldn't open file {}", file_path));
    let input_vec = utils::get_lines_from_input(&contents);
    println!(
        "ACC value: {}",
        solutions::get_acc_value(&input_vec)
    );
}