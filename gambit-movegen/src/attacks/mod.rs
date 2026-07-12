use gambit_models::bitboard::Bitboard;
use gambit_models::location::square::Square;
use gambit_models::piece::colour::Colour;

mod leapers;
mod lines;
mod sliders;

#[inline]
pub fn knight_attacks(square: Square) -> Bitboard {
    leapers::KNIGHT_ATTACKS[square]
}

#[inline]
pub fn king_attacks(square: Square) -> Bitboard {
    leapers::KING_ATTACKS[square]
}

#[inline]
pub fn pawn_attacks(square: Square, colour: Colour) -> Bitboard {
    leapers::PAWN_ATTACKS[colour][square]
}

#[inline]
pub fn rook_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    sliders::rook_attacks(square, occupancy)
}

#[inline]
pub fn bishop_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    sliders::bishop_attacks(square, occupancy)
}

#[inline]
pub fn queen_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    rook_attacks(square, occupancy) | bishop_attacks(square, occupancy)
}

pub use lines::{between, line_through};
