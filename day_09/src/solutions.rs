pub fn value_wihtout_sum(input: &Vec<i64>, preamble: usize) -> i64 {
    let index: usize = preamble;
    for i in index..input.len() {
        let val = input[i];
        let slice = &input[(i - preamble)..i];
        println!("Checking slice {:?} for val {}", slice, val);
        let mut count = 0;
        for j in i - preamble..i {
            for k in j + 1..i {
                if input[k] + input[j] == val {
                    count += 1;
                }
            }
        }
        if count == 0 {
            return val;
        }
    }
    panic!("Something went wrong - couldn't find value")
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_test() {
        let input: Vec<i64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(super::value_wihtout_sum(&input, 5), 127);
    }
}
