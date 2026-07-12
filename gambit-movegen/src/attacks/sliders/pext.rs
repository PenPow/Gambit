use gambit_models::bitboard::Bitboard;
use gambit_models::location::map::square::SquareMap;
use gambit_models::location::square::Square;
use std::arch::x86_64::_pext_u64;

pub struct PextEntry {
    pub mask: u64,
    pub offset: u32,
}

impl PextEntry {
    #[target_feature(enable = "bmi2")]
    pub unsafe fn index(&self, occupied: Bitboard) -> usize {
        // SAFETY: caller has already confirmed BMI2 is present on this CPU
        #[allow(unused_unsafe)]
        let extracted = unsafe { _pext_u64(occupied.bits(), self.mask) };
        self.offset as usize + extracted as usize
    }
}

include!(concat!(env!("OUT_DIR"), "/pext_tables.rs"));

#[target_feature(enable = "bmi2")]
pub(crate) unsafe fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    // SAFETY: caller has already confirmed BMI2 is present on this CPU
    ROOK_PEXT_ATTACKS[unsafe { PextEntry::index(&ROOK_PEXT[square], occupied) }]
}

#[target_feature(enable = "bmi2")]
pub(crate) unsafe fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    // SAFETY: caller has already confirmed BMI2 is present on this CPU
    BISHOP_PEXT_ATTACKS[unsafe { PextEntry::index(&BISHOP_PEXT[square], occupied) }]
}
