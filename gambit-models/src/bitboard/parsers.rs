use crate::bitboard::Bitboard;
use crate::location::file::File;
use crate::location::rank::Rank;
use crate::location::square::Square;
use crate::traits::IntoBitboard;

impl From<u64> for Bitboard {
    fn from(bits: u64) -> Self {
        Self::new(bits)
    }
}

impl From<Bitboard> for u64 {
    fn from(bitboard: Bitboard) -> Self {
        bitboard.0
    }
}

impl From<Square> for Bitboard {
    fn from(square: Square) -> Self {
        Self::from_square(square)
    }
}

impl From<Rank> for Bitboard {
    fn from(rank: Rank) -> Self {
        Self::from_rank(rank)
    }
}

impl From<File> for Bitboard {
    fn from(file: File) -> Self {
        Self::from_file(file)
    }
}

impl IntoBitboard for Bitboard {
    #[inline]
    fn into_bitboard(self) -> Bitboard {
        self
    }
}
