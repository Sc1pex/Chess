use crate::{
    bitboard::BitBoard,
    movegen::{square_attacked, Move, SpecialMove},
    piece::*,
    square::Square,
};
use serde::{Deserialize, Serialize};
pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Board {
    pub w_pawn: BitBoard,
    pub w_knight: BitBoard,
    pub w_bishop: BitBoard,
    pub w_rook: BitBoard,
    pub w_queen: BitBoard,
    pub w_king: BitBoard,
    pub b_pawn: BitBoard,
    pub b_knight: BitBoard,
    pub b_bishop: BitBoard,
    pub b_rook: BitBoard,
    pub b_queen: BitBoard,
    pub b_king: BitBoard,

    pub w_occ: BitBoard,
    pub b_occ: BitBoard,
    pub occ: BitBoard,

    pub side_to_move: Color,
    pub in_check: bool,

    pub(crate) en_passant: Option<Square>,
    pub(crate) can_castle: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Castle {
    WhiteKing = 0b0001,
    WhiteQueen = 0b0010,
    BlackKing = 0b0100,
    BlackQueen = 0b1000,
}

impl Board {
    pub fn make_move(&mut self, m: &Move) -> bool {
        let bb = self.board_mut(m.piece);
        bb.clear(m.from);
        bb.set(m.to);
        self.en_passant = None;

        if m.capture {
            for b in self.boards_color(self.side_to_move.opposite()) {
                if b.get(m.to) {
                    b.clear(m.to);
                    break;
                }
            }

            if self.side_to_move == Color::White {
                if m.to == Square::H8 {
                    self.can_castle &= !(Castle::BlackKing as u8)
                } else if m.to == Square::A8 {
                    self.can_castle &= !(Castle::BlackQueen as u8)
                }
            } else if m.to == Square::H1 {
                self.can_castle &= !(Castle::WhiteKing as u8)
            } else if m.to == Square::A1 {
                self.can_castle &= !(Castle::WhiteQueen as u8)
            }
        }
        match m.special {
            Some(SpecialMove::DoublePawnPush) => {
                self.en_passant = Some(if self.side_to_move == Color::White {
                    Square::from(m.to as u64 - 8)
                } else {
                    Square::from(m.to as u64 + 8)
                })
            }
            Some(SpecialMove::EnPassant) => {
                if self.side_to_move == Color::White {
                    self.b_pawn.clear(m.to as u64 - 8)
                } else {
                    self.w_pawn.clear(m.to as u64 + 8)
                }
            }
            Some(SpecialMove::Promotion(p)) => {
                let b = self.boards_color(self.side_to_move);
                b[0].clear(m.to);
                match p {
                    PieceKind::Knight => b[1].set(m.to),
                    PieceKind::Bishop => b[2].set(m.to),
                    PieceKind::Rook => b[3].set(m.to),
                    PieceKind::Queen => b[4].set(m.to),
                    _ => unreachable!(),
                }
            }
            Some(SpecialMove::Castle) => match m.to {
                Square::G1 => {
                    self.w_rook.clear(Square::H1);
                    self.w_rook.set(Square::F1);
                }
                Square::C1 => {
                    self.w_rook.clear(Square::A1);
                    self.w_rook.set(Square::D1);
                }
                Square::G8 => {
                    self.b_rook.clear(Square::H8);
                    self.b_rook.set(Square::F8);
                }
                Square::C8 => {
                    self.b_rook.clear(Square::A8);
                    self.b_rook.set(Square::D8);
                }
                _ => unreachable!(),
            },
            _ => (),
        }

        if m.piece.kind == PieceKind::King {
            match self.side_to_move {
                Color::White => {
                    self.can_castle &= !(Castle::WhiteQueen as u8 | Castle::WhiteKing as u8)
                }
                Color::Black => {
                    self.can_castle &= !(Castle::BlackQueen as u8 | Castle::BlackKing as u8)
                }
            }
        }
        if m.piece.kind == PieceKind::Rook {
            match self.side_to_move {
                Color::White => {
                    if m.from == Square::A1 {
                        self.can_castle &= !(Castle::WhiteQueen as u8)
                    }
                    if m.from == Square::H1 {
                        self.can_castle &= !(Castle::WhiteKing as u8)
                    }
                }
                Color::Black => {
                    if m.from == Square::A8 {
                        self.can_castle &= !(Castle::BlackQueen as u8)
                    }
                    if m.from == Square::H8 {
                        self.can_castle &= !(Castle::BlackKing as u8)
                    }
                }
            }
        }
        self.update_occ();

        let king = self.boards_color(self.side_to_move)[5].0.trailing_zeros() as u64;
        if square_attacked(self, king, self.side_to_move.opposite()) {
            return false;
        }

        let king = self.boards_color(self.side_to_move.opposite())[5]
            .0
            .trailing_zeros() as u64;
        self.in_check = square_attacked(self, king, self.side_to_move);

        self.side_to_move = self.side_to_move.opposite();

        true
    }

    pub(crate) fn board_mut(&mut self, piece: Piece) -> &mut BitBoard {
        match piece.color {
            Color::White => match piece.kind {
                PieceKind::Pawn => &mut self.w_pawn,
                PieceKind::Knight => &mut self.w_knight,
                PieceKind::Bishop => &mut self.w_bishop,
                PieceKind::Rook => &mut self.w_rook,
                PieceKind::Queen => &mut self.w_queen,
                PieceKind::King => &mut self.w_king,
            },
            Color::Black => match piece.kind {
                PieceKind::Pawn => &mut self.b_pawn,
                PieceKind::Knight => &mut self.b_knight,
                PieceKind::Bishop => &mut self.b_bishop,
                PieceKind::Rook => &mut self.b_rook,
                PieceKind::Queen => &mut self.b_queen,
                PieceKind::King => &mut self.b_king,
            },
        }
    }

    pub(crate) fn board(&self, piece: Piece) -> &BitBoard {
        match piece.color {
            Color::White => match piece.kind {
                PieceKind::Pawn => &self.w_pawn,
                PieceKind::Knight => &self.w_knight,
                PieceKind::Bishop => &self.w_bishop,
                PieceKind::Rook => &self.w_rook,
                PieceKind::Queen => &self.w_queen,
                PieceKind::King => &self.w_king,
            },
            Color::Black => match piece.kind {
                PieceKind::Pawn => &self.b_pawn,
                PieceKind::Knight => &self.b_knight,
                PieceKind::Bishop => &self.b_bishop,
                PieceKind::Rook => &self.b_rook,
                PieceKind::Queen => &self.b_queen,
                PieceKind::King => &self.b_king,
            },
        }
    }

    pub(crate) fn boards_color(&mut self, color: Color) -> [&mut BitBoard; 6] {
        match color {
            Color::White => [
                &mut self.w_pawn,
                &mut self.w_knight,
                &mut self.w_bishop,
                &mut self.w_rook,
                &mut self.w_queen,
                &mut self.w_king,
            ],
            Color::Black => [
                &mut self.b_pawn,
                &mut self.b_knight,
                &mut self.b_bishop,
                &mut self.b_rook,
                &mut self.b_queen,
                &mut self.b_king,
            ],
        }
    }
}

impl Board {
    pub fn start_pos() -> Board {
        Self::from_fen(DEFAULT_FEN)
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut s = Self::default();

        let mut parts = fen.split_whitespace();

        // Piece placement
        let board = parts.next().unwrap();
        for (i, l) in board.split('/').enumerate() {
            let i = 7 - i;
            let mut j = 0;
            for c in l.chars() {
                let idx = (i * 8 + j) as u64;
                match c {
                    '1'..='8' => j += c.to_digit(10).unwrap() as usize - 1,
                    'p' => s.b_pawn.set(idx),
                    'n' => s.b_knight.set(idx),
                    'b' => s.b_bishop.set(idx),
                    'r' => s.b_rook.set(idx),
                    'q' => s.b_queen.set(idx),
                    'k' => s.b_king.set(idx),
                    'P' => s.w_pawn.set(idx),
                    'N' => s.w_knight.set(idx),
                    'B' => s.w_bishop.set(idx),
                    'R' => s.w_rook.set(idx),
                    'Q' => s.w_queen.set(idx),
                    'K' => s.w_king.set(idx),
                    _ => panic!("Invalid FEN"),
                }
                j += 1;
            }
        }

        // Side to move
        match parts.next().unwrap() {
            "w" => s.side_to_move = Color::White,
            "b" => s.side_to_move = Color::Black,
            _ => panic!("Invalid FEN"),
        };

        // Castling availability
        let castling = parts.next().unwrap();
        for c in castling.chars() {
            match c {
                'K' => s.can_castle |= Castle::WhiteKing as u8,
                'Q' => s.can_castle |= Castle::WhiteQueen as u8,
                'k' => s.can_castle |= Castle::BlackKing as u8,
                'q' => s.can_castle |= Castle::BlackQueen as u8,
                '-' => (),
                _ => panic!("Invalid FEN"),
            }
        }

        // En passant target square
        let en_passant = parts.next().unwrap();
        if en_passant != "-" {
            let file = en_passant.chars().nth(0).unwrap() as u8 - b'a';
            let rank = en_passant.chars().nth(1).unwrap() as u8 - b'1';
            s.en_passant = Some(Square::from((rank * 8 + file) as u64));
        }

        // Halfmove clock
        // Fullmove number

        s.update_occ();
        s
    }
}

impl Board {
    pub fn get(&self, idx: u64) -> bool {
        self.w_pawn.get(idx)
            || self.w_knight.get(idx)
            || self.w_bishop.get(idx)
            || self.w_rook.get(idx)
            || self.w_queen.get(idx)
            || self.w_king.get(idx)
            || self.b_pawn.get(idx)
            || self.b_knight.get(idx)
            || self.b_bishop.get(idx)
            || self.b_rook.get(idx)
            || self.b_queen.get(idx)
            || self.b_king.get(idx)
    }

    pub fn piece(&self, idx: u64) -> Option<Piece> {
        if self.w_pawn.get(idx) {
            return Some(Piece::new(PieceKind::Pawn, Color::White));
        }
        if self.w_knight.get(idx) {
            return Some(Piece::new(PieceKind::Knight, Color::White));
        }
        if self.w_bishop.get(idx) {
            return Some(Piece::new(PieceKind::Bishop, Color::White));
        }
        if self.w_rook.get(idx) {
            return Some(Piece::new(PieceKind::Rook, Color::White));
        }
        if self.w_queen.get(idx) {
            return Some(Piece::new(PieceKind::Queen, Color::White));
        }
        if self.w_king.get(idx) {
            return Some(Piece::new(PieceKind::King, Color::White));
        }
        if self.b_pawn.get(idx) {
            return Some(Piece::new(PieceKind::Pawn, Color::Black));
        }
        if self.b_knight.get(idx) {
            return Some(Piece::new(PieceKind::Knight, Color::Black));
        }
        if self.b_bishop.get(idx) {
            return Some(Piece::new(PieceKind::Bishop, Color::Black));
        }
        if self.b_rook.get(idx) {
            return Some(Piece::new(PieceKind::Rook, Color::Black));
        }
        if self.b_queen.get(idx) {
            return Some(Piece::new(PieceKind::Queen, Color::Black));
        }
        if self.b_king.get(idx) {
            return Some(Piece::new(PieceKind::King, Color::Black));
        }
        None
    }
}

impl Board {
    fn update_occ(&mut self) {
        self.w_occ =
            self.w_pawn | self.w_knight | self.w_bishop | self.w_rook | self.w_queen | self.w_king;
        self.b_occ =
            self.b_pawn | self.b_knight | self.b_bishop | self.b_rook | self.b_queen | self.b_king;
        self.occ = self.w_occ | self.b_occ;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..8 {
            write!(f, "  {}  ", 8 - i)?;
            for j in 0..8 {
                let idx = (7 - i) * 8 + j;
                if self.get(idx) {
                    let piece = self.piece(idx).unwrap();
                    let c = match piece.color {
                        Color::White => match piece.kind {
                            PieceKind::Pawn => '♙',
                            PieceKind::Knight => '♘',
                            PieceKind::Bishop => '♗',
                            PieceKind::Rook => '♖',
                            PieceKind::Queen => '♕',
                            PieceKind::King => '♔',
                        },
                        Color::Black => match piece.kind {
                            PieceKind::Pawn => '♟',
                            PieceKind::Knight => '♞',
                            PieceKind::Bishop => '♝',
                            PieceKind::Rook => '♜',
                            PieceKind::Queen => '♛',
                            PieceKind::King => '♚',
                        },
                    };
                    write!(f, "{} ", c)?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "\n     a b c d e f g h")?;
        writeln!(f, "  Side to move: {:?}", self.side_to_move)?;
        writeln!(f, "  En passant: {:?}", self.en_passant.map(Square::from))?;
        write!(f, "  Can castle: ")?;
        [
            (Castle::WhiteKing, "K"),
            (Castle::WhiteQueen, "Q"),
            (Castle::BlackKing, "k"),
            (Castle::BlackQueen, "q"),
        ]
        .iter()
        .for_each(|(c, s)| {
            if self.can_castle & *c as u8 != 0 {
                write!(f, "{}", s).unwrap();
            } else {
                write!(f, "-").unwrap();
            }
        });
        writeln!(f)
    }
}
