use crate::{
    board::Board,
    movegen::{legal_moves, Move},
};
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bot_move(board: &Board) -> Move {
    // calculate();
    *legal_moves(board).choose(&mut rand::thread_rng()).unwrap()
}

fn calculate() {
    let mut _x: u64 = 0;
    for i in 0..10000 {
        for j in 0..2500 {
            let y = rand::random::<u64>();
            if y % 2 == 0 {
                _x += i - j;
            }
        }
    }
}
