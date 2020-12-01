use std::str::FromStr;

const EXPECTED_SUM: i64 = 2020;

pub fn calculate_first_task(input: &String) -> Option<i64> {
    let numbers = get_numbers_from_input(input);
    let len = numbers.len();

    for x in 0..len - 1 {
        let (_, second) = numbers.split_at(x + 1);
        for y in second {
            if numbers[x] + y == EXPECTED_SUM {
                return Some(numbers[x] * y);
            }
        }
    }
    None
}

pub fn calculate_second_task(input: &String) -> Option<i64> {
    let numbers = get_numbers_from_input(input);
    let len = numbers.len();

    for x in 0..len - 2 {
        for y in x + 1..len - 1 {
            let (_, second) = numbers.split_at(y + 1);
            for z in second {
                if numbers[x] + numbers[y] + z == EXPECTED_SUM {
                    return Some(numbers[x] * numbers[y] * z);
                }
            }
        }
    }
    None
}


fn get_numbers_from_input(input: &String) -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();
    for line in input.lines() {
        numbers.push(i64::from_str(line).unwrap());
    }
    numbers
}