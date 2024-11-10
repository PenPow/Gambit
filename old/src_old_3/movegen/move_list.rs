use arrayvec::ArrayVec;

use super::piece_move::Move;

pub const MOVE_LIST_CAP: usize = 218;
pub type MoveList = ArrayVec<Move, MOVE_LIST_CAP>;