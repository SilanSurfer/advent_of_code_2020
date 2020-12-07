use std::fs;

mod solutions;

fn main() {
    let file_path = "inputs/day_07.txt";
    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Couldn't open file {}", file_path));
    let input_vec = utils::get_lines_from_input(&contents);
    println!(
        "Bag colors that can eventually contain at least one shiny gold bag: {}",
        solutions::get_no_of_bag_colors_containing_shiny_gold_bag(&input_vec)
    );
}
