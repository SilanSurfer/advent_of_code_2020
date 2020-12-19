use std::convert::TryInto;

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const SIN_COS_VALS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub fn new() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn new_at(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn move_x(&mut self, val: i32) {
        self.x += val;
    }

    pub fn move_y(&mut self, val: i32) {
        self.y += val;
    }

    pub fn get_x(&self) -> &i32 {
        &self.x
    }

    pub fn get_y(&self) -> &i32 {
        &self.y
    }

    pub fn rotate(&mut self, angle: i32) {
        let norm_angle = (angle as usize / 90) % 4;
        let result = (
            self.x * Point::SIN_COS_VALS[norm_angle].1 - self.y * Point::SIN_COS_VALS[norm_angle].0,
            self.x * Point::SIN_COS_VALS[norm_angle].0 + self.y * Point::SIN_COS_VALS[norm_angle].1,
        );
        self.x = result.0;
        self.y = result.1;
    }

    pub fn get_manhattan_distance(&self) -> u64 {
        (self.x.abs() + self.y.abs())
            .try_into()
            .expect("Couldn't conver i32 to u64!")
    }
}
