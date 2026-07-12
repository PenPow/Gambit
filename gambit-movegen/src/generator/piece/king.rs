use crate::generator::Context;
use crate::list::MoveList;
use crate::state::State;
use crate::{attacks, masks};
use gambit_models::bitboard::Bitboard;
use gambit_models::location::square::Square;
use gambit_models::movement::castling::side::CastlingSide;
use gambit_models::moves::builder::MoveBuilder;
use gambit_models::piece::colour::Colour;
use gambit_models::piece::piece_type::PieceType;

pub(crate) fn generate(state: &State, ctx: &Context, list: &mut MoveList) {
    let from = ctx.king_square;

    let occupancy_without_king = Bitboard::new(ctx.occupancy.bits() & !(1u64 << from.bits()));

    let targets = attacks::king_attacks(from) & !ctx.own;

    for to in targets {
        if masks::attackers_to(state, to, occupancy_without_king, ctx.us.other()).is_empty() {
            if ctx.enemy.contains(to) {
                let captured = state.piece_at(to).piece_type().unwrap();
                list.push(MoveBuilder::capture(from, to, PieceType::King, captured).build());
            } else {
                list.push(MoveBuilder::quiet(from, to, PieceType::King).build());
            }
        }
    }

    if ctx.check_count == 0 {
        generate_castling(state, ctx, list);
    }
}

fn generate_castling(state: &State, ctx: &Context, list: &mut MoveList) {
    let rights = state.position().castling_rights;

    if rights.has(ctx.us, CastlingSide::Kingside) {
        try_castle(state, ctx, list, CastlingSide::Kingside);
    }
    if rights.has(ctx.us, CastlingSide::Queenside) {
        try_castle(state, ctx, list, CastlingSide::Queenside);
    }
}

fn try_castle(state: &State, ctx: &Context, list: &mut MoveList, side: CastlingSide) {
    let from = ctx.king_square;

    let (king_to, occupancy_empty, king_path): (Square, &[Square], &[Square]) = match (ctx.us, side)
    {
        (Colour::White, CastlingSide::Kingside) => (
            Square::G1,
            &[Square::F1, Square::G1],
            &[Square::F1, Square::G1],
        ),
        (Colour::White, CastlingSide::Queenside) => (
            Square::C1,
            &[Square::D1, Square::C1, Square::B1],
            &[Square::D1, Square::C1],
        ),
        (Colour::Black, CastlingSide::Kingside) => (
            Square::G8,
            &[Square::F8, Square::G8],
            &[Square::F8, Square::G8],
        ),
        (Colour::Black, CastlingSide::Queenside) => (
            Square::C8,
            &[Square::D8, Square::C8, Square::B8],
            &[Square::D8, Square::C8],
        ),
    };

    if !occupancy_empty
        .iter()
        .all(|&sq| !ctx.occupancy.contains(sq))
    {
        return;
    }

    let occupancy_without_king = Bitboard::new(ctx.occupancy.bits() & !(1u64 << from.bits()));
    let enemy = ctx.us.other();

    let path_is_safe = king_path
        .iter()
        .all(|&sq| masks::attackers_to(state, sq, occupancy_without_king, enemy).is_empty());

    if path_is_safe {
        list.push(MoveBuilder::castle(from, king_to, side).build());
    }
}
