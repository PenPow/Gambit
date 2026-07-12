use criterion::{Criterion, criterion_group, criterion_main};
use gambit_models::location::square::Square;
use gambit_models::movement::castling::side::CastlingSide;
use gambit_models::moves::Move;
use gambit_models::moves::builder::MoveBuilder;
use gambit_models::piece::piece_type::PieceType;

fn bench_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_construction");

    group.bench_function("quiet", |b| {
        b.iter(|| {
            MoveBuilder::quiet(
                std::hint::black_box(Square::E2),
                std::hint::black_box(Square::E4),
                std::hint::black_box(PieceType::Pawn),
            )
            .build()
        })
    });

    group.bench_function("capture", |b| {
        b.iter(|| {
            MoveBuilder::capture(
                std::hint::black_box(Square::E4),
                std::hint::black_box(Square::D5),
                std::hint::black_box(PieceType::Pawn),
                std::hint::black_box(PieceType::Knight),
            )
            .build()
        })
    });

    group.bench_function("double_pawn_push", |b| {
        b.iter(|| {
            MoveBuilder::double_pawn_push(
                std::hint::black_box(Square::E2),
                std::hint::black_box(Square::E4),
            )
            .build()
        })
    });

    group.bench_function("en_passant", |b| {
        b.iter(|| {
            MoveBuilder::en_passant(
                std::hint::black_box(Square::E5),
                std::hint::black_box(Square::D6),
            )
            .build()
        })
    });

    group.bench_function("castle_kingside", |b| {
        b.iter(|| {
            MoveBuilder::castle(
                std::hint::black_box(Square::E1),
                std::hint::black_box(Square::G1),
                std::hint::black_box(CastlingSide::Kingside),
            )
            .build()
        })
    });

    group.bench_function("promotion", |b| {
        b.iter(|| {
            MoveBuilder::promotion(
                std::hint::black_box(Square::E7),
                std::hint::black_box(Square::E8),
                std::hint::black_box(PieceType::Queen),
            )
            .build()
        })
    });

    group.bench_function("promotion_capture", |b| {
        b.iter(|| {
            MoveBuilder::promotion_capture(
                std::hint::black_box(Square::E7),
                std::hint::black_box(Square::D8),
                std::hint::black_box(PieceType::Knight),
                std::hint::black_box(PieceType::Queen),
            )
            .build()
        })
    });

    // Simulate generating all 4 promotion variants — common in movegen
    group.bench_function("all_promotion_variants", |b| {
        b.iter(|| {
            PieceType::PROMOTION_TARGETS.map(|promo| {
                MoveBuilder::promotion(
                    std::hint::black_box(Square::E7),
                    std::hint::black_box(Square::E8),
                    promo,
                )
                .build()
            })
        })
    });

    group.finish();
}

fn bench_decoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_decoding");

    let quiet = MoveBuilder::quiet(Square::E2, Square::E4, PieceType::Pawn).build();
    let capture =
        MoveBuilder::capture(Square::E4, Square::D5, PieceType::Pawn, PieceType::Knight).build();
    let promo_cap =
        MoveBuilder::promotion_capture(Square::E7, Square::D8, PieceType::Rook, PieceType::Queen)
            .build();

    group.bench_function("from", |b| b.iter(|| std::hint::black_box(quiet).from()));

    group.bench_function("to", |b| b.iter(|| std::hint::black_box(quiet).to()));

    group.bench_function("kind", |b| b.iter(|| std::hint::black_box(quiet).kind()));

    group.bench_function("piece", |b| b.iter(|| std::hint::black_box(quiet).piece()));

    group.bench_function("captured_some", |b| {
        b.iter(|| std::hint::black_box(capture).captured())
    });

    group.bench_function("captured_none", |b| {
        b.iter(|| std::hint::black_box(quiet).captured())
    });

    group.bench_function("promotion_some", |b| {
        b.iter(|| std::hint::black_box(promo_cap).promotion())
    });

    // Decode all fields in sequence — models the cost of a full move inspection
    group.bench_function("decode_all_fields", |b| {
        b.iter(|| {
            let mv = std::hint::black_box(promo_cap);
            (
                mv.from(),
                mv.to(),
                mv.kind(),
                mv.piece(),
                mv.captured(),
                mv.promotion(),
            )
        })
    });

    group.finish();
}

fn bench_classification(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_classification");

    let moves = [
        MoveBuilder::quiet(Square::E2, Square::E4, PieceType::Pawn).build(),
        MoveBuilder::capture(Square::E4, Square::D5, PieceType::Pawn, PieceType::Knight).build(),
        MoveBuilder::promotion(Square::E7, Square::E8, PieceType::Queen).build(),
        MoveBuilder::castle(Square::E1, Square::G1, CastlingSide::Kingside).build(),
        MoveBuilder::en_passant(Square::E5, Square::D6).build(),
    ];

    group.bench_function("is_capture_mixed_list", |b| {
        b.iter(|| {
            moves
                .iter()
                .filter(|&&mv| std::hint::black_box(mv).is_capture())
                .count()
        })
    });

    group.bench_function("is_promotion_mixed_list", |b| {
        b.iter(|| {
            moves
                .iter()
                .filter(|&&mv| std::hint::black_box(mv).is_promotion())
                .count()
        })
    });

    group.bench_function("is_null", |b| {
        b.iter(|| std::hint::black_box(Move::NULL).is_null())
    });

    group.finish();
}

fn bench_try_from(c: &mut Criterion) {
    let valid = MoveBuilder::capture(Square::E4, Square::D5, PieceType::Pawn, PieceType::Knight)
        .build()
        .bits();

    c.bench_function("try_from_u32_valid", |b| {
        b.iter(|| Move::try_from(std::hint::black_box(valid)))
    });
}

fn bench_display(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_display");

    let quiet = MoveBuilder::quiet(Square::E2, Square::E4, PieceType::Pawn).build();
    let promo = MoveBuilder::promotion(Square::E7, Square::E8, PieceType::Queen).build();

    group.bench_function("display_quiet", |b| {
        b.iter(|| std::hint::black_box(quiet).to_string())
    });

    group.bench_function("display_promotion", |b| {
        b.iter(|| std::hint::black_box(promo).to_string())
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_construction,
    bench_decoding,
    bench_classification,
    bench_try_from,
    bench_display,
);

criterion_main!(benches);
