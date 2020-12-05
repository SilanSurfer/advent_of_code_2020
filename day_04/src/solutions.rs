use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum PassportKeys {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

impl PassportKeys {
    pub fn new(name: &str) -> Self {
        match name {
            "byr" => PassportKeys::Byr,
            "iyr" => PassportKeys::Iyr,
            "eyr" => PassportKeys::Eyr,
            "hgt" => PassportKeys::Hgt,
            "hcl" => PassportKeys::Hcl,
            "ecl" => PassportKeys::Ecl,
            "pid" => PassportKeys::Pid,
            "cid" => PassportKeys::Cid,
            _ => panic!("There is no such key as {}", name),
        }
    }
}

pub fn count_valid_passports(input: &Vec<&str>) -> u32 {
    let mut valid_count = 0;
    for passport in input {
        let mut params_map = HashMap::new();
        let params: Vec<&str> = passport.split_whitespace().collect();
        for param in params {
            let mut param_iter = param.split(':');
            params_map.insert(
                PassportKeys::new(param_iter.next().expect("Couldn't split at :, no key!")),
                param_iter.next().expect("Couldn't split at :, no value!"),
            );
        }
        if (params_map.len() == 8)
            || (params_map.len() == 7 && !params_map.contains_key(&PassportKeys::Cid))
        {
            valid_count += 1;
        }
    }
    valid_count
}

pub fn count_valid_passports_with_param_validation(input: &Vec<&str>) -> u32 {
    let mut valid_count = 0;
    for passport in input {
        let mut params_map = HashMap::new();
        let params: Vec<&str> = passport.split_whitespace().collect();
        for param in params {
            let mut param_iter = param.split(':');
            params_map.insert(
                PassportKeys::new(param_iter.next().expect("Couldn't split at :, no key!")),
                param_iter.next().expect("Couldn't split at :, no value!"),
            );
        }
        if (params_map.len() == 8
            || (params_map.len() == 7 && !params_map.contains_key(&PassportKeys::Cid)))
                && validate_parameters(&params_map)
        {
            valid_count += 1;
        }
    }
    valid_count
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
//     If cm, the number must be at least 150 and at most 193.
//     If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.

fn validate_parameters(param_map: &HashMap<PassportKeys, &str>) -> bool {
    const VALID_EYE_COLOR: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    const VALID_LETTER: [char; 6] = ['a', 'b', 'c', 'd', 'e', 'f'];
    let mut result = true;
    for (key, value) in param_map.iter() {
        println!("Checking val {}", value);
        result = result
            && match key {
                PassportKeys::Byr => validate_year(value, 1920, 2002),
                PassportKeys::Iyr => validate_year(value, 2010, 2020),
                PassportKeys::Eyr => validate_year(value, 2020, 2030),
                PassportKeys::Hgt => {
                    if value.len() <= 2 {
                        return false;
                    }
                    let val = &value[..value.len()-2];
                    let unit = &value[value.len()-2..value.len()];
                    let num = val.parse::<u32>().expect("Couldn't convert to u32");
                    if unit == "cm" {
                        num >= 150 && num <= 193
                    } else if unit == "in" {
                        num >= 59 && num <= 76
                    } else {
                        false
                    }
                }
                PassportKeys::Hcl => {
                    value
                        .chars()
                        .enumerate()
                        .filter(|(no, letter)| {
                            if *no == 0  && *letter != '#' {
                                false
                            }
                            else if *no == 0 && *letter == '#' {
                                true
                            } else if *no >= 1 && *no <= 6 {
                                letter.is_digit(10) || VALID_LETTER.contains(letter)
                            } else {
                                false
                            }
                        })
                        .count()
                        == 7
                }
                PassportKeys::Ecl => VALID_EYE_COLOR.contains(value),
                PassportKeys::Pid => {
                    value
                        .chars()
                        .filter(|letter| letter.is_digit(10))
                        .count()
                        == 9
                }
                PassportKeys::Cid => true,
            };
        if !result {
            return result;
        }
    }
    result
}

fn validate_year(value: &str, min: u32, max: u32) -> bool {
    if value.len() != 4 {
        return false;
    }
    let number = value.parse::<u32>();
    if let Ok(number) = number {
        number >= min && number <= max
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_validate_paramaters_on_valid_data() {
        let mut input: HashMap<PassportKeys, &str> = HashMap::new();
        input.insert(PassportKeys::Pid, "087499704");
        input.insert(PassportKeys::Hgt, "74in");
        input.insert(PassportKeys::Ecl, "grn");
        input.insert(PassportKeys::Iyr, "2012");
        input.insert(PassportKeys::Eyr, "2030");
        input.insert(PassportKeys::Byr, "1980");
        input.insert(PassportKeys::Hcl, "#623a2f");

        assert!(validate_parameters(&input));
    }
}
