use super::*;
use crate::{
    bitboardindex::BitBoardIdx,
    movegen::{legal_moves, SpecialMove},
    piece::{Color, Piece, PieceKind},
    square::Square,
    transposition::{TranspositionKind, TranspositionTable},
    zobrist::Zobrist,
};
use zduny_wasm_timer::Instant;

pub struct Bot {
    depth: i32,

    transposition_table: TranspositionTable,
    zobrist: Zobrist,

    time: u64,
    should_stop: bool,
    start: Instant,

    pub pv_table: Vec<Box<[Move]>>,
    pub pv_len: Vec<usize>,
    pub nodes_searched: u64,
    pub tt_hits: u64,
    pub score: i32,
    pub reached_depth: i32,
}

impl Bot {
    pub fn tt_stores(&self) -> usize {
        self.transposition_table.stored_cnt
    }
}

impl BotTrait for Bot {
    fn make_move(&mut self, board: Board) -> Move {
        self.start = Instant::now();
        for depth in 1..=self.depth {
            self.search(board.clone(), -500_000, 500_000, depth, 0, true);
            self.reached_depth = depth;
            if self.should_stop {
                break;
            }
        }
        self.pv_table[0][0]
    }
}

impl Bot {
    pub fn new(depth: i32, tt_entries: usize, ms: u64) -> Self {
        let pv_table_size = 128;
        let pv_table = (0..pv_table_size)
            .map(|i| vec![Move::empty(); pv_table_size - i].into_boxed_slice())
            .collect::<Vec<_>>();

        Self {
            depth,

            transposition_table: TranspositionTable::new(tt_entries),
            zobrist: Zobrist::new(),

            time: ms,
            should_stop: false,
            start: Instant::now(),

            pv_table,
            pv_len: vec![0; pv_table_size],

            nodes_searched: 0,
            score: 0,
            tt_hits: 0,
            reached_depth: 0,
        }
    }
}

const PIECE_KINDS: [PieceKind; 6] = [
    PieceKind::Pawn,
    PieceKind::Knight,
    PieceKind::Bishop,
    PieceKind::Rook,
    PieceKind::Queen,
    PieceKind::King,
];

impl Bot {
    fn check_time(&mut self) {
        let ms = Instant::now() - self.start;
        let ms = ms.as_millis() as u64;
        if ms >= self.time {
            self.should_stop = true;
        }
    }

    fn quiescence(&mut self, board: Board, mut alpha: i32, beta: i32, ply: i32) -> i32 {
        if self.nodes_searched % 5000 == 0 {
            self.check_time();
            if self.should_stop {
                return 0;
            }
        }

        let score = Self::evaluate(&board);
        if score >= beta {
            return beta;
        }
        alpha = alpha.max(score);

        let moves = self.sorted_moves(ply, &board, false);
        for m in moves.into_iter().filter(|m| m.capture) {
            let mut b = board.clone();
            b.make_move(m);
            let score = -self.quiescence(b, -beta, -alpha, ply + 1);

            if self.should_stop {
                return 0;
            }

            if score >= beta {
                return beta;
            }
            alpha = alpha.max(score);
        }

        alpha
    }

    pub fn search(
        &mut self,
        board: Board,
        mut alpha: i32,
        beta: i32,
        depth: i32,
        ply: i32,
        mut follow_pv: bool,
    ) -> i32 {
        self.nodes_searched += 1;
        if self.nodes_searched % 5000 == 0 {
            self.check_time();
            if self.should_stop {
                return 0;
            }
        }

        self.pv_len[ply as usize] = 1;

        if depth == 0 {
            return self.quiescence(board, alpha, beta, ply);
        }

        // TODO: Incremental hash update
        let hash = self.zobrist.hash(&board);
        if let Some(score) = self.transposition_table.probe(hash, depth, alpha, beta) {
            self.tt_hits += 1;
            return score;
        }

        let moves = legal_moves(&board);
        if follow_pv {
            follow_pv = false;
            for m in moves.iter() {
                if m == &self.pv_table[ply as usize][0] {
                    follow_pv = true;
                    break;
                }
            }
        }

        let moves = self.sorted_moves(ply, &board, follow_pv);
        if moves.is_empty() {
            return if board.in_check { -490_000 + ply } else { 0 };
        }

        let mut tt_entry_kind = TranspositionKind::Alpha;

        let next_depth = if board.in_check { depth } else { depth - 1 };
        for m in moves.iter() {
            let mut b = board.clone();
            b.make_move(m);
            let score = -self.search(b, -beta, -alpha, next_depth, ply + 1, follow_pv);

            if self.should_stop {
                return 0;
            }

            if score >= beta {
                self.transposition_table
                    .store(hash, depth, beta, TranspositionKind::Beta, *m);
                return beta;
            }

            if score > alpha {
                tt_entry_kind = TranspositionKind::Exact;
                alpha = score;

                let p = ply as usize;
                self.pv_table[p][0] = *m;
                for i in 0..self.pv_table[p + 1].len() {
                    self.pv_table[p][i + 1] = self.pv_table[p + 1][i];
                }
                self.pv_len[p] = self.pv_len[p + 1] + 1;
            }
        }

        self.transposition_table.store(
            hash,
            depth,
            alpha,
            tt_entry_kind,
            self.pv_table[ply as usize][0],
        );

        alpha
    }

