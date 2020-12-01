use std::str::FromStr;

pub fn get_numbers_from_input(input: &String) -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();
    for line in input.lines() {
        numbers.push(i64::from_str(line).unwrap());
    }
    numbers
}