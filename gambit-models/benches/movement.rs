use criterion::{Criterion, criterion_group, criterion_main};
use gambit_models::movement::castling::rights::CastlingRights;
use gambit_models::movement::castling::side::CastlingSide;
use gambit_models::movement::direction::Direction;
use gambit_models::piece::colour::Colour;
use std::str::FromStr;

fn bench_direction_hot_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("direction_hot_path");

    group.bench_function("offset_all", |b| {
        b.iter(|| {
            Direction::ALL
                .iter()
                .map(|&d| std::hint::black_box(d).offset())
                .sum::<i8>()
        })
    });

    group.bench_function("flip_all", |b| {
        b.iter(|| Direction::ALL.map(|d| std::hint::black_box(d).flip()))
    });

    group.bench_function("is_orthogonal_all", |b| {
        b.iter(|| {
            Direction::ALL
                .iter()
                .filter(|&&d| std::hint::black_box(d).is_orthogonal())
                .count()
        })
    });

    #[allow(clippy::double_ended_iterator_last)]
    group.bench_function("from_index_all", |b| {
        b.iter(|| {
            (0u8..8)
                .map(|i| Direction::from_index(std::hint::black_box(i)))
                .last()
        })
    });

    #[allow(clippy::double_ended_iterator_last)]
    group.bench_function("try_from_i8_valid", |b| {
        let offsets: [i8; 8] = [8, 9, 1, -7, -8, -9, -1, 7];
        b.iter(|| {
            offsets
                .iter()
                .map(|&v| Direction::try_from(std::hint::black_box(v)))
                .last()
        })
    });

    group.finish();
}

fn bench_castling_rights(c: &mut Criterion) {
    let mut group = c.benchmark_group("castling_rights");

    group.bench_function("has_all_combinations", |b| {
        let rights = std::hint::black_box(CastlingRights::ALL);
        b.iter(|| {
            Colour::ALL
                .iter()
                .flat_map(|&colour| {
                    CastlingSide::ALL
                        .iter()
                        .map(move |&side| rights.has(colour, side))
                })
                .all(|v| v)
        })
    });

    group.bench_function("remove_colour", |b| {
        b.iter(|| {
            std::hint::black_box(CastlingRights::ALL)
                .remove_colour(std::hint::black_box(Colour::White))
        })
    });

    group.bench_function("remove_right", |b| {
        b.iter(|| {
            std::hint::black_box(CastlingRights::ALL).remove_right(
                std::hint::black_box(CastlingSide::Kingside),
                std::hint::black_box(Colour::White),
            )
        })
    });

    group.bench_function("from_str_full", |b| {
        b.iter(|| CastlingRights::from_str(std::hint::black_box("KQkq")))
    });

    group.bench_function("from_str_none", |b| {
        b.iter(|| CastlingRights::from_str(std::hint::black_box("-")))
    });

    group.bench_function("display_to_string", |b| {
        let rights = std::hint::black_box(CastlingRights::ALL);
        b.iter(|| rights.to_string())
    });

    group.finish();
}

criterion_group!(benches, bench_direction_hot_path, bench_castling_rights);
criterion_main!(benches);
