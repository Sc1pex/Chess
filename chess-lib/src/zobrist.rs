use crate::{
    board::Board,
    piece::{Color, Piece},
};
use rand::Rng;

pub struct Zobrist {
    pieces: [[u64; 64]; 12],
    castling: [u64; 16],
    en_passant: [u64; 8],
    side: u64,
}

fn piece_idx(p: Piece) -> usize {
    let p = p.kind as usize;
    p
}

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut pieces = [[0; 64]; 12];
        for p in 0..12 {
            for s in 0..64 {
                pieces[p][s] = rng.gen();
            }
        }
        let mut castling = [0; 16];
        for i in 0..16 {
            castling[i] = rng.gen();
        }
        let mut en_passant = [0; 8];
        for s in 0..8 {
            en_passant[s] = rng.gen();
        }
        let side = rng.gen();

        Self {
            pieces,
            castling,
            en_passant,
            side,
        }
    }

    pub fn hash(&self, board: &Board) -> u64 {
        let mut hash = 0;

        for s in 0..64 {
            if let Some(p) = board.piece(s) {
                let p = piece_idx(p);
                hash ^= self.pieces[p][s as usize];
            }
        }
        hash ^= self.castling[board.can_castle as usize];
        if let Some(s) = board.en_passant {
            hash ^= self.en_passant[s.file() as usize];
        }
        if board.side_to_move == Color::Black {
            hash ^= self.side;
        }

        hash
    }
}
