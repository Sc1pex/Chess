use crate::bitboardindex::BitBoardIdx;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use wasm_bindgen::prelude::wasm_bindgen;

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, tsify::Tsify)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

#[wasm_bindgen]
impl Square {
    pub fn file(self) -> u64 {
        self as u64 % 8
    }

    pub fn rank(self) -> u64 {
        self as u64 / 8
    }
}

#[wasm_bindgen]
pub fn square_from_num(value: u32) -> Square {
    match value {
        0..=63 => unsafe { std::mem::transmute(value as u8) },
        _ => panic!("Invalid square index"),
    }
}

#[wasm_bindgen]
pub fn square_to_num(s: Square) -> u32 {
    s as u32
}

impl From<u64> for Square {
    fn from(value: u64) -> Self {
        match value {
            0..=63 => unsafe { std::mem::transmute(value as u8) },
            _ => panic!("Invalid square index"),
        }
    }
}

impl BitBoardIdx for Square {
    fn idx(self) -> u64 {
        self as u64
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            (self.file() as u8 + b'a') as char,
            self.rank() + 1
        )
    }
}
