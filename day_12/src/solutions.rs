use std::convert::TryInto;

// Action N means to move north by the given value.
// Action S means to move south by the given value.
// Action E means to move east by the given value.
// Action W means to move west by the given value.
// Action L means to turn left the given number of degrees.
// Action R means to turn right the given number of degrees.
// Action F means to move forward by the given value in the direction the
#[derive(Debug)]
struct Action {
    name: char,
    value: u32,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}

#[derive(Debug)]
struct Ferry {
    direction: Direction,
    pos_x: i32,
    pos_y: i32,
}

impl Action {
    fn parse(line: &str) -> Action {
        let name = line
            .chars()
            .next()
            .expect("Couldn't get name of operation!");
        let value = line[1..].parse().expect("Couldn't parse to u32!");
        Action { name, value }
    }
}

impl Direction {
    fn from_u32(value: u32) -> Direction {
        match value {
            0 => Direction::East,
            1 => Direction::North,
            2 => Direction::West,
            3 => Direction::South,
            _ => panic!("Can't convert to Direction!"),
        }
    }
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            direction: Direction::East,
            pos_x: 0,
            pos_y: 0,
        }
    }

    fn apply(&mut self, action: &Action) {
        match action.name {
            'N' => {
                self.pos_y += action.value as i32;
            }
            'S' => {
                self.pos_y -= action.value as i32;
            }
            'E' => {
                self.pos_x += action.value as i32;
            }
            'W' => {
                self.pos_x -= action.value as i32;
            }
            'L' => {
                let rot_steps = action.value / 90;
                self.direction = Direction::from_u32((self.direction as u32 + rot_steps) % 4);
            }
            'R' => {
                let rot_steps = action.value / 90;
                self.direction = Direction::from_u32(
                    ((self.direction as i32 - rot_steps as i32).abs() % 4)
                        .try_into()
                        .expect("Couldn't convert i32 to u32"),
                );
            }
            'F' => match self.direction {
                Direction::East => {
                    self.pos_x += action.value as i32;
                }
                Direction::West => {
                    self.pos_x -= action.value as i32;
                }
                Direction::North => {
                    self.pos_y += action.value as i32;
                }
                Direction::South => {
                    self.pos_y -= action.value as i32;
                }
            },
            _ => panic!("Unsupported action!!!"),
        }
    }

    fn get_manhattan_distance(&self) -> u64 {
        (self.pos_x.abs() + self.pos_y.abs())
            .try_into()
            .expect("Couldn't conver i32 to u32!")
    }
}

pub fn get_manhattan_distance(input: &[&str]) -> u64 {
    let actions = input
        .iter()
        .map(|line| Action::parse(line))
        .collect::<Vec<Action>>();
    let mut ferry = Ferry::new();
    for action in actions {
        ferry.apply(&action);
    }
    ferry.get_manhattan_distance()
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_test() {
        let input = vec!["F10", "N3", "F7", "R90", "F11"];
        assert_eq!(25, super::get_manhattan_distance(&input));
    }
}