    pub fn evaluate(board: &Board) -> i32 {
        let score = |c: Color| {
            let mut s = 0;
            for p in PIECE_KINDS.into_iter() {
                let mut b = board.board(Piece::new(p, c)).0;
                while b != 0 {
                    let square = b.trailing_zeros() as usize;
                    b &= b - 1;
                    s += Self::piece_value(p);
                    s += if c == Color::White {
                        Self::piece_square_value(p, square)
                    } else {
                        Self::piece_square_value(p, Self::mirror_square(square))
                    };
                }
            }
            s
        };

        let eval = score(Color::White) - score(Color::Black);
        if board.side_to_move == Color::White {
            eval
        } else {
            -eval
        }
    }

    fn move_score(&self, ply: i32, m: &Move, board: &Board, score_pv: bool) -> i32 {
        if score_pv && m == &self.pv_table[ply as usize][0] {
            return 10_000;
        }

        if matches!(m.special, Some(SpecialMove::EnPassant)) {
            return Self::capture_value(PieceKind::Pawn, PieceKind::Pawn);
        }

        if let Some(p) = board.piece(m.to.idx()) {
            Self::capture_value(m.piece.kind, p.kind)
        } else {
            0
        }
    }

    pub fn sorted_moves(&self, ply: i32, board: &Board, score_pv: bool) -> Box<[Move]> {
        let moves = legal_moves(board);
        let mut moves = moves
            .into_iter()
            .map(|m| (m, self.move_score(ply, m, board, score_pv)))
            .collect::<Vec<_>>();
        moves.sort_by(|a, b| b.1.cmp(&a.1));
        moves.into_iter().map(|(m, _)| *m).collect()
    }

    fn capture_value(p1: PieceKind, p2: PieceKind) -> i32 {
        10 * Self::piece_value(p2) - Self::piece_value(p1)
    }

    fn piece_value(piece_kind: PieceKind) -> i32 {
        match piece_kind {
            PieceKind::Pawn => 100,
            PieceKind::Knight => 300,
            PieceKind::Bishop => 300,
            PieceKind::Rook => 500,
            PieceKind::Queen => 900,
            PieceKind::King => 10000,
        }
    }

    // Piece square tables
    // Stolen from https://www.youtube.com/playlist?list=PLmN0neTso3Jxh8ZIylk74JpwfiWNI76Cs
    #[rustfmt::skip]
    const PAWN_SCORE: [i32; 64] = [
         0,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0, -10, -10,   0,   0,   0,
         0,   0,   0,   5,   5,   0,   0,   0,
         5,   5,  10,  20,  20,   5,   5,   5,
        10,  10,  10,  20,  20,  10,  10,  10,
        20,  20,  20,  30,  30,  30,  20,  20,
        30,  30,  30,  40,  40,  30,  30,  30,
        90,  90,  90,  90,  90,  90,  90,  90,
    ];

    #[rustfmt::skip]
    const KNIGHT_SCORE: [i32; 64] = [
        -5, -10,   0,   0,   0,   0, -10,  -5,
        -5,   0,   0,   0,   0,   0,   0,  -5,
        -5,   5,  20,  10,  10,  20,   5,  -5,
        -5,  10,  20,  30,  30,  20,  10,  -5,
        -5,  10,  20,  30,  30,  20,  10,  -5,
        -5,   5,  20,  20,  20,  20,   5,  -5,
        -5,   0,   0,  10,  10,   0,   0,  -5,
        -5,   0,   0,   0,   0,   0,   0,  -5,
    ];

    #[rustfmt::skip]
    const BISHOP_SCORE: [i32; 64] =  [
         0,   0, -10,   0,   0, -10,   0,   0,
         0,  30,   0,   0,   0,   0,  30,   0,
         0,  10,   0,   0,   0,   0,  10,   0,
         0,   0,  10,  20,  20,  10,   0,   0,
         0,   0,  10,  20,  20,  10,   0,   0,
         0,   0,   0,  10,  10,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0,   0,
    ];

    #[rustfmt::skip]
    const ROOK_SCORE: [i32; 64] = [
         0,   0,   0,  20,  20,   0,   0,   0,
         0,   0,  10,  20,  20,  10,   0,   0,
         0,   0,  10,  20,  20,  10,   0,   0,
         0,   0,  10,  20,  20,  10,   0,   0,
         0,   0,  10,  20,  20,  10,   0,   0,
         0,   0,  10,  20,  20,  10,   0,   0,
        50,  50,  50,  50,  50,  50,  50,  50,
        50,  50,  50,  50,  50,  50,  50,  50,
    ];

    #[rustfmt::skip]
    const KING_SCORE: [i32; 64] = [
         0,   0,   5,   0, -15,   0,  10,   0,
         0,   5,   5,  -5,  -5,   0,   5,   0,
         0,   0,   5,  10,  10,   5,   0,   0,
         0,   5,  10,  20,  20,  10,   5,   0,
         0,   5,  10,  20,  20,  10,   5,   0,
         0,   5,   5,  10,  10,   5,   5,   0,
         0,   0,   5,   5,   5,   5,   0,   0,
         0,   0,   0,   0,   0,   0,   0,   0,
    ];

    fn piece_square_value(piece: PieceKind, square: usize) -> i32 {
        match piece {
            PieceKind::Pawn => Self::PAWN_SCORE[square],
            PieceKind::Knight => Self::KNIGHT_SCORE[square],
            PieceKind::Bishop => Self::BISHOP_SCORE[square],
            PieceKind::Rook => Self::ROOK_SCORE[square],
            PieceKind::Queen => 0,
            PieceKind::King => Self::KING_SCORE[square],
        }
    }

    pub fn mirror_square(square: usize) -> usize {
        let s = Square::from(square as u64);
        let rank = s.rank() as usize;
        let file = s.file() as usize;
        (7 - rank) * 8 + file
    }
}
