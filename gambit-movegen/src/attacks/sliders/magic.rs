use gambit_models::bitboard::Bitboard;
use gambit_models::location::map::square::SquareMap;
use gambit_models::location::square::Square;

pub(crate) struct Magic {
    pub mask: u64,
    pub magic: u64,
    pub shift: u32,
    pub offset: u32,
}

impl Magic {
    #[inline]
    pub fn index(&self, occupied: Bitboard) -> usize {
        let relevant = occupied.bits() & self.mask;
        let hash = relevant.wrapping_mul(self.magic) >> self.shift;

        self.offset as usize + hash as usize
    }
}

include!(concat!(env!("OUT_DIR"), "/magic_tables.rs"));

#[inline]
pub(crate) fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    ROOK_ATTACKS[Magic::index(&ROOK_MAGICS[square], occupied)]
}

#[inline]
pub(crate) fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    BISHOP_ATTACKS[Magic::index(&BISHOP_MAGICS[square], occupied)]
}
