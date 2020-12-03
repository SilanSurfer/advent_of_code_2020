const EXPECTED_SUM: i64 = 2020;

pub fn calculate_first_task(input: &str) -> Option<i64> {
    let numbers = utils::get_numbers_from_input(input);
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

pub fn calculate_second_task(input: &str) -> Option<i64> {
    let numbers = utils::get_numbers_from_input(input);
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
