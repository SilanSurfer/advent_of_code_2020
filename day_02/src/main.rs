use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_02.txt";
    let contents =
        fs::read_to_string(file_path).expect(format!("Couldn't open file {}", file_path).as_str());
    let input_vec = utils::get_lines_from_input(&contents);
    println!(
        "Valid passwords with count policy: {}",
        solutions::get_valid_passwords_count_policy(&input_vec)
    );
    println!(
        "Valid passwords with position policy: {}",
        solutions::get_valid_password_positions_policy(&input_vec)
    );
}
