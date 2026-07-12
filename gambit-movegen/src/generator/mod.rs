use crate::state::State;
use crate::{attacks, masks};
use gambit_models::bitboard::Bitboard;
use gambit_models::location::square::Square;
use gambit_models::piece::colour::Colour;

pub mod legal;
mod piece;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Context {
    pub us: Colour,
    pub occupancy: Bitboard,
    pub own: Bitboard,
    pub enemy: Bitboard,
    pub king_square: Square,
    pub target_mask: Bitboard,
    pub pinned: Bitboard,
    pub check_count: u32,
}

impl Context {
    fn new(state: &State) -> Self {
        let us = state.side_to_move();
        let occupancy = state.occupancy();
        let own = state.our(us);
        let enemy = state.their(us);
        let king_square = state.king_square(us);

        let checkers_bb = masks::checkers(state, us);
        let check_count = checkers_bb.bits().count_ones();

        let target_mask = match check_count {
            0 => Bitboard::UNIVERSE,
            1 => {
                let checker_index = checkers_bb.bits().trailing_zeros() as u8;

                // SAFETY: checkers has exactly one bit set here.
                let checker_sq = unsafe { Square::from_index_unchecked(checker_index) };
                checkers_bb | attacks::between(king_square, checker_sq)
            }
            _ => Bitboard::EMPTY,
        };

        let pinned = masks::pinned_pieces(state, us);

        Self {
            us,
            occupancy,
            own,
            enemy,
            king_square,
            target_mask,
            pinned,
            check_count,
        }
    }
}
