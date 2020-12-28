use std::collections::HashMap;
use std::convert::TryInto;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

pub fn no_of_seats_taken(input: &[&str]) -> u64 {
    let mut seats_map = map_input(input);
    let mut was_change = true;
    while was_change {
        let result = next_step(&seats_map);
        seats_map = result.0;
        was_change = result.1
    }
    seats_map
        .iter()
        .filter(|(_, val)| **val == '#')
        .count()
        .try_into()
        .expect("Couldn't convert final value from usize to u32")
}

fn next_step(seats_map: &HashMap<(usize, usize), char>) -> (HashMap<(usize, usize), char>, bool) {
    let mut next_seats_map = HashMap::new();
    let mut was_change = false;
    for (key, val) in seats_map.iter() {
        for pos in &DIRECTIONS {}
    }
    (next_seats_map, was_change)
}

fn map_input(input: &[&str]) -> HashMap<(usize, usize), char> {
    let mut seats_map = HashMap::new();
    for (y, line) in input.iter().enumerate() {
        line.chars().enumerate().for_each(|(x, val)| {
            seats_map.insert((x, y), val);
        });
    }
    seats_map
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one_test() {
        let input = [
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ];

        assert_eq!(37, super::no_of_seats_taken(&input));
    }
}
