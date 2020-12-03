use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_03.txt";
    let contents =
        fs::read_to_string(file_path).expect(format!("Couldn't open file {}", file_path).as_str());
    let input_vec = utils::get_lines_from_input(&contents);
    println!("No of trees in path: {}", solutions::trees_in_path(&input_vec));
}
