use crate::generator::Context;
use crate::generator::piece;
use crate::list::MoveList;
use crate::state::State;

pub fn generate(state: &State, list: &mut MoveList) {
    let ctx = Context::new(state);

    piece::king::generate(state, &ctx, list);

    if ctx.check_count >= 2 {
        return;
    }

    piece::pawn::generate(state, &ctx, list);
    piece::knight::generate(state, &ctx, list);
    piece::bishop::generate(state, &ctx, list);
    piece::rook::generate(state, &ctx, list);
    piece::queen::generate(state, &ctx, list);
}
