use std::str::FromStr;

pub fn get_numbers_from_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| i64::from_str(line).unwrap())
        .collect::<Vec<i64>>()
}

pub fn get_lines_from_input(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}

pub fn get_lines_from_input_at_empty_line_split(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}
