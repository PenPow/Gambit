use gambit_models::piece::colour::Colour;
use gambit_models::piece::piece_type::PieceType;
use gambit_movegen::state::State;

fn get_piece_score(piece_type: PieceType) -> i32 {
    match piece_type {
        PieceType::Pawn => 100,
        PieceType::Knight | PieceType::Bishop => 300,
        PieceType::Rook => 500,
        PieceType::Queen => 900,
        _ => 0,
    }
}

fn count_material(state: &State, side: Colour) -> i32 {
    let mut score = 0;

    score +=
        get_piece_score(PieceType::Pawn) * state.our_pieces(side, PieceType::Pawn).count() as i32;
    score += get_piece_score(PieceType::Knight)
        * state.our_pieces(side, PieceType::Knight).count() as i32;
    score += get_piece_score(PieceType::Bishop)
        * state.our_pieces(side, PieceType::Bishop).count() as i32;
    score +=
        get_piece_score(PieceType::Rook) * state.our_pieces(side, PieceType::Rook).count() as i32;
    score +=
        get_piece_score(PieceType::Queen) * state.our_pieces(side, PieceType::Queen).count() as i32;

    score
}

pub fn evaluate(state: &State) -> i32 {
    let white_eval = count_material(state, Colour::White);
    let black_eval = count_material(state, Colour::Black);

    let perspective = if state.side_to_move() == Colour::White {
        1
    } else {
        -1
    };

    (white_eval - black_eval) * perspective
}
