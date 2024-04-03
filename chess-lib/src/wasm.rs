use std::fmt::Display;

use crate::{
    board::Board,
    bot::{v1, BotTrait},
    console_log,
    game::Game,
    movegen::{legal_moves, Move, SpecialMove},
    piece::Piece,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct WasmMove {
    pub from: u8,
    pub to: u8,
    pub capture: Option<u8>,
    pub piece: crate::piece::PieceKind,

    pub promotion: Option<crate::piece::PieceKind>,
    pub castle: Option<CastleMove>,
}

impl Display for WasmMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let from = crate::square::Square::from(self.from as u64);
        let to = crate::square::Square::from(self.to as u64);
        let promotion = self
            .promotion
            .map(|p| format!("={}", p.letter()))
            .unwrap_or("".to_string());

        write!(f, "{}{}{}", from, to, promotion)
    }
}

impl From<&Move> for WasmMove {
    fn from(value: &Move) -> Self {
        Self {
            from: value.from as u8,
            to: value.to as u8,
            capture: if value.capture {
                Some(value.to as u8)
            } else {
                None
            },
            piece: value.piece.kind,

            promotion: match value.special {
                Some(SpecialMove::Promotion(p)) => Some(p),
                _ => None,
            },
            castle: match value.special {
                Some(SpecialMove::Castle) => {
                    if value.to.file() == 6 {
                        Some(CastleMove::KingSide)
                    } else {
                        Some(CastleMove::QueenSide)
                    }
                }
                _ => None,
            },
        }
    }
}

impl From<Move> for WasmMove {
    fn from(value: Move) -> Self {
        Self::from(&value)
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum CastleMove {
    KingSide,
    QueenSide,
}

#[wasm_bindgen]
pub struct WasmGame(Game);

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Game::default())
    }

    pub fn from_server(_move_json: String) -> Self {
        todo!()
    }

    pub fn board(&self) -> WasmBoard {
        WasmBoard(self.0.board.clone())
    }

    pub fn side_to_move(&self) -> crate::piece::Color {
        self.0.board.side_to_move
    }

    pub fn game_state(&self) -> GameState {
        self.0.game_state
    }

    pub fn update_state(&mut self) {
        self.0.update_state();
    }

    pub fn legal_moves(&self) -> Vec<WasmMove> {
        legal_moves(&self.0.board).iter().map(Into::into).collect()
    }

    pub fn make_move(&mut self, mv: WasmMove) {
        let promotion_move = |m1: &WasmMove, m2: &Move| match (m1.promotion, m2.special) {
            (Some(p1), Some(SpecialMove::Promotion(p2))) => p1 == p2,
            (_, _) => true,
        };

        let legal_moves = legal_moves(&self.0.board);
        let mv = legal_moves
            .into_iter()
            .find(|m| m.from as u8 == mv.from && m.to as u8 == mv.to && promotion_move(&mv, m))
            .expect("Failed to transform wasm move to lib move");
        self.0.make_move(*mv);
    }

    pub fn move_history(&self) -> Vec<WasmMove> {
        self.0.moves().iter().map(Into::into).collect()
    }

    pub fn moves_server(&self) -> String {
        String::new()
    }

    pub fn board_at(&self, ply: usize) -> WasmBoard {
        WasmBoard(self.0.board_history[ply].clone())
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    InProgress,
    Checkmate,
    Stalemate,
    DrawByRepetition,
    DrawByFiftyMoveRule,
    DrawByInsufficientMaterial,
}

#[wasm_bindgen]
pub struct WasmBoard(Board);

#[wasm_bindgen]
impl WasmBoard {
    pub fn print(&self) {
        console_log!("{}", self.0);
    }

    pub fn pieces(&self) -> Vec<PieceWithIndex> {
        let mut pieces: Vec<PieceWithIndex> = Vec::new();
        for i in 0..64 {
            if let Some(p) = self.0.piece(i as u64) {
                pieces.push(PieceWithIndex { index: i, piece: p });
            }
        }
        pieces
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }

    pub fn from_json(json: String) -> Self {
        Self(serde_json::from_str(&json).unwrap())
    }
}

#[wasm_bindgen]
pub struct PieceWithIndex {
    pub index: i32,
    pub piece: Piece,
}

#[derive(Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct BotMove {
    pub m: WasmMove,
    pub nodes_searched: u64,
    pub seconds: f64,
    pub score: i32,
    pub depth: i32,

    pv: Box<[WasmMove]>,
}

#[wasm_bindgen]
impl BotMove {
    pub fn pv(&self) -> String {
        self.pv
            .into_iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
}

#[wasm_bindgen]
pub fn bot_move(board: WasmBoard, depth: i32, tt_size: usize, max_time: u64) -> BotMove {
    console_error_panic_hook::set_once();

    let mut bot = v1::Bot::new(depth, tt_size, max_time);
    let start = js_sys::Date::now();
    let m = bot.make_move(board.0);
    let seconds = (js_sys::Date::now() - start) / 1000.0;

    BotMove {
        m: m.into(),
        nodes_searched: bot.nodes_searched,
        seconds,
        score: bot.score,
        depth: bot.reached_depth,

        pv: bot.pv_table[0]
            .into_iter()
            .take(bot.pv_len[0] - 1)
            .cloned()
            .map(Into::into)
            .collect(),
    }
}
