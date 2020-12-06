use std::collections::HashSet;

pub fn get_sum_of_questions_with_yes_answer(input: &[&str]) -> u64 {
    let mut result: u64 = 0;
    for group in input.iter() {
        let mut set: HashSet<char> = HashSet::new();
        for line in group.lines() {
            line.chars().for_each(|c| { set.insert(c); } );
        }
        // group.lines().map(|line| line.to_str()).chars().try_for_each(|c| set.insert(c));
        result += set.iter().count() as u64;
    }
    result
}