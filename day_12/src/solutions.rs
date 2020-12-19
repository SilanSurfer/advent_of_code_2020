use crate::point::Point;

#[derive(Debug)]
enum Action {
    East(i32),
    North(i32),
    West(i32),
    South(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
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
    pos: Point,
}

#[derive(Debug)]
struct FerryWithWaypoint {
    ferry: Point,
    waypoint: Point,
}

impl Action {
    fn parse(line: &str) -> Action {
        let value = line[1..].parse().expect("Couldn't parse to u32!");
        match &line[..1] {
            "N" => Action::North(value),
            "S" => Action::South(value),
            "E" => Action::East(value),
            "W" => Action::West(value),
            "L" => Action::Left(value),
            "R" => Action::Right(value),
            "F" => Action::Forward(value),
            _ => unreachable!(),
        }
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

impl FerryWithWaypoint {
    fn new() -> Self {
        FerryWithWaypoint {
            ferry: Point::new(),
            waypoint: Point::new_at(10, 1),
        }
    }

    fn apply(&mut self, action: &Action) {
        match action {
            Action::North(val) => {
                self.waypoint.move_y(*val);
            }
            Action::South(val) => {
                self.waypoint.move_y(-*val);
            }
            Action::East(val) => {
                self.waypoint.move_x(*val);
            }
            Action::West(val) => {
                self.waypoint.move_x(-*val);
            }
            Action::Left(rot) => {
                self.waypoint.rotate(*rot);
            }
            Action::Right(rot) => {
                self.waypoint.rotate(360 - *rot);
            }
            Action::Forward(val) => {
                self.ferry = Point::new_at(
                    self.ferry.get_x() + self.waypoint.get_x() * val,
                    self.ferry.get_y() + self.waypoint.get_y() * val,
                );
            }
        }
    }

    fn get_manhattan_distance(&self) -> u64 {
        self.ferry.get_manhattan_distance()
    }
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            direction: Direction::East,
            pos: Point::new(),
        }
    }

    fn apply(&mut self, action: &Action) {
        match action {
            Action::North(val) => {
                self.pos.move_y(*val);
            }
            Action::South(val) => {
                self.pos.move_y(-*val);
            }
            Action::East(val) => {
                self.pos.move_x(*val);
            }
            Action::West(val) => {
                self.pos.move_x(-*val);
            }
            Action::Left(val) => {
                let rot_steps = val / 90;
                self.direction =
                    Direction::from_u32((self.direction as u32 + rot_steps as u32) % 4);
            }
            Action::Right(val) => {
                let rot_steps = val / 90;
                self.direction =
                    Direction::from_u32((4 + self.direction as u32 - rot_steps as u32) % 4);
            }
            Action::Forward(val) => match self.direction {
                Direction::East => {
                    self.pos.move_x(*val);
                }
                Direction::West => {
                    self.pos.move_x(-*val);
                }
                Direction::North => {
                    self.pos.move_y(*val);
                }
                Direction::South => {
                    self.pos.move_y(-*val);
                }
            },
        }
    }

    fn get_manhattan_distance(&self) -> u64 {
        self.pos.get_manhattan_distance()
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
