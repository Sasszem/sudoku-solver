use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::Uint32Array;

mod board_neighbours;

extern crate serde_json;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// TODO: remove all logging (should shrink WASM size)
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub struct Board {
    data: Vec<u8>
}


#[wasm_bindgen]
impl Board {
    pub fn new(js_obj: &JsValue) -> Board {
        let array: Vec<u8> = js_obj.into_serde().unwrap();
        assert!(array.len()==81, "Mismatched array size");

        return Board{
            data: array
        }
    }

    pub fn validate_square(&self, row: u8, column: u8) -> bool {
        let idx = board_neighbours::to_index(row, column);
        let val = self.data[idx];
        return !board_neighbours::neighbours(row, column).any(|x| self.data[x]==val && self.data[x]!=0);
    }

    pub fn validate_all(&self) -> Uint32Array {
        let mut x: Vec<u8> = vec![];
        for row in 0..9 {
            for column in 0..9 {
                if !self.validate_square(row, column) {
                    x.push(9*row + column);
                }
            }
        }

        let y: Vec<u32> = x.iter().map(|&x| x as u32).collect();
        return Uint32Array::from(&y[..]);
    }

    pub fn to_array(&self) -> Uint32Array {
        let y: Vec<u32> = self.data.iter().map(|&x| x as u32).collect();
        return Uint32Array::from(&y[..]);
    }

    pub fn set(&mut self, index: usize, val: u8) {
        self.data[index] = val;
    }

    pub fn get(&self, index: usize) -> u8 {
        self.data[index]
    }
}

#[derive(Clone, Copy)]
struct BoardSolvingEntry {
    pub possible: [bool; 9],
    pub finished: bool
}

#[wasm_bindgen]
pub struct BoardSolving {
    data: [BoardSolvingEntry; 81]
}

#[wasm_bindgen]
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
        
        let index = board_neighbours::to_index(row, column);

        // mark as finished
        self.data[index].finished = true;
        
        // replace [true; 9] with [false; 9] and write one true
        self.data[index].possible = [false; 9];
        self.data[index].possible[usize::from(value-1)] = true;
        
        // update neightbors possibilities
        // and set them if they can be only one possible value
        for i in board_neighbours::neighbours(row, column) {
            if !self.data[i].finished {
                self.data[i].possible[usize::from(value-1)] = false;

                let cnt = self.data[i].possible.into_iter().filter(|&x| *x).count();
                if cnt==1 {
                    let pos = self.data[i].possible.into_iter().position(|&x| x).unwrap() as u8;
                    self.set((i/9) as u8, (i%9) as u8, pos + 1);
                }
            }
        }
    }

    // TODO: necessity solving
    // i.e every row, column and big square needs all numbers
    // so check every row, column and big square
    // and check every numbers' possible positions
    // and if there's only one
    // set it

    // Easy debug log function
    // will remove later, but keep for now
    // TODO: remove this
    fn debug_dump(&self) {
        for i in 0..3 {
            for j in 0..3 {
                let idx = board_neighbours::to_index(i, j);
                log!("{}, {} (idx={})", i, j, idx);
                log!("\tfinished = {}", self.data[idx].finished);
                for k in 0..9 {
                    log!("\t{} -> {}", k, self.data[idx].possible[k]);
                }
            }
        }
    }

    // convert it into a Board so we can return it to JS
    pub fn to_board(&self) -> Board {
        let mut data: Vec<u8> = vec![0; 81];

        for i in 0..81 {
            if self.data[i].possible.into_iter().filter(|&x| *x).count() == 1 {
                data[i] = 1+self.data[i].possible.into_iter().position(|&x| x).unwrap() as u8;
            }
        }

        return Board{
            data
        };
    }

    fn check_only_possible(&mut self, val: u8, iter: impl Iterator<Item=usize>) {
        let row : Vec<BoardSolvingEntry> = iter.map(|x| self.data[x]).collect();
        if row.iter().filter(|x| x.finished).filter(|x| x.possible[usize::from(val)]).count()==1 {
            return;
        }
        if row.iter().filter(|x| x.possible[usize::from(val)]).count()==1 {
            self.set(0, row.iter().position(|x| x.possible[usize::from(val)]).unwrap() as u8, (val+1) as u8);
        }
    }

    pub fn solve_step(&mut self) {
        for i in 0..9 {
            for r in 0..9 {
                self.check_only_possible(i, board_neighbours::row_indexes(r));
                self.check_only_possible(i, board_neighbours::column_indexes(r));
                self.check_only_possible(i, board_neighbours::square_indexes(r/3, r%3));
            }
        }
    }

    // set up numbers from a Board
    // so we can just create it easily from JS
    pub fn from_board(b: Board) -> BoardSolving {
        let mut bs = BoardSolving::new();
        for i in 0..81 {
            bs.set(i/9, i%9, b.get(i.into()));
        }
        return bs;
    }
}

// TODO: create wrappers around objects so we can just call *pure* functions from JS