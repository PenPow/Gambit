use gambit_models::bitboard::Bitboard;
use gambit_models::location::square::Square;

pub mod magic;

#[cfg(all(target_arch = "x86_64", feature = "bmi2"))]
pub mod pext;

#[cfg(all(target_arch = "x86_64", feature = "bmi2"))]
fn has_bmi2() -> bool {
    use std::sync::OnceLock;

    static HAS_BMI2: OnceLock<bool> = OnceLock::new();

    *HAS_BMI2.get_or_init(|| is_x86_feature_detected!("bmi2"))
}

#[inline]
pub(crate) fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    #[cfg(all(target_arch = "x86_64", feature = "bmi2"))]
    if has_bmi2() {
        // SAFETY: has_bmi2() confirmed the instruction is available
        return unsafe { pext::rook_attacks(square, occupied) };
    }

    magic::rook_attacks(square, occupied)
}

#[inline]
pub(crate) fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    #[cfg(all(target_arch = "x86_64", feature = "bmi2"))]
    if has_bmi2() {
        // SAFETY: has_bmi2() confirmed the instruction is available
        return unsafe { pext::bishop_attacks(square, occupied) };
    }

    magic::bishop_attacks(square, occupied)
}
