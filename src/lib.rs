use wasm_bindgen::prelude::*;

mod board_neighbours;
mod board;
mod board_solving;


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
    Ok(())
}

static mut BOARD: board::Board = board::Board::new();

#[wasm_bindgen]
pub fn clear() {
    unsafe {
        BOARD.clear();
    }
}

#[wasm_bindgen]
pub fn set(idx: u8, val: u8) {
    unsafe {
        BOARD.set(usize::from(idx), val);
    }
}

#[wasm_bindgen]
pub fn get(idx: u8) -> u8 {
    unsafe {
        return BOARD.get(usize::from(idx));
    }
}

#[wasm_bindgen]
pub fn validate() {
    unsafe {
        BOARD.validate_all();
    }
}

#[wasm_bindgen]
pub fn get_error(idx: u8) -> bool {
    unsafe {
        return BOARD.get_error(usize::from(idx));
    }
}

#[wasm_bindgen]
pub fn solve_step() {
    unsafe {
        let mut bs = board_solving::BoardSolving::from_board(&BOARD);
        bs.solve_step();
        bs.write_board(&mut BOARD);
    }
}