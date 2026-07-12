use crate::state::State;
use gambit_models::location::file::File;
use gambit_models::location::map::file::FileMap;
use gambit_models::location::map::square::SquareMap;
use gambit_models::location::square::Square;
use gambit_models::movement::castling::rights::CastlingRights;
use gambit_models::piece::colour::Colour;
use gambit_models::piece::map::{ColourMap, PieceTypeMap};
use gambit_models::piece::piece_type::PieceType;

const ZOBRIST_SEED: u64 = 0x9E37_79B9_7F4A_7C15;

// https://rosettacode.org/wiki/Pseudo-random_numbers/Splitmix64
const fn split_mix64(state: u64) -> (u64, u64) {
    let state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    (z ^ (z >> 31), state)
}

pub struct ZobristKeys {
    pub piece_square: ColourMap<PieceTypeMap<SquareMap<u64>>>,
    pub side_to_move: u64,
    pub castling: [u64; 4],
    pub en_passant_file: FileMap<u64>,
}

const fn fill_u64_array<const N: usize>(mut state: u64) -> ([u64; N], u64) {
    let mut arr = [0u64; N];

    let mut i = 0;
    while i < N {
        let (value, next_state) = split_mix64(state);
        arr[i] = value;
        state = next_state;
        i += 1;
    }

    (arr, state)
}

const fn generate_zobrist_keys() -> ZobristKeys {
    let mut rng_state = ZOBRIST_SEED;

    let mut raw_piece_square = [[[0u64; Square::COUNT]; PieceType::COUNT]; Colour::COUNT];
    let mut colour = 0;
    while colour < Colour::COUNT {
        let mut piece_type = 0;
        while piece_type < PieceType::COUNT {
            let (squares, next_state) = fill_u64_array::<{ Square::COUNT }>(rng_state);

            raw_piece_square[colour][piece_type] = squares;
            rng_state = next_state;
            piece_type += 1;
        }
        colour += 1;
    }

    let piece_square = ColourMap::from_array([
        PieceTypeMap::from_array([
            SquareMap::from_array(raw_piece_square[0][0]),
            SquareMap::from_array(raw_piece_square[0][1]),
            SquareMap::from_array(raw_piece_square[0][2]),
            SquareMap::from_array(raw_piece_square[0][3]),
            SquareMap::from_array(raw_piece_square[0][4]),
            SquareMap::from_array(raw_piece_square[0][5]),
        ]),
        PieceTypeMap::from_array([
            SquareMap::from_array(raw_piece_square[1][0]),
            SquareMap::from_array(raw_piece_square[1][1]),
            SquareMap::from_array(raw_piece_square[1][2]),
            SquareMap::from_array(raw_piece_square[1][3]),
            SquareMap::from_array(raw_piece_square[1][4]),
            SquareMap::from_array(raw_piece_square[1][5]),
        ]),
    ]);

    let (side_to_move, rng_state) = split_mix64(rng_state);

    let mut castling = [0u64; 4];
    let mut i = 0;
    let mut rng_state = rng_state;
    while i < 4 {
        let (value, next_state) = split_mix64(rng_state);
        castling[i] = value;
        rng_state = next_state;
        i += 1;
    }

    let (raw_en_passant, _rng_state) = fill_u64_array::<{ File::COUNT }>(rng_state);
    let en_passant_file = FileMap::from_array(raw_en_passant);

    ZobristKeys {
        piece_square,
        side_to_move,
        castling,
        en_passant_file,
    }
}

pub const ZOBRIST: ZobristKeys = generate_zobrist_keys();

#[inline]
pub fn piece_square_key(colour: Colour, piece_type: PieceType, square: Square) -> u64 {
    ZOBRIST.piece_square[colour][piece_type][square]
}

#[inline]
pub fn side_to_move_key() -> u64 {
    ZOBRIST.side_to_move
}

#[inline]
pub fn castling_key(rights: CastlingRights) -> u64 {
    debug_assert_eq!(rights.bits().count_ones(), 1);

    ZOBRIST.castling[rights.bits().trailing_zeros() as usize]
}

#[inline]
pub fn en_passant_file_key(file: File) -> u64 {
    ZOBRIST.en_passant_file[file]
}

pub fn hash_state(state: &State) -> u64 {
    let mut hash = 0u64;

    for square in Square::ALL {
        let piece = state.piece_at(square);
        if let (Some(colour), Some(piece_type)) = (piece.colour(), piece.piece_type()) {
            hash ^= piece_square_key(colour, piece_type, square);
        }
    }

    if state.side_to_move() == Colour::Black {
        hash ^= side_to_move_key();
    }

    let castling_rights = state.position().castling_rights;

    for right in CastlingRights::RIGHTS {
        if castling_rights.contains(right) {
            hash ^= castling_key(right);
        }
    }

    if let Some(ep_square) = state.position().en_passant {
        hash ^= en_passant_file_key(ep_square.file());
    }

    hash
}
