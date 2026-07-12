use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    eval::write_eval_tables(out_dir);
}

mod eval {
    use gambit_models::bitboard::Bitboard;
    use gambit_models::location::map::square::SquareMap;
    use gambit_models::location::square::Square;
    use std::fmt::Write;
    use std::fs;
    use std::path::Path;
    use gambit_models::location::file::File;
    use gambit_models::location::map::file::FileMap;
    use gambit_models::location::rank::Rank;

    fn bitboard_hex(bb: Bitboard) -> String {
        format!("Bitboard::new({:#018x})", bb.bits())
    }

    fn build_adjacent_file_masks() -> FileMap<Bitboard> {
        let mut masks = FileMap::new();

        for file in File::ALL {
            if file > File::A {
                masks[file] |= file.offset(-1).unwrap();
            }
            if file < File::H {
                masks[file] |=file.offset(1).unwrap();
            }
        }

        masks
    }

    fn build_passed_pawn_masks(
        adjacent_file_masks: &FileMap<Bitboard>,
    ) -> (SquareMap<Bitboard>, SquareMap<Bitboard>) {
        let mut white = SquareMap::from_array([Bitboard::EMPTY; 64]);
        let mut black = SquareMap::from_array([Bitboard::EMPTY; 64]);

        for square in Square::ALL {
            let file = square.file();
            let rank = square.rank();
            let files = file.bitboard() | adjacent_file_masks[file];

            let mut white_mask = Bitboard::EMPTY;
            let mut black_mask = Bitboard::EMPTY;

            for other in Square::ALL {
                let other_rank = other.rank();
                if files.contains(other) {
                    if other_rank > rank {
                        white_mask |= other;
                    }
                    if other_rank < rank {
                        black_mask |= other;
                    }
                }
            }

            white[square] = white_mask;
            black[square] = black_mask;
        }

        (white, black)
    }

    fn build_king_shield_masks(
        adjacent_file_masks: &FileMap<Bitboard>,
    ) -> (SquareMap<Bitboard>, SquareMap<Bitboard>) {
        let mut white = SquareMap::from_array([Bitboard::EMPTY; 64]);
        let mut black = SquareMap::from_array([Bitboard::EMPTY; 64]);

        for square in Square::ALL {
            let file = square.file();
            let rank = square.rank();
            let files = file.bitboard() | adjacent_file_masks[file];

            let mut white_mask = Bitboard::EMPTY;
            let mut black_mask = Bitboard::EMPTY;

            for other in Square::ALL {
                if !files.contains(other) {
                    continue;
                }

                let other_rank = other.rank();

                if let Some(offset) = rank.offset(1) && other_rank == offset {
                    white_mask |= other;
                }

                if rank > Rank::One && let Some(offset) = rank.offset(-1) && other_rank == offset {
                    black_mask |= other;
                }
            }

            white[square] = white_mask;
            black[square] = black_mask;
        }

        (white, black)
    }

    fn format_flat_array(masks: &FileMap<Bitboard>) -> String {
        let mut src = String::new();
        for mask in masks {
            write!(src, "{}, ", bitboard_hex(*mask)).unwrap();
        }

        src
    }

    fn format_square_map(map: &SquareMap<Bitboard>) -> String {
        let mut src = String::new();
        for square in Square::ALL {
            write!(src, "{}, ", bitboard_hex(map[square])).unwrap();
        }

        src
    }

    pub fn write_eval_tables(out_dir: &Path) {
        let adjacent_file_masks = build_adjacent_file_masks();
        let (passed_white, passed_black) =
            build_passed_pawn_masks(&adjacent_file_masks);
        let (shield_white, shield_black) =
            build_king_shield_masks(&adjacent_file_masks);

        let src = format!(
            "pub(crate) static ADJACENT_FILE_MASKS: FileMap<Bitboard> = FileMap::from_array([{}]);\n\
             pub(crate) static PASSED_PAWN_MASKS_WHITE: SquareMap<Bitboard> = SquareMap::from_array([{}]);\n\
             pub(crate) static PASSED_PAWN_MASKS_BLACK: SquareMap<Bitboard> = SquareMap::from_array([{}]);\n\
             pub(crate) static KING_SHIELD_MASKS_WHITE: SquareMap<Bitboard> = SquareMap::from_array([{}]);\n\
             pub(crate) static KING_SHIELD_MASKS_BLACK: SquareMap<Bitboard> = SquareMap::from_array([{}]);\n",
            format_flat_array(&adjacent_file_masks),
            format_square_map(&passed_white),
            format_square_map(&passed_black),
            format_square_map(&shield_white),
            format_square_map(&shield_black),
        );

        fs::write(out_dir.join("eval_tables.rs"), src).unwrap();
    }
}