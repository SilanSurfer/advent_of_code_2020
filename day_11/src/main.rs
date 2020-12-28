use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_11.txt";
    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Couldn't open file {}", file_path));
    let input_vec = utils::get_lines_from_input(&contents);
    println!(
        "Number of taken seats when state is stable {}",
        solutions::no_of_seats_taken(&input_vec)
    );
}
