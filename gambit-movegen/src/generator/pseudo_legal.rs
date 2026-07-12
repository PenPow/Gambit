use gambit_models::bitboard::Bitboard;
use gambit_models::location::square::Square;
use gambit_models::piece::colour::Colour;
use gambit_models::piece::piece_type::PieceType;
use crate::attacks;

pub fn attacks(
    piece_type: PieceType,
    square: Square,
    colour: Colour,
    occupied: Bitboard,
) -> Bitboard {
    match piece_type {
        PieceType::Pawn => attacks::pawn_attacks(square, colour),
        PieceType::Knight => attacks::knight_attacks(square),
        PieceType::Bishop => attacks::bishop_attacks(square, occupied),
        PieceType::Rook => attacks::rook_attacks(square, occupied),
        PieceType::Queen => attacks::queen_attacks(square, occupied),
        PieceType::King => attacks::king_attacks(square),
    }
}