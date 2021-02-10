pub fn to_index(row: u8, column: u8) -> usize {
    assert!(row<=9, "Row ({}) is out of range!", row);
    assert!(column<=9, "Column ({}) is out of range!", column);
    return usize::from(9*row + column);
}

struct Neighbours {
    row: u8,
    column: u8,
    current: u8 
}

impl Neighbours {
    pub fn new(row: u8, column: u8) -> Neighbours {
        Neighbours{
            row,
            column,
            current: 0
        }
    }
}

impl Iterator for Neighbours {
    type Item = usize;

    fn next(&mut self) ->Option<Self::Item> {
        if self.current==81 {
            self.current = self.current + 1;
            None
        } else {
            let m = self.current / 9;
            let n = self.current % 9;
            self.current =  self.current + 1;
            match m {
                0 => {
                    Some(to_index(self.row, n))
                }
                1 => {
                    Some(to_index(n, self.column))
                }
                2 => {
                    let base_row = (self.row/3) * 3;
                    let base_column = (self.column/3) * 3;
                    let new_row = base_row + n/3;
                    let new_column = base_column + n%3;
                    Some(to_index(new_row, new_column))
                }
                _ => None
            }
        }
    }
}

pub fn neighbours(row: u8, column: u8) -> impl Iterator<Item = usize>{
    let idx = to_index(row, column);
    return Neighbours::new(row, column).filter(move |&x| x!= idx);
}