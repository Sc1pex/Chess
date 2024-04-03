use crate::{
    board::Board,
    movegen::{legal_moves, square_attacked, Move},
    piece::PieceKind,
    wasm::GameState,
};

pub struct Game {
    pub board: Board,
    pub moves: Vec<Move>,
    pub board_history: Vec<Board>,

    pub game_state: GameState,

    pub fifty_move_rule: u8,
}

impl Game {
    pub fn from_fen(fen: &str) -> Self {
        Self {
            board: Board::from_fen(fen),
            moves: Vec::new(),
            board_history: Vec::new(),
            game_state: GameState::InProgress,

            fifty_move_rule: 0,
        }
    }

    pub fn moves(&self) -> Vec<Move> {
        self.moves.clone()
    }

    pub fn make_move(&mut self, mv: Move) {
        if !mv.capture && mv.piece.kind != PieceKind::Pawn {
            self.fifty_move_rule += 1;
        } else {
            self.fifty_move_rule = 0;
        }

        self.board.make_move(&mv);
        self.board_history.push(self.board.clone());
        self.moves.push(mv);
        self.update_state();
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::start_pos(),
            moves: Vec::new(),
            board_history: vec![Board::start_pos()],
            game_state: GameState::InProgress,

            fifty_move_rule: 0,
        }
    }
}

impl Game {
    pub fn update_state(&mut self) {
        let moves = legal_moves(&self.board);
        if moves.is_empty() {
            let king = self.board.boards_color(self.board.side_to_move)[5]
                .0
                .trailing_zeros() as u64;
            if square_attacked(&self.board, king, self.board.side_to_move.opposite()) {
                self.game_state = GameState::Checkmate;
            } else {
                self.game_state = GameState::Stalemate;
            }
        }

        // Fifty move rule
        if self.fifty_move_rule >= 100 {
            self.game_state = GameState::DrawByFiftyMoveRule;
        }

        // Unsufficient material
        if self.board.w_pawn == 0
            && self.board.b_pawn == 0
            && self.board.w_rook == 0
            && self.board.b_rook == 0
            && self.board.w_queen == 0
            && self.board.b_queen == 0
            && self.board.w_knight.0.count_ones() + self.board.w_bishop.0.count_ones() <= 1
            && self.board.b_knight.0.count_ones() + self.board.b_bishop.0.count_ones() <= 1
        {
            self.game_state = GameState::DrawByInsufficientMaterial;
        }

        // Threefold repetition
        if self
            .board_history
            .iter()
            .filter(|b| **b == self.board)
            .count()
            >= 3
        {
            self.game_state = GameState::DrawByRepetition;
        }
    }
}
