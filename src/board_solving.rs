use crate::board_neighbours::to_index;
use crate::board::Board;

#[derive(Clone, Copy)]
struct BoardSolvingEntry {
    pub possible: [bool; 9],
    pub finished: bool
}

pub struct BoardSolving {
    data: [BoardSolvingEntry; 81]
}

impl BoardSolving {
    pub fn new() -> BoardSolving {
        return BoardSolving {
            data: [BoardSolvingEntry{possible: [true; 9], finished: false}; 81]
        }
    }

    pub fn set(&mut self, row: u8, column: u8, value: u8) {
        // 0 has a special meaning (unknown)
        if value==0 {
            return;
        }
        
        let index = to_index(row, column);

        // mark as finished
        self.data[index].finished = true;
        
        // replace [true; 9] with [false; 9] and write one true
        self.data[index].possible = [false; 9];
        self.data[index].possible[usize::from(value-1)] = true;
        
        // update neightbors possibilities
        // and set them if they can be only one possible value
        for i in crate::board_neighbours::neighbours(row, column) {
            if !self.data[i].finished {
                self.data[i].possible[usize::from(value-1)] = false;

                let cnt = self.data[i].possible.iter().filter(|&x| *x).count();
                if cnt==1 {
                    let pos = self.data[i].possible.iter().position(|&x| x).unwrap() as u8;
                    self.set((i/9) as u8, (i%9) as u8, pos + 1);
                }
            }
        }
    }

    // convert it into a Board so we can return it to JS
    pub fn write_board(&self, board: &mut Board) {
        board.clear();

        for i in 0..81 {
            if self.data[i].possible.iter().filter(|&x| *x).count() == 1 {
                let v = 1+self.data[i].possible.iter().position(|&x| x).unwrap() as u8;
                board.set(i, v);
            }
        }
    }

    fn check_only_possible(&mut self, val: u8, iter: impl Iterator<Item=usize>) {
        // collect iterator and map into array
        // w/o any dynamic allocation
        let mut indexes: [usize; 9] = [0;9];
        let mut entries: [BoardSolvingEntry; 9] = [BoardSolvingEntry{finished: false, possible:[true; 9]};9];
        let mut p = 0;
        for i in iter {
            indexes[p] = i;
            entries[p] = self.data[i];
            p = p+1;
        }

        // skip if we already have that number
        if entries.iter().filter(|x| x.finished).filter(|x| x.possible[usize::from(val)]).count()==1 {
            return;
        }
        // if there's only one possiblilty, write it
        if entries.iter().filter(|x| x.possible[usize::from(val)]).count()==1 {
            let pos = entries.iter().position(|x| x.possible[usize::from(val)]).unwrap();
            self.set((indexes[pos]/9) as u8, (indexes[pos]%9) as u8, (val+1) as u8);
        }
    }

    pub fn solve_step(&mut self) {
        for i in 0..9 {
            for r in 0..9 {
                self.check_only_possible(i, crate::board_neighbours::row_indexes(r));
                self.check_only_possible(i, crate::board_neighbours::column_indexes(r));
                self.check_only_possible(i, crate::board_neighbours::square_indexes(r/3, r%3));
            }
        }
    }

    // set up numbers from a Board
    // so we can just create it easily from JS
    pub fn from_board(b: &Board) -> BoardSolving {
        let mut bs = BoardSolving::new();
        for i in 0..81 {
            bs.set(i/9, i%9, b.get(i.into()));
        }
        return bs;
    }
}