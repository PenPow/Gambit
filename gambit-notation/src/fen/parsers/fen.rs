use crate::fen::FenLike;
use crate::fen::common::{
    parse_board, parse_en_passant, parse_fullmove_number, parse_halfmove_clock, parse_side_to_move,
};
use crate::fen::error::FenError;
use gambit_models::location::square::Square;
use gambit_models::mailbox::Mailbox;
use gambit_models::movement::castling::rights::CastlingRights;
use gambit_models::piece::colour::Colour;
use gambit_models::position::{FullmoveNumber, HalfmoveClock, Position};
use std::str::FromStr;

/// A parsed FEN string, providing access to each standard field.
///
/// # Examples
///
/// ```rust
/// # use gambit_notation::fen::parsers::Fen;
/// # use gambit_notation::fen::FenLike;
/// let fen = Fen::parse(
///     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
/// ).unwrap();
/// assert_eq!(fen.fullmove_number().value(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct Fen {
    position: Position,
}

impl Fen {
    fn parse_castling_rights(input: &str) -> Result<CastlingRights, FenError> {
        CastlingRights::from_str(input).map_err(FenError::InvalidCastling)
    }
}

impl FenLike for Fen {
    fn parse(input: &str) -> Result<Self, FenError>
    where
        Self: Sized,
    {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 6 {
            return Err(FenError::WrongFieldCount(parts.len()));
        }

        let board = parse_board(parts[0])?;
        let side_to_move = parse_side_to_move(parts[1])?;
        let castling_rights = Self::parse_castling_rights(parts[2])?;
        let en_passant = parse_en_passant(parts[3])?;
        let halfmove_clock = parse_halfmove_clock(parts[4])?;
        let fullmove_number = parse_fullmove_number(parts[5])?;

        let position = Position {
            board,
            side_to_move,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
        };

        Ok(Self { position })
    }

    fn position(&self) -> Position {
        self.position
    }

    fn board(&self) -> Mailbox {
        self.position.board
    }

    fn side_to_move(&self) -> Colour {
        self.position.side_to_move
    }

    fn castling_rights(&self) -> CastlingRights {
        self.position.castling_rights
    }

    fn en_passant(&self) -> Option<Square> {
        self.position.en_passant
    }

    fn halfmove_clock(&self) -> HalfmoveClock {
        self.position.halfmove_clock
    }

