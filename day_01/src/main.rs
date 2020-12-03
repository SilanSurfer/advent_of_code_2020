use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_01.txt";
    let contents =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Couldn't open file {}", file_path));
    println!(
        "First task: {}",
        solutions::calculate_first_task(&contents).unwrap()
    );
    println!(
        "Second task: {}",
        solutions::calculate_second_task(&contents).unwrap()
    );
}
