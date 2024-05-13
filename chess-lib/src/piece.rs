use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceKind {
    pub fn letter(&self) -> &'static str {
        match self {
            Self::Pawn => "p",
            Self::Knight => "n",
            Self::Bishop => "b",
            Self::Rook => "r",
            Self::Queen => "q",
            Self::King => "k",
        }
    }
}

impl From<u32> for PieceKind {
    fn from(value: u32) -> Self {
        match value {
            0..=5 => unsafe { std::mem::transmute(value as u8) },
            _ => panic!("Invalid square index {}", value),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum Color {
    #[default]
    White,
    Black,
}

#[wasm_bindgen]
pub fn opposite_color(color: Color) -> Color {
    color.opposite()
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

#[wasm_bindgen]
impl Piece {
    #[wasm_bindgen(constructor)]
    pub fn new(kind: PieceKind, color: Color) -> Self {
        Self { kind, color }
    }
}

impl Piece {
    pub fn to_bits(self) -> u32 {
        let b = if self.color == Color::White {
            0b1000
        } else {
            0b0
        };
        let k = self.kind as u32;
        b | k
    }

    pub fn from_bits(bits: u32) -> Self {
        let color = if (bits & 0b1000) != 0 {
            Color::White
        } else {
            Color::Black
        };
        let kind: PieceKind = (bits & 0b111).into();

        Self { color, kind }
    }
}
