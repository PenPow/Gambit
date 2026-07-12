use gambit_models::bitboard::Bitboard;
use gambit_models::location::square::Square;
use gambit_models::piece::colour::Colour;
use gambit_models::piece::piece_type::PieceType;

use crate::attacks;
use crate::state::State;

#[inline]
pub fn attackers_to(state: &State, square: Square, occupancy: Bitboard, by: Colour) -> Bitboard {
    let mut attackers = Bitboard::EMPTY;

    attackers |= attacks::pawn_attacks(square, by.other()) & state.our_pieces(by, PieceType::Pawn);
    attackers |= attacks::knight_attacks(square) & state.our_pieces(by, PieceType::Knight);
    attackers |= attacks::king_attacks(square) & state.our_pieces(by, PieceType::King);
    attackers |= attacks::rook_attacks(square, occupancy)
        & (state.our_pieces(by, PieceType::Rook) | state.our_pieces(by, PieceType::Queen));
    attackers |= attacks::bishop_attacks(square, occupancy)
        & (state.our_pieces(by, PieceType::Bishop) | state.our_pieces(by, PieceType::Queen));

    attackers
}

#[inline]
pub fn checkers(state: &State, us: Colour) -> Bitboard {
    attackers_to(state, state.king_square(us), state.occupancy(), us.other())
}

pub fn is_in_check(state: &State, colour: Colour) -> bool {
    !checkers(state, colour).is_empty()
}

pub fn pinned_pieces(state: &State, us: Colour) -> Bitboard {
    let king_sq = state.king_square(us);

    let their_rooks_and_queens =
        state.their_pieces(us, PieceType::Rook) | state.their_pieces(us, PieceType::Queen);
    let their_bishops_and_queens =
        state.their_pieces(us, PieceType::Bishop) | state.their_pieces(us, PieceType::Queen);

    let rook_xray = attacks::rook_attacks(king_sq, state.their(us)) & their_rooks_and_queens;
    let bishop_xray = attacks::bishop_attacks(king_sq, state.their(us)) & their_bishops_and_queens;

    let mut pinned = Bitboard::EMPTY;
    let candidates = rook_xray | bishop_xray;

    for pinner in candidates {
        let between = attacks::between(king_sq, pinner) & state.our(us);
        if between.bits().count_ones() == 1 {
            pinned |= between;
        }
    }

    pinned
}
