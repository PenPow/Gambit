use gambit_models::bitboard::Bitboard;
use gambit_models::location::map::square::SquareMap;
use gambit_models::location::square::Square;
use gambit_models::piece::map::ColourMap;

const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];

const KING_OFFSETS: [(i8, i8); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

const WHITE_PAWN_OFFSETS: [(i8, i8); 2] = [(-1, 1), (1, 1)];
const BLACK_PAWN_OFFSETS: [(i8, i8); 2] = [(-1, -1), (1, -1)];

const fn leaper_attacks_from(square: Square, offsets: &[(i8, i8)]) -> Bitboard {
    let (file, rank) = square.coordinates();

    let mut bb = Bitboard::EMPTY;

    let mut i = 0;
    while i < offsets.len() {
        let (df, dr) = offsets[i];

        let target_file = file.offset(df);
        let target_rank = rank.offset(dr);

        if let Some(file) = target_file
            && let Some(rank) = target_rank
        {
            let target_square = Square::from_coordinates((file, rank));

            bb = Bitboard::new(bb.bits() | Bitboard::from_square(target_square).bits());
        }

        i += 1;
    }

    bb
}

const fn build_table(offsets: &[(i8, i8)]) -> SquareMap<Bitboard> {
    let mut table = [Bitboard::EMPTY; Square::COUNT];
    let mut i = 0;

    while i < 64 {
        table[i] = leaper_attacks_from(Square::from_index(i as u8), offsets);
        i += 1;
    }

    SquareMap::from_array(table)
}

pub(crate) static KNIGHT_ATTACKS: SquareMap<Bitboard> = build_table(&KNIGHT_OFFSETS);
pub(crate) static KING_ATTACKS: SquareMap<Bitboard> = build_table(&KING_OFFSETS);
pub(crate) static PAWN_ATTACKS: ColourMap<SquareMap<Bitboard>> = ColourMap::from_array([
    build_table(&WHITE_PAWN_OFFSETS),
    build_table(&BLACK_PAWN_OFFSETS),
]);
