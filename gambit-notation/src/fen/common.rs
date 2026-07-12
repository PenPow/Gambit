use crate::fen::error::FenError;
use gambit_models::location::file::File;
use gambit_models::location::rank::Rank;
use gambit_models::location::square::Square;
use gambit_models::mailbox::Mailbox;
use gambit_models::piece::Piece;
use gambit_models::piece::colour::Colour;
use gambit_models::position::{FullmoveNumber, HalfmoveClock};
use std::str::FromStr;

pub(in crate::fen) fn parse_board(input: &str) -> Result<Mailbox, FenError> {
    let mut mailbox = Mailbox::default();

    for (rank_index, rank_str) in input.split('/').enumerate() {
        if rank_str.len() > 8 {
            return Err(FenError::InvalidRankLength(rank_str.len() as u8));
        }

        let rank = Rank::try_from((7 - rank_index) as u8)
            .or(Err(FenError::InvalidPlacement("invalid rank".to_string())))?;

        let mut file_index = 0u8;
        for ch in rank_str.chars() {
            if ch.is_ascii_digit() {
                file_index += ch.to_digit(10).unwrap() as u8;
                if file_index > 8 {
                    return Err(FenError::InvalidPlacement(
                        "file overflowed off edge".to_string(),
                    ));
                }
            } else {
                if file_index > 8 {
                    return Err(FenError::InvalidPlacement(
                        "file overflowed off edge".to_string(),
                    ));
                }

                let file = File::from_index(file_index);
                let square = Square::from_coordinates((file, rank));
                let piece =
                    Piece::try_from(ch).map_err(|_| FenError::InvalidPlacement(ch.to_string()))?;

                mailbox[square] = piece;

                file_index += 1;
            }
        }
    }

    Ok(mailbox)
}

pub(in crate::fen) fn parse_side_to_move(input: &str) -> Result<Colour, FenError> {
    match input {
        "w" => Ok(Colour::White),
        "b" => Ok(Colour::Black),
        _ => Err(FenError::InvalidColour(input.chars().next().unwrap_or('?'))),
    }
}

pub(in crate::fen) fn parse_en_passant(input: &str) -> Result<Option<Square>, FenError> {
    if input == "-" {
        Ok(None)
    } else {
        Square::from_str(input)
            .map(Some)
            .map_err(FenError::InvalidSquare)
    }
}

pub(in crate::fen) fn parse_halfmove_clock(input: &str) -> Result<HalfmoveClock, FenError> {
    input
        .parse()
        .map(HalfmoveClock)
        .map_err(|_| FenError::InvalidHalfmoveClock(input.to_string()))
}

pub(in crate::fen) fn parse_fullmove_number(input: &str) -> Result<FullmoveNumber, FenError> {
    input
        .parse()
        .map(FullmoveNumber)
        .map_err(|_| FenError::InvalidFullmoveNumber(input.to_string()))
}
