use crate::attacks;
use crate::generator::Context;
use crate::list::MoveList;
use crate::state::State;
use gambit_models::bitboard::Bitboard;
use gambit_models::location::rank::Rank;
use gambit_models::location::square::Square;
use gambit_models::moves::builder::MoveBuilder;
use gambit_models::piece::colour::Colour;
use gambit_models::piece::piece_type::PieceType;

#[inline]
fn push_offset(colour: Colour) -> i8 {
    match colour {
        Colour::White => 8,
        Colour::Black => -8,
    }
}

#[inline]
fn is_promotion_rank(square: Square, colour: Colour) -> bool {
    let (_, rank) = square.coordinates();
    match colour {
        Colour::White => rank == Rank::Eight,
        Colour::Black => rank == Rank::One,
    }
}

#[inline]
fn is_start_rank(square: Square, colour: Colour) -> bool {
    let (_, rank) = square.coordinates();
    match colour {
        Colour::White => rank == Rank::Two,
        Colour::Black => rank == Rank::Seven,
    }
}

fn push_promotions(list: &mut MoveList, from: Square, to: Square) {
    for promotion in PieceType::PROMOTION_TARGETS {
        list.push(MoveBuilder::promotion(from, to, promotion).build());
    }
}

fn push_promotion_captures(list: &mut MoveList, from: Square, to: Square, captured: PieceType) {
    for promotion in PieceType::PROMOTION_TARGETS {
        list.push(MoveBuilder::promotion_capture(from, to, promotion, captured).build());
    }
}

pub(crate) fn generate(state: &State, ctx: &Context, list: &mut MoveList) {
    let offset = push_offset(ctx.us);

    let pawns = state.our_pieces(ctx.us, PieceType::Pawn);

    for from in pawns {
        let pin_line = if ctx.pinned.contains(from) {
            attacks::line_through(ctx.king_square, from)
        } else {
            Bitboard::UNIVERSE
        };

        generate_pushes(ctx, list, from, offset, pin_line);
        generate_captures(state, ctx, list, from, pin_line);
        generate_en_passant(state, ctx, list, from, offset, pin_line);
    }
}

fn generate_pushes(
    ctx: &Context,
    list: &mut MoveList,
    from: Square,
    offset: i8,
    pin_line: Bitboard,
) {
    let single_index = from.bits() as i16 + offset as i16;
    if !(0..64).contains(&single_index) {
        return;
    }

    // SAFETY: single_index was just bounds-checked into 0..64.
    let single_to = unsafe { Square::from_index_unchecked(single_index as u8) };

    if ctx.occupancy.contains(single_to) {
        return;
    }

    if ctx.target_mask.contains(single_to) && pin_line.contains(single_to) {
        if is_promotion_rank(single_to, ctx.us) {
            push_promotions(list, from, single_to);
        } else {
            list.push(MoveBuilder::quiet(from, single_to, PieceType::Pawn).build());
        }
    }

    if is_start_rank(from, ctx.us) {
        let double_index = single_index + offset as i16;
        // SAFETY: a pawn on its start rank pushing two squares forward always lands in 0..64
        let double_to = unsafe { Square::from_index_unchecked(double_index as u8) };

        if !ctx.occupancy.contains(double_to)
            && ctx.target_mask.contains(double_to)
            && pin_line.contains(double_to)
        {
            list.push(MoveBuilder::double_pawn_push(from, double_to).build());
        }
    }
}

fn generate_captures(
    state: &State,
    ctx: &Context,
    list: &mut MoveList,
    from: Square,
    pin_line: Bitboard,
) {
    let targets = attacks::pawn_attacks(from, ctx.us) & ctx.enemy & ctx.target_mask & pin_line;

    for to in targets {
        let captured = state.piece_at(to).piece_type().unwrap();

        if is_promotion_rank(to, ctx.us) {
            push_promotion_captures(list, from, to, captured);
        } else {
            list.push(MoveBuilder::capture(from, to, PieceType::Pawn, captured).build());
        }
    }
}
fn generate_en_passant(
    state: &State,
    ctx: &Context,
    list: &mut MoveList,
    from: Square,
    offset: i8,
    pin_line: Bitboard,
) {
    let Some(ep_square) = state.position().en_passant else {
        return;
    };

    if !attacks::pawn_attacks(from, ctx.us).contains(ep_square) {
        return;
    }

    let captured_index = (ep_square.bits() as i16 - offset as i16) as u8;

    // SAFETY: the en passant target is always one rank behind a pawn that
    // just double-pushed, so this index is always in 0..64.
    let captured_square = unsafe { Square::from_index_unchecked(captured_index) };

    let resolves_check =
        ctx.target_mask.contains(ep_square) || ctx.target_mask.contains(captured_square);
    let stays_on_pin_line = pin_line.contains(ep_square);

    if !resolves_check || !stays_on_pin_line {
        return;
    }

    let mut occ_bits = ctx.occupancy.bits();
    occ_bits &= !(1u64 << from.bits());
    occ_bits &= !(1u64 << captured_square.bits());
    occ_bits |= 1u64 << ep_square.bits();
    let occ_after = Bitboard::new(occ_bits);

    let enemy_rooks_queens =
        state.their_pieces(ctx.us, PieceType::Rook) | state.their_pieces(ctx.us, PieceType::Queen);
    let exposed = attacks::rook_attacks(ctx.king_square, occ_after) & enemy_rooks_queens;

    if exposed.is_empty() {
        list.push(MoveBuilder::en_passant(from, ep_square).build());
    }
}
