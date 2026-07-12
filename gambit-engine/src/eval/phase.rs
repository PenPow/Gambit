use gambit_models::piece::colour::Colour;
use gambit_models::piece::piece_type::PieceType;
use gambit_movegen::state::State;

const KNIGHT_PHASE: i32 = 1;
const BISHOP_PHASE: i32 = 1;
const ROOK_PHASE: i32 = 2;
const QUEEN_PHASE: i32 = 4;
pub(crate) const TOTAL_PHASE: i32 = KNIGHT_PHASE * 4 + BISHOP_PHASE * 4 + ROOK_PHASE * 4 + QUEEN_PHASE * 2;

pub fn game_phase(state: &State) -> i32 {
    let mut phase = 0;

    for colour in Colour::ALL {
        phase += KNIGHT_PHASE * state.our_pieces(colour, PieceType::Knight).count() as i32;
        phase += BISHOP_PHASE * state.our_pieces(colour, PieceType::Bishop).count() as i32;
        phase += ROOK_PHASE * state.our_pieces(colour, PieceType::Rook).count() as i32;
        phase += QUEEN_PHASE * state.our_pieces(colour, PieceType::Queen).count() as i32;
    }

    phase.min(TOTAL_PHASE)
}