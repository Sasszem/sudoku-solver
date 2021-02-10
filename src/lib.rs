use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::Uint32Array;

mod board_neighbours;

extern crate serde_json;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
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

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(&format!("Hello, {}!", s));
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

#[wasm_bindgen]
#[derive(Clone)]
pub struct BoardSolving {
    possible: [[bool; 10]; 81]
}

#[wasm_bindgen]
impl BoardSolving {
    pub fn new() -> BoardSolving {
        return BoardSolving {
            possible: [[true; 10]; 81]
        }
    }

    pub fn set(&mut self, row: u8, column: u8, value: u8) -> bool {
        if value==0 {
            return true;
        }
        let idx = board_neighbours::to_index(row, column);
        if self.possible[idx].into_iter().filter(|&x| *x).count() == 1 && !self.possible[idx][usize::from(value)] {
            return false;
        }
        self.possible[idx] = [false;10];
        self.possible[idx][usize::from(value)] = true;
        for i in board_neighbours::neighbours(row, column) {
            self.possible[i][usize::from(value)] = false;
            let c = self.possible[i].into_iter().filter(|&x| *x).count();
            if c==0 {
                return false;
            }
            if c==2 && self.possible[i][9] {
                self.possible[i][0] = false;
                let p = self.possible[i].into_iter().position(|&x| x).unwrap();
                if !self.set((i/9) as u8, (i%9) as u8, p as u8) {
                    return false;
                } 
            }
        }
        return true;
    }

    pub fn to_board(&self) -> Board {
        let mut data: Vec<u8> = vec![0; 81];
        for i in 0..81 {
            if self.possible[i].into_iter().filter(|&x| *x).count() == 1 {
                data[i] = self.possible[i].into_iter().position(|&x| x).unwrap() as u8;
            }
        }

        return Board{
            data
        };
    }

    pub fn from_board(b: Board) -> BoardSolving {
        let mut bs = BoardSolving::new();
        for i in 0..81 {
            bs.set(i/9, i%9, b.get(i.into()));
        }
        return bs;
    }

}