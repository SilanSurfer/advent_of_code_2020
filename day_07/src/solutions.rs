pub fn get_no_of_bag_colors_containing_shiny_gold_bag(input: &[&str]) -> u32 {
    input.iter().for_each(|line| parse_input_line(line));
    0
}

fn parse_input_line(line: &str) {
    let split: Vec<&str> = line.split("bags contain").collect();
    let rule = split[1]
        .trim_start()
        .strip_suffix(".")
        .unwrap()
        .trim_end_matches("bags")
        .trim_end_matches("bag");
    println!("{} -> {}", split[0], rule);   
}
