use gambit_models::location::file::File;
use gambit_models::piece::colour::Colour;
use gambit_models::piece::piece_type::PieceType;
use gambit_movegen::state::State;
use crate::eval::tables::{KING_SHIELD_MASKS_WHITE, KING_SHIELD_MASKS_BLACK};

const SHIELD_PAWN_BONUS: i32 = 10;
const OPEN_FILE_NEAR_KING_PENALTY: i32 = -20;
const SEMI_OPEN_FILE_NEAR_KING_PENALTY: i32 = -10;

pub fn king_safety(state: &State) -> i32 {
    let mut score = 0;

    for colour in Colour::ALL {
        let sign = if colour == Colour::White { 1 } else { -1 };
        let is_white = colour == Colour::White;

        let king_square = state.our_pieces(colour, PieceType::King).pop().expect("expected only single king, but found none");
        let king_file = king_square.file();

        let is_flank = king_file <= File::C || king_file >= File::F;
        if !is_flank {
            continue;
        }

        let own_pawns = state.our_pieces(colour, PieceType::Pawn);
        let enemy_pawns = state.our_pieces(colour.other(), PieceType::Pawn);

        let shield_mask = if is_white {
            KING_SHIELD_MASKS_WHITE[king_square]
        } else {
            KING_SHIELD_MASKS_BLACK[king_square]
        };

        let shield_pawns = (own_pawns & shield_mask).count();
        score += sign * SHIELD_PAWN_BONUS * shield_pawns as i32;

        for file_offset in -1i32..=1 {
            let file_index = king_file.bits() as i32 + file_offset;
            if !(0..8).contains(&file_index) {
                continue;
            }

            let file = unsafe { File::from_index_unchecked(file_index as u8) };

            let file_mask = file.bitboard();
            let own_pawn_on_file = !(own_pawns & file_mask).is_empty();
            let enemy_pawn_on_file = !(enemy_pawns & file_mask).is_empty();

            if !own_pawn_on_file && !enemy_pawn_on_file {
                score += sign * OPEN_FILE_NEAR_KING_PENALTY;
            } else if !own_pawn_on_file {
                score += sign * SEMI_OPEN_FILE_NEAR_KING_PENALTY;
            }
        }
    }

    score
}