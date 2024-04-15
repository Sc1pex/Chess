use crate::{
    board::Board,
    bot::Bot,
    console_log,
    game::Game,
    movegen::{legal_moves, Move, SpecialMove},
    piece::Piece,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
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

#[wasm_bindgen]
impl WasmMove {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
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

    pub fn from_server(move_json: String) -> Self {
        console_error_panic_hook::set_once();

        let move_json: String = serde_json::from_str(&move_json).unwrap();
        let moves: Vec<Move> = serde_json::from_str(&move_json).unwrap();
        Self(Game::from_moves(moves))
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
        serde_json::to_string(&self.0.moves()).unwrap()
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
    pub nodes_searched: u64,
    pub score: i32,
    pub depth: i32,
    pub best_move: WasmMove,
}

#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[wasm_bindgen]
pub fn bot_move(board: WasmBoard, difficulty: Difficulty) -> BotMove {
    console_error_panic_hook::set_once();

    let (depth, tt_size, max_time) = match difficulty {
        Difficulty::Easy => (5, 10000000, 3000),
        Difficulty::Medium => (7, 10000000, 3000),
        Difficulty::Hard => (30, 10000000, 3000),
    };

    let mut bot = Bot::new(depth, tt_size, max_time);
    let m = bot.make_move(board.0, difficulty == Difficulty::Hard);

    BotMove {
        best_move: select_move(m, difficulty),
        nodes_searched: bot.nodes_searched,
        score: bot.score,
        depth: bot.reached_depth,
    }
}

fn select_move(moves: Box<[(Move, i32)]>, difficulty: Difficulty) -> WasmMove {
    match difficulty {
        Difficulty::Easy => {
            if moves[0].1 >= 400_000 {
                return moves[0].0.into();
            }

            let score_brackets = [50, 100, 200, 500, 2000];
            let probabilites = [0.35, 0.25, 0.2, 0.15, 0.05];

            let moves_per_bracked: Vec<_> = score_brackets
                .iter()
                .map(|b| {
                    moves
                        .iter()
                        .filter(|(_, s)| moves[0].1 - *s <= *b)
                        .collect::<Vec<_>>()
                })
                .collect();

            loop {
                let mut r: f64 = rand::thread_rng().gen();
                console_log!("Generated {r}");
                let mut bracket = 0;
                for i in 0..probabilites.len() {
                    r -= probabilites[i];
                    bracket = i;
                    if r <= 0. {
                        break;
                    }
                }
                console_log!("Bracket {bracket} - {}", score_brackets[bracket]);

                if !moves_per_bracked[bracket].is_empty() {
                    let idx = rand::thread_rng().gen_range(0..moves_per_bracked[bracket].len());
                    return moves[idx].0.into();
                }
            }
        }
        Difficulty::Medium => {
            if moves[0].1 >= 400_000 {
                return moves[0].0.into();
            }

            let score_brackets = [20, 50, 100, 200, 400];
            let probabilites = [0.5, 0.3, 0.1, 0.08, 0.02];

            let moves_per_bracked: Vec<_> = score_brackets
                .iter()
                .map(|b| {
                    moves
                        .iter()
                        .filter(|(_, s)| moves[0].1 - *s <= *b)
                        .collect::<Vec<_>>()
                })
                .collect();

            loop {
                let mut r: f64 = rand::thread_rng().gen();
                console_log!("Generated {r}");
                let mut bracket = 0;
                for i in 0..probabilites.len() {
                    r -= probabilites[i];
                    bracket = i;
                    if r <= 0. {
                        break;
                    }
                }
                console_log!("Bracket {bracket} - {}", score_brackets[bracket]);

                if !moves_per_bracked[bracket].is_empty() {
                    let idx = rand::thread_rng().gen_range(0..moves_per_bracked[bracket].len());
                    return moves[idx].0.into();
                }
            }
        }
        Difficulty::Hard => moves[0].0.into(),
    }
}
