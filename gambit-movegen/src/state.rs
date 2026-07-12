use crate::zobrist;
use gambit_models::bitboard::Bitboard;
use gambit_models::location::square::Square;
use gambit_models::movement::castling::rights::CastlingRights;
use gambit_models::moves::Move;
use gambit_models::moves::kind::MoveKind;
use gambit_models::piece::Piece;
use gambit_models::piece::colour::Colour;
use gambit_models::piece::map::{ColourMap, PieceTypeMap};
use gambit_models::piece::piece_type::PieceType;
use gambit_models::position::{HalfmoveClock, Position};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct State {
    position: Position,
    occupancy: Bitboard,
    colour_bitboard: ColourMap<Bitboard>,
    piece_bitboard: PieceTypeMap<Bitboard>,
    hash: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Undo {
    castling_rights: CastlingRights,
    en_passant: Option<Square>,
    halfmove_clock: HalfmoveClock,
}

impl State {
    pub fn from_position(position: Position) -> Self {
        let mut colour_bitboard = ColourMap::new();
        let mut piece_bitboard = PieceTypeMap::new();

        let mut hash = 0u64;

        for (square, piece) in position.board {
            if piece.is_some() {
                // SAFETY: As piece is some, colour will also be some
                let colour = unsafe { piece.colour().unwrap_unchecked() };
                // SAFETY: As piece is some, piece type will also be some
                let piece_type = unsafe { piece.piece_type().unwrap_unchecked() };

                colour_bitboard[colour] |= square;
                piece_bitboard[piece_type] |= square;

                hash ^= zobrist::piece_square_key(colour, piece_type, square);
            }
        }

        if position.side_to_move == Colour::Black {
            hash ^= zobrist::side_to_move_key();
        }

        for right in CastlingRights::RIGHTS {
            if position.castling_rights.contains(right) {
                hash ^= zobrist::castling_key(right);
            }
        }

        if let Some(ep) = position.en_passant {
            hash ^= zobrist::en_passant_file_key(ep.file());
        }

        Self {
            position,
            occupancy: colour_bitboard[Colour::White] | colour_bitboard[Colour::Black],
            colour_bitboard,
            piece_bitboard,
            hash,
        }
    }

    #[inline]
    pub fn hash(&self) -> u64 {
        self.hash
    }

    #[inline]
    pub fn occupancy(&self) -> Bitboard {
        self.occupancy
    }

    #[inline]
    pub fn our(&self, us: Colour) -> Bitboard {
        self.colour_bitboard[us]
    }

    #[inline]
    pub fn their(&self, us: Colour) -> Bitboard {
        self.colour_bitboard[us.other()]
    }

    #[inline]
    pub fn side_to_move(&self) -> Colour {
        self.position.side_to_move
    }

    #[inline]
    pub fn position(&self) -> &Position {
        &self.position
    }

    #[inline]
    pub fn piece_at(&self, square: Square) -> Piece {
        self.position.board[square]
    }

    #[inline]
    pub fn pieces(&self, piece_type: PieceType) -> Bitboard {
        self.piece_bitboard[piece_type]
    }

    #[inline]
    pub fn our_pieces(&self, us: Colour, piece_type: PieceType) -> Bitboard {
        self.our(us) & self.pieces(piece_type)
    }

    #[inline]
    pub fn their_pieces(&self, us: Colour, piece_type: PieceType) -> Bitboard {
        self.their(us) & self.pieces(piece_type)
    }

    #[inline]
    pub fn king_square(&self, colour: Colour) -> Square {
        let bitboard = self.our_pieces(colour, PieceType::King);

        debug_assert!(!bitboard.is_empty());

        unsafe { Square::from_index_unchecked(bitboard.bits().trailing_zeros() as u8) }
    }

    #[must_use = "an Undo must be passed to unmake_move or the position will be left permanently mutated"]
    pub fn make_move(&mut self, mv: Move) -> Undo {
        let undo = Undo {
            castling_rights: self.position.castling_rights,
            en_passant: self.position.en_passant,
            halfmove_clock: self.position.halfmove_clock,
        };

        let us = self.side_to_move();
        let them = us.other();

        let from = mv.from();
        let to = mv.to();
        let kind = mv.kind();
        let piece = mv.piece();

        if let Some(old_ep) = self.position.en_passant {
            self.hash ^= zobrist::en_passant_file_key(old_ep.file());
        }

        self.position.en_passant = None;

        if piece == PieceType::Pawn || kind.is_capture() {
            self.position.halfmove_clock.clear();
        } else {
            self.position.halfmove_clock.increment();
        }

        match kind {
            MoveKind::Quiet => {
                self.move_piece(us, piece, from, to);
            }
            MoveKind::DoublePawnPush => {
                self.move_piece(us, piece, from, to);
                let ep_index = ((from.bits() as u16 + to.bits() as u16) >> 1) as u8;

                // SAFETY: the midpoint of a two-square pawn push is always
                // a valid board index.
                let ep_square = unsafe { Square::from_index_unchecked(ep_index) };
                self.position.en_passant = Some(ep_square);

                self.hash ^= zobrist::en_passant_file_key(ep_square.file());
            }
            MoveKind::Capture => {
                let captured = mv.captured().unwrap();
                self.remove_piece(them, captured, to);
                self.move_piece(us, piece, from, to);
            }
            MoveKind::EnPassant => {
                let offset = if us == Colour::White { 8i8 } else { -8i8 };
                let captured_index = (to.bits() as i8 - offset) as u8;

                // SAFETY: always one rank behind the en passant target.
                let captured_square = unsafe { Square::from_index_unchecked(captured_index) };
                self.remove_piece(them, PieceType::Pawn, captured_square);
                self.move_piece(us, PieceType::Pawn, from, to);
            }
            MoveKind::Promotion => {
                self.remove_piece(us, PieceType::Pawn, from);
                self.add_piece(us, mv.promotion().unwrap(), to);
            }
            MoveKind::PromotionCapture => {
                let captured = mv.captured().unwrap();
                self.remove_piece(them, captured, to);
                self.remove_piece(us, PieceType::Pawn, from);
                self.add_piece(us, mv.promotion().unwrap(), to);
            }
            MoveKind::KingsideCastle | MoveKind::QueensideCastle => {
                self.move_piece(us, PieceType::King, from, to);
                let (rook_from, rook_to) = castling_rook_squares(us, kind);
                self.move_piece(us, PieceType::Rook, rook_from, rook_to);
            }
        }

        let old_rights = self.position.castling_rights;
        let new_rights = old_rights.updated_by(from, to);

        let changed = old_rights ^ new_rights;

        for right in CastlingRights::RIGHTS {
            if changed.contains(right) {
                self.hash ^= zobrist::castling_key(right);
            }
        }

        self.position.castling_rights = new_rights;

        self.position.side_to_move = them;
        self.hash ^= zobrist::side_to_move_key();

        if us == Colour::Black {
            self.position.fullmove_number.increment();
        }

        self.occupancy = self.colour_bitboard[Colour::White] | self.colour_bitboard[Colour::Black];

        undo
    }

    pub fn unmake_move(&mut self, mv: Move, undo: Undo) {
        let them = self.side_to_move();
        let us = them.other();

        let from = mv.from();
        let to = mv.to();
        let kind = mv.kind();
        let piece = mv.piece();

        match kind {
            MoveKind::Quiet | MoveKind::DoublePawnPush => self.move_piece(us, piece, to, from),
            MoveKind::Capture => {
                let captured = mv.captured().unwrap();
                self.move_piece(us, piece, to, from);
                self.add_piece(them, captured, to);
            }
            MoveKind::EnPassant => {
                let offset = if us == Colour::White { 8i8 } else { -8i8 };
                let captured_index = (to.bits() as i8 - offset) as u8;

                // SAFETY: always one rank behind the en passant target.
                let captured_square = unsafe { Square::from_index_unchecked(captured_index) };

                self.move_piece(us, PieceType::Pawn, to, from);
                self.add_piece(them, PieceType::Pawn, captured_square);
            }
            MoveKind::Promotion => {
                self.remove_piece(us, mv.promotion().unwrap(), to);
                self.add_piece(us, PieceType::Pawn, from);
            }
            MoveKind::PromotionCapture => {
                let captured = mv.captured().unwrap();

                self.remove_piece(us, mv.promotion().unwrap(), to);
                self.add_piece(us, PieceType::Pawn, from);
                self.add_piece(them, captured, to);
            }
            MoveKind::KingsideCastle | MoveKind::QueensideCastle => {
                self.move_piece(us, PieceType::King, to, from);

                let (rook_from, rook_to) = castling_rook_squares(us, kind);

                self.move_piece(us, PieceType::Rook, rook_to, rook_from);
            }
        }

        let current_rights = self.position.castling_rights;
        let changed = current_rights ^ undo.castling_rights;

        for right in CastlingRights::RIGHTS {
            if changed.contains(right) {
                self.hash ^= zobrist::castling_key(right);
            }
        }

        if let MoveKind::DoublePawnPush = kind {
            let ep_index = ((from.bits() as u16 + to.bits() as u16) >> 1) as u8;

            // SAFETY: same midpoint computed (and proven valid) in make_move.
            let ep_square = unsafe { Square::from_index_unchecked(ep_index) };
            self.hash ^= zobrist::en_passant_file_key(ep_square.file());
        }

        if let Some(old_ep) = undo.en_passant {
            self.hash ^= zobrist::en_passant_file_key(old_ep.file());
        }

        self.position.castling_rights = undo.castling_rights;
        self.position.en_passant = undo.en_passant;
        self.position.halfmove_clock = undo.halfmove_clock;
        self.position.side_to_move = us;

        self.hash ^= zobrist::side_to_move_key();

        if us == Colour::Black {
            self.position.fullmove_number.decrement();
        }

        self.occupancy = self.colour_bitboard[Colour::White] | self.colour_bitboard[Colour::Black];
    }

    fn move_piece(&mut self, colour: Colour, piece: PieceType, from: Square, to: Square) {
        self.remove_piece(colour, piece, from);
        self.add_piece(colour, piece, to);
    }

    fn add_piece(&mut self, colour: Colour, piece: PieceType, square: Square) {
        self.colour_bitboard[colour] |= square;
        self.piece_bitboard[piece] |= square;
        self.position.board[square] = Piece::new(piece, colour);

        self.hash ^= zobrist::piece_square_key(colour, piece, square);
    }

    fn remove_piece(&mut self, colour: Colour, piece: PieceType, square: Square) {
        let clear = Bitboard::new(!(1u64 << square.bits()));
        self.colour_bitboard[colour] &= clear;
        self.piece_bitboard[piece] &= clear;
        self.position.board[square] = Piece::NONE;

        self.hash ^= zobrist::piece_square_key(colour, piece, square);
    }
}

fn castling_rook_squares(colour: Colour, kind: MoveKind) -> (Square, Square) {
    match (colour, kind) {
        (Colour::White, MoveKind::KingsideCastle) => (Square::H1, Square::F1),
        (Colour::White, MoveKind::QueensideCastle) => (Square::A1, Square::D1),
        (Colour::Black, MoveKind::KingsideCastle) => (Square::H8, Square::F8),
        (Colour::Black, MoveKind::QueensideCastle) => (Square::A8, Square::D8),
        _ => unreachable!("castling_rook_squares called with a non-castling MoveKind"),
    }
}
