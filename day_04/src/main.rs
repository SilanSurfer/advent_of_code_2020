use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_04.txt";
    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Couldn't open file {}", file_path));
    let input_vec = utils::get_lines_from_input_at_empty_line_split(&contents);
    println!(
        "Valid passports: {}",
        solutions::count_valid_passports(&input_vec)
    );
    println!(
        "Valid passports with param validation: {}",
        solutions::count_valid_passports_with_param_validation(&input_vec)
    );
}
