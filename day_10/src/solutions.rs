pub fn get_jolt_differences(input: &Vec<i64>) -> (i64, i64) {
    let mut sorted = input.clone();
    sorted.push(0);
    sorted.sort();
    let result = sorted
        .iter()
        .zip(sorted.iter().skip(1))
        .fold((0, 0), |acc, x| {
            match x.1 - x.0 {
                1 => {
                    // println!("Diff 1");
                    (acc.0 + 1, acc.1)
                }
                2 => {
                    // println!("Diff 2");
                    acc
                }
                3 => {
                    // println!("Diff 3");
                    (acc.0, acc.1 + 1)
                }
                _ => {
                    panic!("What to do now?");
                }
            }
        });
    (result.0, result.1 + 1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_case() {
        let input: Vec<i64> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!((7, 5), super::get_jolt_differences(&input));
    }

    #[test]
    fn second_case() {
        let input: Vec<i64> = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!((22, 10), super::get_jolt_differences(&input));
    }
}
