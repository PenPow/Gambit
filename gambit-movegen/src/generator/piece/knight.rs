use crate::attacks;
use crate::generator::Context;
use crate::list::MoveList;
use crate::state::State;
use gambit_models::moves::builder::MoveBuilder;
use gambit_models::piece::piece_type::PieceType;

pub(crate) fn generate(state: &State, ctx: &Context, list: &mut MoveList) {
    // exclude pinned knights
    let knights = state.our_pieces(ctx.us, PieceType::Knight) & !ctx.pinned;

    for from in knights {
        let targets = attacks::knight_attacks(from) & !ctx.own & ctx.target_mask;

        for to in targets {
            if ctx.enemy.contains(to) {
                let captured = state.piece_at(to).piece_type().unwrap();
                list.push(MoveBuilder::capture(from, to, PieceType::Knight, captured).build());
            } else {
                list.push(MoveBuilder::quiet(from, to, PieceType::Knight).build());
            }
        }
    }
}
