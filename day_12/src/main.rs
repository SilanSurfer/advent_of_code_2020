use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_12.txt";
    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Couldn't open file {}", file_path));
    let input_vec = utils::get_lines_from_input(&contents);
    println!(
        "Manhattan distance is {}",
        solutions::get_manhattan_distance(&input_vec)
    );
    println!(
        "Manhattan distance with waypoint is{}",
        solutions::get_manhattan_distance_with_waypoint(&input_vec)
    );
}
