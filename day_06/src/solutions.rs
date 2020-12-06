use std::collections::{HashSet, HashMap};

/// For each group, count the number of questions to which anyone answered "yes".
/// What is the sum of those counts?
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


/// For each group, count the number of questions to which everyone answered "yes".
/// What is the sum of those counts?
pub fn get_sum_of_questions_with_everyone_yes_answer(input: &[&str]) -> u64 {
    let mut result: u64 = 0;
    for group in input.iter() {
        let mem_no: u32 = group.lines().count() as u32;
        let mut answers_map: HashMap<char, u32> = HashMap::new();
        for line in group.lines() {
            line
                .chars()
                .for_each(|c| {
                    if let Some(val) = answers_map.insert(c, 1) {
                        answers_map.insert(c, val + 1);
                    }
                });
        }
        result += answers_map.iter().filter(|(_, val)| **val == mem_no).count() as u64
    }
    result
}