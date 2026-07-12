use gambit_models::piece::colour::Colour;
use gambit_models::piece::piece_type::PieceType;
use gambit_movegen::generator::pseudo_legal;
use gambit_movegen::state::State;

const KNIGHT_MOBILITY_WEIGHT: i32 = 4;
const BISHOP_MOBILITY_WEIGHT: i32 = 3;
const ROOK_MOBILITY_WEIGHT: i32 = 2;
const QUEEN_MOBILITY_WEIGHT: i32 = 1;

pub fn mobility(state: &State) -> i32 {
    let mut score = 0;

    for colour in Colour::ALL {
        let sign = if colour == Colour::White { 1 } else { -1 };
        let own_occupied = state.our(colour);

        for (pt, weight) in [
            (PieceType::Knight, KNIGHT_MOBILITY_WEIGHT),
            (PieceType::Bishop, BISHOP_MOBILITY_WEIGHT),
            (PieceType::Rook, ROOK_MOBILITY_WEIGHT),
            (PieceType::Queen, QUEEN_MOBILITY_WEIGHT),
        ] {
            let bb = state.our_pieces(colour, pt);
            let occupancy = state.occupancy();

            for square in bb {
                let attacks = pseudo_legal::attacks(pt, square, colour, occupancy);

                let count = (attacks & !own_occupied).count();
                score += sign * weight * count as i32;
            }
        }
    }

    score
}