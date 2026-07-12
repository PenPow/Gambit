use gambit_models::location::file::File;
use gambit_models::location::map::rank::RankMap;
use gambit_models::location::rank::Rank;
use gambit_models::piece::colour::Colour;
use gambit_models::piece::piece_type::PieceType;
use gambit_movegen::state::State;
use crate::eval::tables::{ADJACENT_FILE_MASKS, PASSED_PAWN_MASKS_BLACK, PASSED_PAWN_MASKS_WHITE};

const DOUBLED_PAWN_PENALTY: i32 = -15;
const ISOLATED_PAWN_PENALTY: i32 = -20;
const PASSED_PAWN_BONUS: RankMap<i32> = RankMap::from_array([0, 5, 10, 20, 35, 60, 100, 0]);

pub fn pawn_structure(state: &State) -> i32 {
    let mut score = 0;

    for colour in Colour::ALL {
        let sign = if colour == Colour::White { 1 } else { -1 };
        let is_white = colour == Colour::White;

        let own_pawns = state.our_pieces(colour, PieceType::Pawn);
        let enemy_pawns = state.our_pieces(colour.other(), PieceType::Pawn);

        for file in File::ALL {
            let count = (own_pawns & file.bitboard()).count();

            if count > 1 {
                score += sign * DOUBLED_PAWN_PENALTY * (count as i32 - 1);
            }
        }

        let bb = own_pawns;
        for square in bb {
            let file = square.file();

            if (own_pawns & ADJACENT_FILE_MASKS[file]).is_empty() {
                score += sign * ISOLATED_PAWN_PENALTY;
            }

            let passed_mask = if is_white {
                PASSED_PAWN_MASKS_WHITE[square]
            } else {
                PASSED_PAWN_MASKS_BLACK[square]
            };

            if (enemy_pawns & passed_mask).is_empty() {
                let rank = square.rank();
                let relative_rank = if is_white { rank } else { unsafe { Rank::from_index_unchecked(7 - rank.bits()) } };
                score += sign * PASSED_PAWN_BONUS[relative_rank];
            }
        }
    }

    score
}