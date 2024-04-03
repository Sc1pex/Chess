use crate::{board::Board, movegen::Move};

pub mod v1;

pub trait BotTrait {
    fn make_move(&mut self, board: Board) -> Move;
}
