use crate::board_neighbours::to_index;
use crate::board_neighbours::neighbours;

pub struct Board {
    data: [u8; 81],
    problems: [bool;81]
}


impl Board {
    pub const fn new() -> Board {
        return Board{
            data: [0; 81],
            problems: [false;81]
        }
    }

    pub fn from_array(data: [u8; 81]) -> Board {
        return Board{
            data,
            problems: [false; 81]
        }
    }

    pub fn validate_square(&self, row: u8, column: u8) -> bool {
        let idx = to_index(row, column);
        let val = self.data[idx];
        return !neighbours(row, column).any(|x| self.data[x]==val && self.data[x]!=0);
    }

    pub fn validate_all(&mut self) {
        self.problems = [false; 81];
        for row in 0..9 {
            for column in 0..9 {
                if !self.validate_square(row, column) {
                    self.problems[usize::from(9*row + column)] = true;
                }
            }
        }
    }

    pub fn set(&mut self, index: usize, val: u8) {
        self.data[index] = val;
    }

    pub fn get(&self, index: usize) -> u8 {
        self.data[index]
    }

    pub fn clear(&mut self) {
        self.data = [0;81];
    }

    pub fn get_error(&self, idx: usize) -> bool {
        return self.problems[idx];
    }
}