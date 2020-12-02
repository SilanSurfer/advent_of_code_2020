use regex::Regex;
use std::fs;
use std::str::FromStr;

const REGEX_EXTRACT_DATA: &str = r"(\d+)-(\d+) (\w): (\w+)";

fn get_valid_passwords_count_policy(contents: &Vec<&str>) -> u32 {
    let re = Regex::new(REGEX_EXTRACT_DATA).unwrap();
    let mut valid: u32 = 0;
    for line in contents {
        let caps = re.captures(line).unwrap();
        let min: usize = caps
            .get(1)
            .map(|val| usize::from_str(val.as_str()).unwrap())
            .unwrap();
        let max: usize = caps
            .get(2)
            .map(|val| usize::from_str(val.as_str()).unwrap())
            .unwrap();
        let character = caps.get(3).unwrap().as_str();
        let password = caps.get(4).unwrap().as_str();
        let c = password.matches(character).count();
        if (min..=max).contains(&c) {
            valid += 1;
        }
    }
    valid
}

fn get_valid_password_positions_policy(contents: &Vec<&str>) -> u32 {
    let re = Regex::new(REGEX_EXTRACT_DATA).unwrap();
    let mut valid: u32 = 0;
    for line in contents {
        let caps = re.captures(line).unwrap();
        let first_pos: usize = caps
            .get(1)
            .map(|val| usize::from_str(val.as_str()).unwrap())
            .unwrap() - 1;
        let second_pos: usize = caps
            .get(2)
            .map(|val| usize::from_str(val.as_str()).unwrap())
            .unwrap() - 1;
        let character = caps.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let password = caps.get(4).unwrap().as_str();
        let first_char = password.chars().nth(first_pos).unwrap();
        let second_char = password.chars().nth(second_pos).unwrap();
        if first_char == character && second_char != character {
            valid += 1;
        } else if first_char != character && second_char == character {
            valid += 1;
        }
    }
    valid
}

fn main() {
    let file_path = "inputs/day_02.txt";
    let contents =
        fs::read_to_string(file_path).expect(format!("Couldn't open file {}", file_path).as_str());
    let input_vec = utils::get_lines_from_input(&contents);
    println!(
        "Valid passwords with count policy: {}",
        get_valid_passwords_count_policy(&input_vec)
    );
    println!(
        "Valid passwords with position policy: {}",
        get_valid_password_positions_policy(&input_vec)
    );
}
