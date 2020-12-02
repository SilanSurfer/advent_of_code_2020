use regex::Regex;
use std::str::FromStr;

const REGEX_EXTRACT_DATA: &str = r"(\d+)-(\d+) (\w): (\w+)";

pub fn get_valid_passwords_count_policy(contents: &Vec<&str>) -> u32 {
    let re = Regex::new(REGEX_EXTRACT_DATA).unwrap();
    let mut valid: u32 = 0;
    for line in contents {
        let caps = re.captures(line).expect("Could match regexp!");
        let min: usize = caps
            .get(1)
            .map(|val| usize::from_str(val.as_str()).expect("Min value should be unsigned int!"))
            .expect("Min value couldn't be captured");
        let max: usize = caps
            .get(2)
            .map(|val| usize::from_str(val.as_str()).expect("Max value should be unsigned int!"))
            .expect("Max value couldn't be captured");
        let character = caps.get(3).expect("Char couldn't be captured!").as_str();
        let password = caps
            .get(4)
            .expect("Password couldn't be captured!")
            .as_str();
        let c = password.matches(character).count();
        if (min..=max).contains(&c) {
            valid += 1;
        }
    }
    valid
}

pub fn get_valid_password_positions_policy(contents: &Vec<&str>) -> u32 {
    let re = Regex::new(REGEX_EXTRACT_DATA).unwrap();
    let mut valid: u32 = 0;
    for line in contents {
        let caps = re.captures(line).expect("Could match regexp!");
        let first_pos: usize = caps
            .get(1)
            .map(|val| usize::from_str(val.as_str()).expect("Min value should be unsigned int!"))
            .expect("Min value couldn't be captured")
            - 1;
        let second_pos: usize = caps
            .get(2)
            .map(|val| usize::from_str(val.as_str()).expect("Max value should be unsigned int!"))
            .expect("Max value couldn't be captured")
            - 1;
        let character = caps
            .get(3)
            .expect("Char couldn't be captured!")
            .as_str()
            .chars()
            .next()
            .unwrap();
        let password = caps
            .get(4)
            .expect("Password couldn't be captured!")
            .as_str();
        let first_char = password
            .chars()
            .nth(first_pos)
            .expect("Couldn't find char at first position!");
        let second_char = password
            .chars()
            .nth(second_pos)
            .expect("Couldn't find char at second position!");
        if first_char == character && second_char != character {
            valid += 1;
        } else if first_char != character && second_char == character {
            valid += 1;
        }
    }
    valid
}
