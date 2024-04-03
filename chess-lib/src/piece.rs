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
