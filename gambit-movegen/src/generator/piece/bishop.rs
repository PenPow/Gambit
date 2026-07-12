use crate::generator::Context;
use crate::list::MoveList;
use crate::state::State;
use gambit_models::piece::piece_type::PieceType;

pub(crate) fn generate(state: &State, ctx: &Context, list: &mut MoveList) {
    super::sliders::generate(state, ctx, list, PieceType::Bishop)
}
