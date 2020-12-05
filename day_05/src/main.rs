use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_05.txt";
    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Couldn't open file {}", file_path));
    let input_vec = utils::get_lines_from_input(&contents);
    println!(
        "Highest seat ID: {:?}",
        solutions::get_highest_seat_id(&input_vec)
    );
    println!(
        "Ordered seat IDs: {:?}",
        solutions::get_your_seat_id(&input_vec)
    )
}
