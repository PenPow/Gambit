use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use gambit_models::location::file::File;
use gambit_models::location::rank::Rank;
use gambit_models::location::square::Square;
use gambit_models::mailbox::Mailbox;
use gambit_models::piece::Piece;
use std::str::FromStr;

fn bench_square_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("square_construction");

    group.bench_function("from_index", |b| {
        b.iter(|| Square::from_index(std::hint::black_box(28u8)))
    });

    group.bench_function("from_coordinates", |b| {
        b.iter(|| {
            Square::from_coordinates((
                std::hint::black_box(File::E),
                std::hint::black_box(Rank::Four),
            ))
        })
    });

    group.bench_function("from_str", |b| {
        b.iter(|| Square::from_str(std::hint::black_box("e4")))
    });

    group.bench_function("try_from_u8", |b| {
        b.iter(|| Square::try_from(std::hint::black_box(28u8)))
    });

    group.bench_function("to_string", |b| {
        b.iter(|| std::hint::black_box(Square::E4).to_string())
    });

    group.finish();
}

fn bench_square_accessors(c: &mut Criterion) {
    let mut group = c.benchmark_group("square_accessors");

    group.bench_function("file", |b| {
        b.iter(|| std::hint::black_box(Square::E4).file())
    });

    group.bench_function("rank", |b| {
        b.iter(|| std::hint::black_box(Square::E4).rank())
    });

    group.bench_function("coordinates", |b| {
        b.iter(|| std::hint::black_box(Square::E4).coordinates())
    });

    group.bench_function("bitboard", |b| {
        b.iter(|| std::hint::black_box(Square::E4).bitboard())
    });

    group.bench_function("flip_rank", |b| {
        b.iter(|| std::hint::black_box(Square::E4).flip_rank())
    });

    group.bench_function("flip_file", |b| {
        b.iter(|| std::hint::black_box(Square::E4).flip_file())
    });

    group.finish();
}

fn bench_file_rank_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_rank_ops");

    group.bench_function("file_offset_in_bounds", |b| {
        b.iter(|| std::hint::black_box(File::E).offset(std::hint::black_box(2i8)))
    });

    group.bench_function("file_offset_out_of_bounds", |b| {
        b.iter(|| std::hint::black_box(File::H).offset(std::hint::black_box(1i8)))
    });

    group.bench_function("file_distance", |b| {
        b.iter(|| std::hint::black_box(File::A).distance(std::hint::black_box(File::H)))
    });

    group.bench_function("rank_offset_in_bounds", |b| {
        b.iter(|| std::hint::black_box(Rank::Four).offset(std::hint::black_box(2i8)))
    });

    group.bench_function("rank_offset_out_of_bounds", |b| {
        b.iter(|| std::hint::black_box(Rank::Eight).offset(std::hint::black_box(1i8)))
    });

    group.bench_function("rank_distance", |b| {
        b.iter(|| std::hint::black_box(Rank::One).distance(std::hint::black_box(Rank::Eight)))
    });

    group.bench_function("file_bitboard", |b| {
        b.iter(|| std::hint::black_box(File::E).bitboard())
    });

    group.bench_function("rank_bitboard", |b| {
        b.iter(|| std::hint::black_box(Rank::Four).bitboard())
    });

    group.finish();
}

fn bench_mailbox(c: &mut Criterion) {
    let mut group = c.benchmark_group("mailbox");

    let mut mailbox = Mailbox::default();

    mailbox[Square::E1] = Piece::WHITE_KING;
    mailbox[Square::E8] = Piece::BLACK_KING;
    mailbox[Square::D1] = Piece::WHITE_QUEEN;
    mailbox[Square::D8] = Piece::BLACK_QUEEN;
    for i in 0u8..8 {
        let file = File::from_index(i);
        mailbox[Square::from_coordinates((file, Rank::Two))] = Piece::WHITE_PAWN;
        mailbox[Square::from_coordinates((file, Rank::Seven))] = Piece::BLACK_PAWN;
    }

    group.bench_function("iter_all_squares", |b| {
        b.iter(|| {
            std::hint::black_box(mailbox)
                .iter()
                .map(|(_, piece)| piece.bits() as u32)
                .sum::<u32>()
        })
    });

    group.bench_function("iter_pieces_only", |b| {
        b.iter(|| std::hint::black_box(mailbox).iter_pieces().count())
    });

    group.bench_function("index_read", |b| {
        b.iter(|| std::hint::black_box(mailbox)[std::hint::black_box(Square::E4)])
    });

    group.bench_function("index_write", |b| {
        let mut m = mailbox;
        b.iter(|| {
            m[std::hint::black_box(Square::E4)] = std::hint::black_box(Piece::WHITE_PAWN);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_square_construction,
    bench_square_accessors,
    bench_file_rank_ops,
    bench_mailbox,
);
criterion_main!(benches);
