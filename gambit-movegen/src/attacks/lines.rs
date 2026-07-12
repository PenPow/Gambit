use gambit_models::bitboard::Bitboard;
use gambit_models::location::map::square::SquareMap;
use gambit_models::location::square::Square;

include!(concat!(env!("OUT_DIR"), "/line_tables.rs"));

#[inline]
pub fn between(a: Square, b: Square) -> Bitboard {
    BETWEEN[a][b]
}

#[inline]
pub fn line_through(a: Square, b: Square) -> Bitboard {
    LINE_THROUGH[a][b]
}
