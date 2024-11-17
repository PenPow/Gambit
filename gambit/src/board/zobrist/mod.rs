//! Pre-computed zobrist randoms

#![allow(clippy::unreadable_literal)]

/// The type of each generated random
#[allow(clippy::module_name_repetitions)] // Not dead code, used by generated_randoms
pub type ZobristRandom = u64;

include!("generated_randoms.rs");