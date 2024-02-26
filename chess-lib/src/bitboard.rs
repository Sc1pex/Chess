use crate::bitboardindex::BitBoardIdx;
use serde::{Deserialize, Serialize};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Neg, Not, Shl, Shr, Sub};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn set(&mut self, bit: impl BitBoardIdx) {
        self.0 |= 1 << (bit.idx());
    }

    pub fn get(&self, bit: impl BitBoardIdx) -> bool {
        (self.0 >> (bit.idx())) & 1 == 1
    }

    pub fn clear(&mut self, bit: impl BitBoardIdx) {
        self.0 &= !(1 << (bit.idx()));
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}
impl BitOr<u64> for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 | rhs)
    }
}
impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
impl BitOrAssign<u64> for BitBoard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl Shl<u64> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}
impl Shr<u64> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}
impl BitAnd<u64> for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 & rhs)
    }
}
impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
impl BitAndAssign<u64> for BitBoard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs;
    }
}

impl Sub for BitBoard {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 - rhs.0)
    }
}
impl Sub<u64> for BitBoard {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 - rhs)
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl Neg for BitBoard {
    type Output = Self;

    fn neg(self) -> Self::Output {
        BitBoard(self.0.wrapping_neg())
    }
}

impl PartialEq<u64> for BitBoard {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..8 {
            write!(f, "  {}  ", 8 - i)?;
            for j in 0..8 {
                let idx = (7 - i) * 8 + j;
                if self.get(idx) {
                    write!(f, "1 ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "\n     a b c d e f g h")
    }
}

pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = FILE_A << 1;
pub const FILE_C: u64 = FILE_A << 2;
pub const FILE_D: u64 = FILE_A << 3;
pub const FILE_E: u64 = FILE_A << 4;
pub const FILE_F: u64 = FILE_A << 5;
pub const FILE_G: u64 = FILE_A << 6;
pub const FILE_H: u64 = FILE_A << 7;

pub const RANK_1: u64 = 0xFF;
pub const RANK_2: u64 = RANK_1 << 8;
pub const RANK_3: u64 = RANK_1 << 16;
pub const RANK_4: u64 = RANK_1 << 24;
pub const RANK_5: u64 = RANK_1 << 32;
pub const RANK_6: u64 = RANK_1 << 40;
pub const RANK_7: u64 = RANK_1 << 48;
pub const RANK_8: u64 = RANK_1 << 56;

pub const FILE_AB: u64 = FILE_A | FILE_B;
pub const FILE_GH: u64 = FILE_G | FILE_H;
pub const RANK_12: u64 = RANK_1 | RANK_2;
pub const RANK_78: u64 = RANK_7 | RANK_8;

// pub const NOT_FILE_A: u64 = !FILE_A;
// pub const NOT_FILE_H: u64 = !FILE_H;
// pub const NOT_FILE_AB: u64 = !(FILE_A | FILE_B);
// pub const NOT_FILE_GH: u64 = !(FILE_G | FILE_H);
