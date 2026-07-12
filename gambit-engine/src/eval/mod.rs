mod pst;
mod phase;
mod pawns;
mod tables;
mod king_safety;
mod mobility;

use gambit_models::piece::colour::Colour;
use gambit_models::piece::piece_type::PieceType;
use gambit_movegen::state::State;
use pst::pst;
use phase::{game_phase, TOTAL_PHASE};
use crate::eval::king_safety::king_safety;
use crate::eval::mobility::mobility;
use crate::eval::pawns::pawn_structure;

fn bishop_pair_bonus(state: &State) -> i32 {
    const BISHOP_PAIR_BONUS: i32 = 30;

    let white_bishops = state.our_pieces(Colour::White, PieceType::Bishop).count();
    let black_bishops = state.our_pieces(Colour::Black, PieceType::Bishop).count();

    let mut bonus = 0;
    if white_bishops >= 2 {
        bonus += BISHOP_PAIR_BONUS;
    }
    if black_bishops >= 2 {
        bonus -= BISHOP_PAIR_BONUS;
    }

    bonus
}

fn get_piece_score(piece_type: PieceType) -> i32 {
    match piece_type {
        PieceType::Pawn => 100,
        PieceType::Knight | PieceType::Bishop => 300,
        PieceType::Rook => 500,
        PieceType::Queen => 900,
        _ => 0,
    }
}

fn material_and_pst(state: &State) -> (i32, i32) {
    let mut mg = 0;
    let mut eg = 0;

    for colour in Colour::ALL {
        let sign = if colour == Colour::White { 1 } else { -1 };

        for pt in PieceType::ALL {
            let bitboard = state.our_pieces(colour, pt);

            for square in bitboard {
                let material = get_piece_score(pt);

                mg += sign * (material + pst(pt, square, colour, false));
                eg += sign * (material + pst(pt, square, colour, true));
            }
        }
    }

    (mg, eg)
}

pub fn evaluate(state: &State) -> i32 {
    let (mg, eg) = material_and_pst(state);

    let phase = game_phase(state);
    let tapered = (mg * phase + eg * (TOTAL_PHASE - phase)) / TOTAL_PHASE;

    let bishop_pair = bishop_pair_bonus(state);
    let pawns = pawn_structure(state);
    let king_safety = king_safety(state);
    let mobility = mobility(state);

    let total = tapered + bishop_pair + pawns + king_safety + mobility;

    let perspective = if state.side_to_move() == Colour::White { 1 } else { -1 };
    total * perspective
}