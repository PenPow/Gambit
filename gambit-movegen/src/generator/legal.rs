use crate::generator::Context;
use crate::generator::piece;
use crate::list::MoveList;
use crate::state::State;

pub fn generate(state: &State, list: &mut MoveList) {
    let ctx = Context::new(state);

    piece::king::generate(state, &ctx, list, false);

    if ctx.check_count >= 2 {
        return;
    }

    piece::pawn::generate(state, &ctx, list, false);
    piece::knight::generate(state, &ctx, list, false);
    piece::bishop::generate(state, &ctx, list, false);
    piece::rook::generate(state, &ctx, list, false);
    piece::queen::generate(state, &ctx, list, false);
}

pub fn generate_captures(state: &State, list: &mut MoveList) {
    let ctx = Context::new(state);

    piece::king::generate(state, &ctx, list, true);

    if ctx.check_count >= 2 {
        return;
    }

    piece::pawn::generate(state, &ctx, list, true);
    piece::knight::generate(state, &ctx, list, true);
    piece::bishop::generate(state, &ctx, list, true);
    piece::rook::generate(state, &ctx, list, true);
    piece::queen::generate(state, &ctx, list, true);
}
