use gambit_models::moves::builder::MoveBuilder;
use gambit_models::piece::piece_type::PieceType;

use crate::attacks;
use crate::generator::Context;
use crate::list::MoveList;
use crate::state::State;

pub(crate) fn generate(
    state: &State,
    ctx: &Context,
    list: &mut MoveList,
    piece_type: PieceType,
    captures_only: bool,
) {
    for from in state.our_pieces(ctx.us, piece_type) {
        let mut attack = match piece_type {
            PieceType::Bishop => attacks::bishop_attacks(from, ctx.occupancy),
            PieceType::Rook => attacks::rook_attacks(from, ctx.occupancy),
            PieceType::Queen => attacks::queen_attacks(from, ctx.occupancy),
            _ => unreachable!("sliders::generate is only called with Bishop, Rook, or Queen"),
        };

        attack &= !ctx.own;
        attack &= ctx.target_mask;

        if ctx.pinned.contains(from) {
            attack &= attacks::line_through(ctx.king_square, from);
        }

        for to in attack {
            if ctx.enemy.contains(to) {
                let captured = state.piece_at(to).piece_type().unwrap();
                list.push(MoveBuilder::capture(from, to, piece_type, captured).build());
            } else if !captures_only {
                list.push(MoveBuilder::quiet(from, to, piece_type).build());
            }
        }
    }
}
