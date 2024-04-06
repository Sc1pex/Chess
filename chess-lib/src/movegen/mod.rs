use self::precalc::PRECALC;
use crate::{
    bitboard::*,
    bitboardindex::BitBoardIdx,
    board::{Board, Castle},
    piece::{Color, Piece, PieceKind},
    square::Square,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod magic;
pub mod precalc;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Move {
    pub(crate) from: Square,
    pub(crate) to: Square,
    pub(crate) piece: Piece,
    pub(crate) capture: bool,
    pub(crate) special: Option<SpecialMove>,
}

impl Move {
    pub fn empty() -> Self {
        Self {
            from: Square::A1,
            to: Square::A1,
            piece: Piece::new(PieceKind::Pawn, Color::White),
            capture: false,
            special: None,
        }
    }
}

impl Move {
    pub fn new(
        from: Square,
        to: Square,
        piece: Piece,
        capture: bool,
        special: Option<SpecialMove>,
    ) -> Self {
        Self {
            from,
            to,
            piece,
            capture,
            special,
        }
    }

    pub fn to_str(&self) -> String {
        self.to_string()
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.from,
            self.to,
            if let Some(SpecialMove::Promotion(p)) = self.special {
                p.letter()
            } else {
                ""
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SpecialMove {
    Promotion(PieceKind),
    EnPassant,
    DoublePawnPush,
    Castle,
}

pub fn legal_moves(board: &Board) -> Box<[Move]> {
    generate_moves(board)
        .into_iter()
        .filter(|m| {
            let mut b = board.clone();
            b.make_move(m)
        })
        .collect()
}

pub fn generate_moves(board: &Board) -> Vec<Move> {
    let mut vmoves = Vec::new();
    vmoves.reserve_exact(220);

    if board.side_to_move == Color::White {
        white_pawn_moves(board, &mut vmoves);
        white_castle(board, &mut vmoves);
        knight_moves(board, board.w_knight, board.w_occ, board.b_occ, &mut vmoves);
        king_moves(board, board.w_king, board.w_occ, board.b_occ, &mut vmoves);
        bishop_moves(board, board.w_bishop, board.b_occ, false, &mut vmoves);
        rook_moves(board, board.w_rook, board.b_occ, false, &mut vmoves);
        queen_moves(board, board.w_queen, board.b_occ, &mut vmoves);
    } else {
        black_pawn_moves(board, &mut vmoves);
        black_castle(board, &mut vmoves);
        knight_moves(board, board.b_knight, board.b_occ, board.w_occ, &mut vmoves);
        king_moves(board, board.b_king, board.b_occ, board.w_occ, &mut vmoves);
        bishop_moves(board, board.b_bishop, board.w_occ, false, &mut vmoves);
        rook_moves(board, board.b_rook, board.w_occ, false, &mut vmoves);
        queen_moves(board, board.b_queen, board.w_occ, &mut vmoves);
    }

    vmoves
}

pub fn queen_moves(board: &Board, queens: BitBoard, o_occ: BitBoard, vmoves: &mut Vec<Move>) {
    rook_moves(board, queens, o_occ, true, vmoves);
    bishop_moves(board, queens, o_occ, true, vmoves);
}

pub fn bishop_moves(
    board: &Board,
    mut bishops: BitBoard,
    o_occ: BitBoard,
    queen: bool,
    vmoves: &mut Vec<Move>,
) {
    while bishops != 0 {
        let from = Square::from(bishops.0.trailing_zeros() as u64);
        let moves = PRECALC.bishop_attack(from, board.occ);
        let mut attacks = moves & o_occ;
        let mut moves = moves & !board.occ;

        while moves != 0 {
            let to = Square::from(moves.0.trailing_zeros() as u64);
            moves &= moves - 1;
            vmoves.push(Move {
                from,
                to,
                capture: false,
                piece: Piece::new(
                    if queen {
                        PieceKind::Queen
                    } else {
                        PieceKind::Bishop
                    },
                    board.side_to_move,
                ),
                special: None,
            });
        }
        while attacks != 0 {
            let to = Square::from(attacks.0.trailing_zeros() as u64);
            attacks &= attacks - 1;
            vmoves.push(Move {
                from,
                to,
                capture: true,
                piece: Piece::new(
                    if queen {
                        PieceKind::Queen
                    } else {
                        PieceKind::Bishop
                    },
                    board.side_to_move,
                ),
                special: None,
            })
        }

        bishops &= bishops - 1;
    }
}

pub fn rook_moves(
    board: &Board,
    mut rooks: BitBoard,
    o_occ: BitBoard,
    queen: bool,
    vmoves: &mut Vec<Move>,
) {
    while rooks != 0 {
        let from = Square::from(rooks.0.trailing_zeros() as u64);
        let moves = PRECALC.rook_attack(from, board.occ);
        let mut attacks = moves & o_occ;
        let mut moves = moves & !board.occ;

        while moves != 0 {
            let to = Square::from(moves.0.trailing_zeros() as u64);
            moves &= moves - 1;
            vmoves.push(Move {
                from,
                to,
                capture: false,
                piece: Piece::new(
                    if queen {
                        PieceKind::Queen
                    } else {
                        PieceKind::Rook
                    },
                    board.side_to_move,
                ),
                special: None,
            });
        }
        while attacks != 0 {
            let to = Square::from(attacks.0.trailing_zeros() as u64);
            attacks &= attacks - 1;
            vmoves.push(Move {
                from,
                to,
                capture: true,
                piece: Piece::new(
                    if queen {
                        PieceKind::Queen
                    } else {
                        PieceKind::Rook
                    },
                    board.side_to_move,
                ),
                special: None,
            })
        }

        rooks &= rooks - 1;
    }
}

pub fn knight_moves(
    board: &Board,
    mut knights: BitBoard,
    c_occ: BitBoard,
    o_occ: BitBoard,
    vmoves: &mut Vec<Move>,
) {
    while knights != 0 {
        let from = Square::from(knights.0.trailing_zeros() as u64);
        let moves = PRECALC.knight[from as usize] & !c_occ;
        let mut attacks = moves & o_occ;
        let mut moves = moves & !o_occ;

        while moves != 0 {
            let to = Square::from(moves.0.trailing_zeros() as u64);
            moves &= moves - 1;
            vmoves.push(Move {
                from,
                to,
                capture: false,
                piece: Piece::new(PieceKind::Knight, board.side_to_move),
                special: None,
            });
        }
        while attacks != 0 {
            let to = Square::from(attacks.0.trailing_zeros() as u64);
            attacks &= attacks - 1;
            vmoves.push(Move {
                from,
                to,
                capture: true,
                piece: Piece::new(PieceKind::Knight, board.side_to_move),
                special: None,
            })
        }

        knights &= knights - 1;
    }
}

pub fn king_moves(
    board: &Board,
    mut king: BitBoard,
    c_occ: BitBoard,
    o_occ: BitBoard,
    vmoves: &mut Vec<Move>,
) {
    while king != 0 {
        let from = Square::from(king.0.trailing_zeros() as u64);
        let moves = PRECALC.king[from as usize] & !c_occ;
        let mut attacks = moves & o_occ;
        let mut moves = moves & !o_occ;

        while moves != 0 {
            let to = Square::from(moves.0.trailing_zeros() as u64);
            moves &= moves - 1;
            vmoves.push(Move {
                from,
                to,
                capture: false,
                piece: Piece::new(PieceKind::King, board.side_to_move),
                special: None,
            });
        }
        while attacks != 0 {
            let to = Square::from(attacks.0.trailing_zeros() as u64);
            attacks &= attacks - 1;
            vmoves.push(Move {
                from,
                to,
                capture: true,
                piece: Piece::new(PieceKind::King, board.side_to_move),
                special: None,
            })
        }

        king &= king - 1;
    }
}

pub fn white_castle(board: &Board, vmoves: &mut Vec<Move>) {
    castle(
        board,
        Castle::WhiteKing,
        &[Square::F1, Square::G1],
        Color::White,
        vmoves,
    );
    castle(
        board,
        Castle::WhiteQueen,
        &[Square::D1, Square::C1, Square::B1],
        Color::White,
        vmoves,
    );
}

pub fn black_castle(board: &Board, vmoves: &mut Vec<Move>) {
    castle(
        board,
        Castle::BlackKing,
        &[Square::F8, Square::G8],
        Color::Black,
        vmoves,
    );
    castle(
        board,
        Castle::BlackQueen,
        &[Square::D8, Square::C8, Square::B8],
        Color::Black,
        vmoves,
    );
}

fn castle(
    board: &Board,
    castle_bit: Castle,
    squares: &[Square],
    color: Color,
    vmoves: &mut Vec<Move>,
) {
    if board.can_castle & castle_bit as u8 != 0 {
        if color == Color::White {
            if square_attacked(board, Square::E1, Color::Black) {
                return;
            }
        } else if square_attacked(board, Square::E8, Color::White) {
            return;
        }

        let squares_empty = squares
            .iter()
            .all(|&s| board.occ & BitBoard(1 << s as u64) == 0);
        let squares_attacked = squares
            .iter()
            .take(2)
            .any(|&s| square_attacked(board, s, color.opposite()));

        if squares_empty && !squares_attacked {
            vmoves.push(Move {
                from: if color == Color::White {
                    Square::E1
                } else {
                    Square::E8
                },
                to: squares[1],
                capture: false,
                piece: Piece::new(PieceKind::King, color),
                special: Some(SpecialMove::Castle),
            });
        }
    }
}

pub fn white_pawn_moves(board: &Board, vmoves: &mut Vec<Move>) {
    let mut pawns = board.w_pawn;
    let occ = board.occ;

    while pawns != 0 {
        let from = Square::from(pawns.0.trailing_zeros() as u64);
        let pawn = BitBoard(1 << pawns.0.trailing_zeros());
        pawns &= pawns - 1;

        let one_rank_up = pawn << 8 & !occ;
        let two_ranks_up = (one_rank_up & RANK_3) << 8 & !occ;
        let mut moves = one_rank_up | two_ranks_up;

        while moves != 0 {
            let to = Square::from(moves.0.trailing_zeros() as u64);
            if to.rank() == 7 {
                vmoves.push(Move {
                    from,
                    to,
                    capture: false,
                    piece: Piece::new(PieceKind::Pawn, Color::White),
                    special: Some(SpecialMove::Promotion(PieceKind::Queen)),
                });
                vmoves.push(Move {
                    from,
                    to,
                    capture: false,
                    piece: Piece::new(PieceKind::Pawn, Color::White),
                    special: Some(SpecialMove::Promotion(PieceKind::Rook)),
                });
                vmoves.push(Move {
                    from,
                    to,
                    capture: false,
                    piece: Piece::new(PieceKind::Pawn, Color::White),
                    special: Some(SpecialMove::Promotion(PieceKind::Bishop)),
                });
                vmoves.push(Move {
                    from,
                    to,
                    capture: false,
                    piece: Piece::new(PieceKind::Pawn, Color::White),
                    special: Some(SpecialMove::Promotion(PieceKind::Knight)),
                });
            } else {
                vmoves.push(Move {
                    from,
                    to,
                    capture: false,
                    piece: Piece::new(PieceKind::Pawn, Color::White),
                    special: if from.rank() == 1 && to.rank() == 3 {
                        Some(SpecialMove::DoublePawnPush)
                    } else {
                        None
                    },
                });
            }
            moves &= moves - 1;
        }

        let mut attacks = PRECALC.pawns[from.idx_usize()][0] & board.b_occ;
        while attacks != 0 {
            let to = Square::from(attacks.0.trailing_zeros() as u64);
            if to.rank() == 7 {
                vmoves.push(Move {
                    from,
                    to,
                    capture: true,
                    piece: Piece::new(PieceKind::Pawn, Color::White),
                    special: Some(SpecialMove::Promotion(PieceKind::Queen)),
                });
                vmoves.push(Move {
                    from,
                    to,
                    capture: true,
                    piece: Piece::new(PieceKind::Pawn, Color::White),
                    special: Some(SpecialMove::Promotion(PieceKind::Rook)),
                });
                vmoves.push(Move {
                    from,
                    to,
                    capture: true,
                    piece: Piece::new(PieceKind::Pawn, Color::White),
                    special: Some(SpecialMove::Promotion(PieceKind::Bishop)),
                });
                vmoves.push(Move {
                    from,
                    to,
                    capture: true,
                    piece: Piece::new(PieceKind::Pawn, Color::White),
                    special: Some(SpecialMove::Promotion(PieceKind::Knight)),
                });
            } else {
                vmoves.push(Move {
                    from,
                    to,
                    capture: true,
                    piece: Piece::new(PieceKind::Pawn, Color::White),
                    special: if from.rank() == 1 && to.rank() == 3 {
                        Some(SpecialMove::DoublePawnPush)
                    } else {
                        None
                    },
                });
            }
            attacks &= attacks - 1;
        }
        if let Some(ep) = board.en_passant {
            let epb = BitBoard(1 << ep as u64);
            if PRECALC.pawns[from.idx_usize()][0] & epb != 0 {
                vmoves.push(Move {
                    from,
                    to: ep,
                    capture: true,
                    piece: Piece::new(PieceKind::Pawn, Color::White),
                    special: Some(SpecialMove::EnPassant),
                });
            }
        }
    }
}

pub fn black_pawn_moves(board: &Board, vmoves: &mut Vec<Move>) {
    let mut pawns = board.b_pawn;
    let occ = board.occ;

    while pawns != 0 {
        let from = Square::from(pawns.0.trailing_zeros() as u64);
        let pawn = BitBoard(1 << pawns.0.trailing_zeros());
        pawns &= pawns - 1;

        let one_rank_up = pawn >> 8 & !occ;
        let two_ranks_up = (one_rank_up & RANK_6) >> 8 & !occ;
        let mut moves = one_rank_up | two_ranks_up;

        while moves != 0 {
            let to = Square::from(moves.0.trailing_zeros() as u64);
            if to.rank() == 0 {
                vmoves.push(Move {
                    from,
                    to,
                    capture: false,
                    piece: Piece::new(PieceKind::Pawn, Color::Black),
                    special: Some(SpecialMove::Promotion(PieceKind::Queen)),
                });
                vmoves.push(Move {
                    from,
                    to,
                    capture: false,
                    piece: Piece::new(PieceKind::Pawn, Color::Black),
                    special: Some(SpecialMove::Promotion(PieceKind::Rook)),
                });
                vmoves.push(Move {
                    from,
                    to,
                    capture: false,
                    piece: Piece::new(PieceKind::Pawn, Color::Black),
                    special: Some(SpecialMove::Promotion(PieceKind::Bishop)),
                });
                vmoves.push(Move {
                    from,
                    to,
                    capture: false,
                    piece: Piece::new(PieceKind::Pawn, Color::Black),
                    special: Some(SpecialMove::Promotion(PieceKind::Knight)),
                });
            } else {
                vmoves.push(Move {
                    from,
                    to,
                    capture: false,
                    piece: Piece::new(PieceKind::Pawn, Color::Black),
                    special: if from.rank() == 6 && to.rank() == 4 {
                        Some(SpecialMove::DoublePawnPush)
                    } else {
                        None
                    },
                });
            }
            moves &= moves - 1;
        }

        let mut attacks = PRECALC.pawns[from.idx_usize()][1] & board.w_occ;
        while attacks != 0 {
            let to = Square::from(attacks.0.trailing_zeros() as u64);
            if to.rank() == 0 {
                vmoves.push(Move {
                    from,
                    to,
                    capture: true,
                    piece: Piece::new(PieceKind::Pawn, Color::Black),
                    special: Some(SpecialMove::Promotion(PieceKind::Queen)),
                });
                vmoves.push(Move {
                    from,
                    to,
                    capture: true,
                    piece: Piece::new(PieceKind::Pawn, Color::Black),
                    special: Some(SpecialMove::Promotion(PieceKind::Rook)),
                });
                vmoves.push(Move {
                    from,
                    to,
                    capture: true,
                    piece: Piece::new(PieceKind::Pawn, Color::Black),
                    special: Some(SpecialMove::Promotion(PieceKind::Bishop)),
                });
                vmoves.push(Move {
                    from,
                    to,
                    capture: true,
                    piece: Piece::new(PieceKind::Pawn, Color::Black),
                    special: Some(SpecialMove::Promotion(PieceKind::Knight)),
                });
            } else {
                vmoves.push(Move {
                    from,
                    to,
                    capture: true,
                    piece: Piece::new(PieceKind::Pawn, Color::Black),
                    special: if from.rank() == 6 && to.rank() == 4 {
                        Some(SpecialMove::DoublePawnPush)
                    } else {
                        None
                    },
                });
            }
            attacks &= attacks - 1;
        }
        if let Some(ep) = board.en_passant {
            let epb = BitBoard(1 << ep as u64);
            if PRECALC.pawns[from.idx_usize()][1] & epb != 0 {
                vmoves.push(Move {
                    from,
                    to: ep,
                    capture: true,
                    piece: Piece::new(PieceKind::Pawn, Color::Black),
                    special: Some(SpecialMove::EnPassant),
                });
            }
        }
    }
}

pub fn square_attacked(board: &Board, square: impl BitBoardIdx, side: Color) -> bool {
    if side == Color::White {
        if PRECALC.pawns[square.idx_usize()][1] & board.w_pawn != BitBoard(0) {
            return true;
        }
    } else if PRECALC.pawns[square.idx_usize()][0] & board.b_pawn != BitBoard(0) {
        return true;
    }

    let boards = match side {
        Color::White => (
            board.w_knight,
            board.w_king,
            board.w_bishop,
            board.w_rook,
            board.w_queen,
        ),
        Color::Black => (
            board.b_knight,
            board.b_king,
            board.b_bishop,
            board.b_rook,
            board.b_queen,
        ),
    };

    if PRECALC.knight[square.idx_usize()] & boards.0 != 0 {
        return true;
    }
    if PRECALC.king[square.idx_usize()] & boards.1 != 0 {
        return true;
    }
    if PRECALC.bishop_attack(square, board.occ) & boards.2 != 0 {
        return true;
    }
    if PRECALC.rook_attack(square, board.occ) & boards.3 != 0 {
        return true;
    }
    if PRECALC.queen_attack(square, board.occ) & boards.4 != 0 {
        return true;
    }
    false
}

pub fn attacked_suares(board: &Board, side: Color) -> BitBoard {
    let mut bb = BitBoard(0);
    for square in 0..64 {
        if square_attacked(board, square, side) {
            bb.set(square as u64);
        }
    }
    bb
}
