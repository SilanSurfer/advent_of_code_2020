use std::str::FromStr;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
struct BoardingPassError(String);

impl fmt::Display for BoardingPassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq)]
struct BoardingPass {
    row: u32,
    col: u32,
    id: u64,
}

impl Ord for BoardingPass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for BoardingPass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BoardingPass {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


impl FromStr for BoardingPass {
    type Err = BoardingPassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row_val: u32;
        let mut col_val: u32;
        let row = &s[0..7];
        let mut row_no: Vec<u32> = (0..128).collect();
        let mut col_no: Vec<u32> = (0..8).collect();
        let mut split_point = row_no.len();
        for letter in row.chars() {
            split_point /= 2;
            let (left, right) = row_no.split_at(split_point);
            match letter {
                'F' => {
                    row_no = left.to_vec();
                }
                'B' => {
                    row_no = right.to_vec();
                }
                _ => {
                    return Err(BoardingPassError(String::from("Found unexpected letter when searching for row")));
                }
            }
        }
        if row_no.len() == 1 {
            row_val = row_no.pop().ok_or(BoardingPassError(String::from("Couldn't extract row value")))?;
        } else {
            return Err(BoardingPassError(String::from("Row vector should have size 1")));
        }

        split_point = col_no.len();
        let col = &s[7..];
        for letter in col.chars() {
            split_point /= 2;
            let (left, right) = col_no.split_at(split_point);
            match letter {
                'R' => {
                    col_no = right.to_vec();
                }
                'L' => {
                    col_no = left.to_vec();
                }
                _ => {
                    return Err(BoardingPassError(String::from("Found unexpected letter when searching for col")));
                }
            }
        }
        if col_no.len() == 1 {
            col_val = col_no.pop().ok_or(BoardingPassError(String::from("Couldn't extract col value")))?;
        } else {
            return Err(BoardingPassError(String::from("Col vector should have size 1")));
        }
        Ok(BoardingPass {
            row: row_val,
            col: col_val,
            id: row_val as u64 * 8 + col_val as u64,
        })
    }
}

pub fn get_ordered_seat_ids(lines: &Vec<&str>) -> Vec<u64> {
    let mut results: Vec<BoardingPass> = Vec::new();
    for line in lines {
        match BoardingPass::from_str(line) {
            Ok(boarding_pass) => { 
                results.push(boarding_pass);
            }
            Err(e) => println!("Error from_str {:?}", e), 
        }
    }
    let mut ids: Vec<u64> = results.iter().map(|boarding_pass| boarding_pass.id).collect();
    ids.sort();
    ids
}

pub fn get_highest_seat_id(lines: &Vec<&str>) -> u64 {
    *get_ordered_seat_ids(lines).iter().max().expect("Something went wrong when searching for max")
}
