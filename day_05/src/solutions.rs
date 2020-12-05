use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct BoardingSeatError(String);

impl fmt::Display for BoardingSeatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct BoardingSeat(u64);

impl FromStr for BoardingSeat {
    type Err = BoardingSeatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row_no: Vec<u32> = (0..128).collect();
        let mut col_no: Vec<u32> = (0..8).collect();
        let mut split_point = row_no.len();
        for (_, letter) in s.chars().enumerate().take_while(|(no, _)| no < &7) {
            split_point /= 2;
            let (left, right) = row_no.split_at(split_point);
            match letter {
                'F' => row_no = left.to_vec(),
                'B' => row_no = right.to_vec(),
                _ => {}
            }
        }

        split_point = col_no.len();
        for (_, letter) in s.chars().enumerate().skip(7) {
            split_point /= 2;
            let (left, right) = col_no.split_at(split_point);
            match letter {
                'R' => col_no = right.to_vec(),
                'L' => col_no = left.to_vec(),
                _ => {}
            }
        }
        let row = row_no.pop().unwrap();
        let col = col_no.pop().unwrap();
        Ok(BoardingSeat(row as u64 * 8 + col as u64))
    }
}

fn get_sorted_seat_ids(lines: &Vec<&str>) -> Vec<BoardingSeat> {
    let mut ids: Vec<BoardingSeat> = lines
        .iter()
        .map(|line| match BoardingSeat::from_str(line) {
            Ok(boarding_pass) => boarding_pass,
            Err(e) => panic!(format!("Error {:?}", e)),
        })
        .collect();
    ids.sort();
    ids
}

pub fn get_highest_seat_id(lines: &Vec<&str>) -> u64 {
    get_sorted_seat_ids(lines)
        .iter()
        .max()
        .expect("Couldn't get max value").0
}

pub fn get_your_seat_id(lines: &Vec<&str>) -> u64 {
    let seats = get_sorted_seat_ids(lines);
    seats
        .iter()
        .zip(seats.iter().skip(1))
        .find(|(prev, next)| (**next).0 - (**prev).0 == 2)
        .expect("Couldn't find seat!")
        .0.0
        + 1
}
