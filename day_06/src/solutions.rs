use std::collections::{HashMap, HashSet};

/// For each group, count the number of questions to which anyone answered "yes".
/// What is the sum of those counts?
pub fn get_sum_of_questions_with_yes_answer(input: &[&str]) -> u64 {
    input
        .iter()
        .map(|group| {
            group
                .lines()
                .flat_map(|line| line.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>() as u64
}

/// For each group, count the number of questions to which everyone answered "yes".
/// What is the sum of those counts?
pub fn get_sum_of_questions_with_everyone_yes_answer(input: &[&str]) -> u64 {
    let mut result: u64 = 0;
    for group in input.iter() {
        let mem_no: u32 = group.lines().count() as u32;
        let mut answers_map: HashMap<char, u32> = HashMap::new();
        for line in group.lines() {
            line.chars().for_each(|c| {
                if let Some(val) = answers_map.insert(c, 1) {
                    answers_map.insert(c, val + 1);
                }
            });
        }
        result += answers_map
            .iter()
            .filter(|(_, val)| **val == mem_no)
            .count() as u64
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_first_part() {
        let input: Vec<&str> = vec!["abc\n", "a\nb\nc\n", "ab\nac\n", "a\na\na\na\n", "b\n"];
        assert_eq!(11, super::get_sum_of_questions_with_yes_answer(&input));
    }
}
