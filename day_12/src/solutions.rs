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

#[derive(Debug)]
struct Waypoint {
    pos: (i32, i32),
}

#[derive(Debug)]
struct FerryWithWaypoint {
    ferry: (i32, i32),
    waypoint: Waypoint,
}

impl FerryWithWaypoint {
    fn new() -> Self {
        FerryWithWaypoint {
            ferry: (0, 0),
            waypoint: Waypoint::new(),
        }
    }

    fn apply(&mut self, action: &Action) {
        match action.name {
            'N' | 'S' | 'E' | 'W' => { self.waypoint.move_point(action); }
            'L' => {
                self.rotate_waypoint(action.value as i32);
            }
            'R' => {
                self.rotate_waypoint(360 - action.value as i32);
            }
            'F' => {
                self.ferry = (self.ferry.0 + self.waypoint.pos.0 * 10, self.ferry.1 + self.waypoint.pos.1 * 10);
            }
            _ => panic!("Unsupported action!!!"),
        }
    }

    fn get_manhattan_distance(&self) -> u64 {
        (self.ferry.0.abs() + self.ferry.1.abs())
            .try_into()
            .expect("Couldn't conver i32 to u64!")
    }

    fn rotate_waypoint(&mut self, angle: i32) {
        let original: (f64, f64) = (self.waypoint.pos.0 as f64, self.waypoint.pos.1 as f64);
        let angle_in_radians = (angle as f64).to_radians();
        let rotated = (original.0 * angle_in_radians.cos() - original.1 * angle_in_radians.sin(),
                       original.0 * angle_in_radians.sin() + original.1 * angle_in_radians.cos());
        self.waypoint.pos = (rotated.0 as i32, rotated.1 as i32);
    }
}

impl Waypoint {
    fn new() -> Self {
        Waypoint {
            pos: (10, 1),
        }
    }

    fn move_point(&mut self, action: &Action) {
        match action.name {
            'N' => {
                self.pos.0 += action.value as i32;
                println!("Action: {:#?} -> Waypoint: {:#?}", action, self);
            }
            'S' => {
                self.pos.0 -= action.value as i32;
                println!("Action: {:#?} -> Waypoint: {:#?}", action, self);
            }
            'E' => {
                self.pos.1 += action.value as i32;
                println!("Action: {:#?} -> Waypoint: {:#?}", action, self);
            }
            'W' => {
                self.pos.1 -= action.value as i32;
                println!("Action: {:#?} -> Waypoint: {:#?}", action, self);
            }
            _ => panic!("Waypoint doesn't support this operation")
        }
    }
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
                println!("Action: {:#?} -> Ferry: {:#?}", action, self);
            }
            'S' => {
                self.pos_y -= action.value as i32;
                println!("Action: {:#?} -> Ferry: {:#?}", action, self);
            }
            'E' => {
                self.pos_x += action.value as i32;
                println!("Action: {:#?} -> Ferry: {:#?}", action, self);
            }
            'W' => {
                self.pos_x -= action.value as i32;
                println!("Action: {:#?} -> Ferry: {:#?}", action, self);
            }
            'L' => {
                let rot_steps = action.value / 90;
                self.direction = Direction::from_u32((self.direction as u32 + rot_steps) % 4);
                println!("Action: {:#?} -> Ferry: {:#?}", action, self);
            }
            'R' => {
                let rot_steps = action.value / 90;
                self.direction =
                    Direction::from_u32((4 + self.direction as u32 - rot_steps as u32) % 4);
                println!("Action: {:#?} -> Ferry: {:#?}", action, self);
            }
            'F' => match self.direction {
                Direction::East => {
                    self.pos_x += action.value as i32;
                    println!("Action: {:#?} -> Ferry: {:#?}", action, self);
                }
                Direction::West => {
                    self.pos_x -= action.value as i32;
                    println!("Action: {:#?} -> Ferry: {:#?}", action, self);
                }
                Direction::North => {
                    self.pos_y += action.value as i32;
                    println!("Action: {:#?} -> Ferry: {:#?}", action, self);
                }
                Direction::South => {
                    self.pos_y -= action.value as i32;
                    println!("Action: {:#?} -> Ferry: {:#?}", action, self);
                }
            },
            _ => panic!("Unsupported action!!!"),
        }
    }

    fn get_manhattan_distance(&self) -> u64 {
        (self.pos_x.abs() + self.pos_y.abs())
            .try_into()
            .expect("Couldn't conver i32 to u64!")
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

pub fn get_manhattan_distance_with_waypoint(input: &[&str]) -> u64 {
    let actions = input
        .iter()
        .map(|line| Action::parse(line))
        .collect::<Vec<Action>>();
    let mut ferry_with_waypoint = FerryWithWaypoint::new();
    for action in actions {
        ferry_with_waypoint.apply(&action);
    }
    println!("Ferry with waypoint: {:#?}", ferry_with_waypoint);
    ferry_with_waypoint.get_manhattan_distance()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one_test() {
        let input = vec!["F10", "N3", "F7", "R90", "F11"];
        assert_eq!(25, super::get_manhattan_distance(&input));
    }

    #[test]
    fn part_two_test() {
        let input = vec!["F10", "N3", "F7", "R90", "F11"];
        assert_eq!(286, super::get_manhattan_distance_with_waypoint(&input));
    }
}
