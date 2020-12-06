use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_06.txt";
    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Couldn't open file {}", file_path));
    let input_vec = utils::get_lines_from_input_at_empty_line_split(&contents);
    println!(
        "Sum of unique yes answers: {}",
        solutions::get_sum_of_questions_with_yes_answer(&input_vec)
    );
    println!(
        "Sum of question with everyone in group yes answered: {}",
        solutions::get_sum_of_questions_with_everyone_yes_answer(&input_vec)
    );
}
