use std::str::FromStr;

pub fn get_numbers_from_input(input: &String) -> Vec<i64> {
    input
        .lines()
        .map(|line| i64::from_str(line).unwrap())
        .collect::<Vec<i64>>()
}
