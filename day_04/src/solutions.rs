use std::collections::HashMap;

pub fn count_valid_passports(input: &Vec<&str>) -> u32 {
    let mut valid_count = 0;
    for passport in input {
        let mut params_map = HashMap::new();
        let params: Vec<&str> = passport.split_whitespace().collect();
        for param in params {
            let mut param_iter = param.split(':');
            params_map.insert(param_iter.next(), param_iter.next());
        }
        if (params_map.len() == 8)
            || (params_map.len() == 7 && !params_map.contains_key(&Some("cid")))
        {
            valid_count += 1;
        }
    }
    valid_count
}

pub fn count_valid_passports_with_param_validation(input: &Vec<&str>) -> u32 {
    let mut valid_count = 0;
    valid_count
}
