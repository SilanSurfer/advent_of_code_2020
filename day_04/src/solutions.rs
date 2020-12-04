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
            _ => panic!("There is no such key as {}", name)
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
        if (params_map.len() == 8) || (params_map.len() == 7 && !params_map.contains_key(&PassportKeys::Cid)) {
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
        if (params_map.len() == 8)
            || (params_map.len() == 7 && !params_map.contains_key(&PassportKeys::Cid))
                && validate_parameters(&params_map)
        {
            valid_count += 1;
        }
    }
    valid_count
}
