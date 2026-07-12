use crate::algebraic::error::AlgebraicError;
use gambit_models::location::square::Square;
use gambit_models::moves::Move;
use gambit_models::piece::piece_type::PieceType;
use gambit_movegen::generator::legal::generate;
use gambit_movegen::list::MoveList;
use gambit_movegen::state::State;
use std::str::FromStr;

pub fn parse(input: &str, state: &State) -> Result<Move, AlgebraicError> {
    let len = input.len();
    if len != 4 && len != 5 {
        return Err(AlgebraicError::Malformed(input.to_string()));
    }

    let from = Square::from_str(&input[0..2])
        .map_err(|_| AlgebraicError::InvalidSquare(input[0..2].to_string()))?;
    let to = Square::from_str(&input[2..4])
        .map_err(|_| AlgebraicError::InvalidSquare(input[2..4].to_string()))?;

    let promotion = if len == 5 {
        let promotion_char = input.as_bytes()[4] as char;

        Some(
            PieceType::try_from(promotion_char)
                .map_err(|_| AlgebraicError::InvalidPromotion(promotion_char))?,
        )
    } else {
        None
    };

    let mut list = MoveList::new();
    generate(state, &mut list);

    list.iter()
        .copied()
        .find(|mv| mv.from() == from && mv.to() == to && mv.promotion() == promotion)
        .ok_or_else(|| AlgebraicError::NoSuchMove(input.to_string()))
}