    fn fullmove_number(&self) -> FullmoveNumber {
        self.position.fullmove_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gambit_models::piece::Piece;
    use gambit_models::piece::colour::Colour::*;
    use gambit_models::piece::piece_type::PieceType;

    #[test]
    fn parse_starting_position() {
        let fen = Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(fen.board(), Mailbox::STARTING_POSITION);
        assert_eq!(fen.side_to_move(), White);
        assert_eq!(fen.castling_rights(), CastlingRights::ALL);
        assert_eq!(fen.en_passant(), None);
    }

    #[test]
    fn parse_black_to_move() {
        let fen =
            Fen::parse("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
        assert_eq!(fen.side_to_move(), Black);
    }

    #[test]
    fn parse_en_passant_square() {
        let fen =
            Fen::parse("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
        assert_eq!(fen.en_passant(), Some(Square::E3));
    }

    #[test]
    fn parse_halfmove_clock() {
        let fen = Fen::parse("8/8/8/8/8/8/8/8 w - - 42 1").unwrap();
        assert_eq!(fen.halfmove_clock(), HalfmoveClock(42));
    }

    #[test]
    fn parse_fullmove_number() {
        let fen = Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 123").unwrap();
        assert_eq!(fen.fullmove_number(), FullmoveNumber(123));
    }

    #[test]
    fn parse_castling_k_only() {
        let fen = Fen::parse("r3k2r/8/8/8/8/8/8/R3K2R w Kk - 0 1").unwrap();
        let expected = CastlingRights::WHITE_KINGSIDE | CastlingRights::BLACK_KINGSIDE;
        assert_eq!(fen.castling_rights(), expected);
    }

    #[test]
    fn parse_castling_q_only() {
        let fen = Fen::parse("r3k2r/8/8/8/8/8/8/R3K2R w Qq - 0 1").unwrap();
        let expected = CastlingRights::WHITE_QUEENSIDE | CastlingRights::BLACK_QUEENSIDE;
        assert_eq!(fen.castling_rights(), expected);
    }

    #[test]
    fn parse_castling_none() {
        let fen = Fen::parse("r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1").unwrap();
        assert_eq!(fen.castling_rights(), CastlingRights::NONE);
    }

    #[test]
    fn parse_kings_only_endgame() {
        let fen = Fen::parse("4k3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        let mut expected = Mailbox::empty();
        expected[Square::E8] = Piece::BLACK_KING;
        expected[Square::E1] = Piece::WHITE_KING;
        assert_eq!(fen.board(), expected);
        assert_eq!(fen.side_to_move(), White);
        assert_eq!(fen.castling_rights(), CastlingRights::NONE);
        assert_eq!(fen.en_passant(), None);
    }

    #[test]
    fn error_wrong_field_count_too_few() {
        let err = Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0").unwrap_err();
        assert_eq!(err, FenError::WrongFieldCount(5));
    }

    #[test]
    fn error_wrong_field_count_too_many() {
        let err = Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 extra")
            .unwrap_err();
        assert_eq!(err, FenError::WrongFieldCount(7));
    }

    #[test]
    fn error_wrong_field_count_empty() {
        let err = Fen::parse("").unwrap_err();
        assert_eq!(err, FenError::WrongFieldCount(0));
    }

    #[test]
    fn error_invalid_side_to_move() {
        let err =
            Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1").unwrap_err();
        assert_eq!(err, FenError::InvalidColour('x'));
    }

    #[test]
    fn error_invalid_castling_char() {
        let err = Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kx - 0 1").unwrap_err();
        assert!(matches!(err, FenError::InvalidCastling(_)));
    }

    #[test]
    fn error_invalid_en_passant_square() {
        let err =
            Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e9 0 1").unwrap_err();
        assert!(matches!(err, FenError::InvalidSquare(_)));
    }

    #[test]
    fn error_invalid_en_passant_bad_format() {
        let err =
            Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq blah 0 1").unwrap_err();
        assert!(matches!(err, FenError::InvalidSquare(_)));
    }

    #[test]
    fn error_invalid_halfmove_clock_negative() {
        let err =
            Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - -1 1").unwrap_err();
        assert_eq!(err, FenError::InvalidHalfmoveClock("-1".to_string()));
    }

    #[test]
    fn error_invalid_halfmove_clock_non_numeric() {
        let err =
            Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - abc 1").unwrap_err();
        assert_eq!(err, FenError::InvalidHalfmoveClock("abc".to_string()));
    }

    #[test]
    fn error_invalid_fullmove_number_negative() {
        let err =
            Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 -1").unwrap_err();
        assert_eq!(err, FenError::InvalidFullmoveNumber("-1".to_string()));
    }

    #[test]
    fn error_invalid_fullmove_number_overflow() {
        let err =
            Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 65536").unwrap_err();
        assert_eq!(err, FenError::InvalidFullmoveNumber("65536".to_string()));
    }

    #[test]
    fn error_invalid_fullmove_number_non_numeric() {
        let err =
            Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 abc").unwrap_err();
        assert_eq!(err, FenError::InvalidFullmoveNumber("abc".to_string()));
    }

    #[test]
    fn parse_board_rank_with_overflow() {
        let err =
            Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNRR w KQkq - 0 1").unwrap_err();
        assert_eq!(err, FenError::InvalidRankLength(9));
    }

    #[test]
    fn parse_board_invalid_piece_char() {
        let err =
            Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPX/RNBQKBNR w KQkq - 0 1").unwrap_err();
        assert_eq!(err, FenError::InvalidPlacement("X".to_string()));
    }

    #[test]
    fn parse_board_file_overflow_too_many_pieces() {
        let err =
            Fen::parse("rnbqkbnr/pppppppp/8/8/8/9/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap_err();
        assert_eq!(
            err,
            FenError::InvalidPlacement("file overflowed off edge".to_string())
        );
    }

    #[test]
    fn parse_empty_board() {
        let fen = Fen::parse("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();
        assert_eq!(fen.board(), Mailbox::empty());
    }

    #[test]
    fn parse_board_exactly_64_chars_expanded() {
        let fen = Fen::parse(
            "pppppppp/pppppppp/pppppppp/pppppppp/pppppppp/pppppppp/pppppppp/pppppppp b - - 0 1",
        )
        .unwrap();
        let board = fen.board();
        for (_, piece) in board.iter() {
            assert_eq!(piece.piece_type(), Some(PieceType::Pawn));
            assert_eq!(piece.colour(), Some(Black));
        }
    }

    #[test]
    fn parse_side_to_move_white() {
        assert_eq!(crate::fen::common::parse_side_to_move("w").unwrap(), White);
    }

    #[test]
    fn parse_side_to_move_black() {
        assert_eq!(crate::fen::common::parse_side_to_move("b").unwrap(), Black);
    }

    #[test]
    fn parse_side_to_move_invalid() {
        assert_eq!(
            crate::fen::common::parse_side_to_move("W").unwrap_err(),
            FenError::InvalidColour('W')
        );
        assert_eq!(
            crate::fen::common::parse_side_to_move("").unwrap_err(),
            FenError::InvalidColour('?')
        );
    }

    #[test]
    fn parse_en_passant_dash() {
        assert_eq!(crate::fen::common::parse_en_passant("-").unwrap(), None);
    }

    #[test]
    fn parse_en_passant_valid_square() {
        assert_eq!(
            crate::fen::common::parse_en_passant("a3").unwrap(),
            Some(Square::A3)
        );
        assert_eq!(
            crate::fen::common::parse_en_passant("h6").unwrap(),
            Some(Square::H6)
        );
    }

    #[test]
    fn parse_clock_halfmove_zero() {
        assert_eq!(
            crate::fen::common::parse_halfmove_clock("0").unwrap(),
            HalfmoveClock(0)
        );
    }

    #[test]
    fn parse_clock_halfmove_max() {
        assert_eq!(
            crate::fen::common::parse_halfmove_clock("255").unwrap(),
            HalfmoveClock(255)
        );
    }

    #[test]
    fn parse_number_fullmove_one() {
        assert_eq!(
            crate::fen::common::parse_fullmove_number("1").unwrap(),
            FullmoveNumber(1)
        );
    }

    #[test]
    fn parse_number_fullmove_large() {
        assert_eq!(
            crate::fen::common::parse_fullmove_number("65535").unwrap(),
            FullmoveNumber(65535)
        );
    }

    #[test]
    fn parse_number_fullmove_invalid_negative() {
        let err = crate::fen::common::parse_fullmove_number("-1").unwrap_err();
        assert_eq!(err, FenError::InvalidFullmoveNumber("-1".to_string()));
    }

    #[test]
    fn parse_number_fullmove_invalid_overflow() {
        let err = crate::fen::common::parse_fullmove_number("65536").unwrap_err();
        assert_eq!(err, FenError::InvalidFullmoveNumber("65536".to_string()));
    }
}
