use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use gambit_models::piece::Piece;
use gambit_models::piece::colour::Colour;
use gambit_models::piece::map::{ColourMap, PieceTypeMap};
use gambit_models::piece::piece_type::PieceType;

fn bench_piece_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("piece_construction");

    group.bench_function("new", |b| {
        b.iter(|| {
            Piece::new(
                std::hint::black_box(PieceType::Queen),
                std::hint::black_box(Colour::White),
            )
        })
    });

    group.bench_function("from_bits", |b| {
        b.iter(|| Piece::from_bits(std::hint::black_box(4u8)))
    });

    group.bench_function("try_from_u8_valid", |b| {
        b.iter(|| Piece::try_from(std::hint::black_box(4u8)))
    });

    group.bench_function("try_from_u8_invalid", |b| {
        b.iter(|| Piece::try_from(std::hint::black_box(6u8)))
    });

    group.bench_function("try_from_char", |b| {
        b.iter(|| Piece::try_from(std::hint::black_box('Q')))
    });

    group.bench_function("all_12_named_pieces_via_new", |b| {
        b.iter(|| {
            let mut sum = 0u8;
            for pt in PieceType::ALL {
                for colour in Colour::ALL {
                    sum = sum.wrapping_add(
                        Piece::new(std::hint::black_box(pt), std::hint::black_box(colour)).bits(),
                    );
                }
            }
            sum
        })
    });

    group.finish();
}

fn bench_piece_decoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("piece_decoding");

    let piece = Piece::WHITE_QUEEN;

    group.bench_function("piece_type", |b| {
        b.iter(|| std::hint::black_box(piece).piece_type())
    });

    group.bench_function("colour", |b| {
        b.iter(|| std::hint::black_box(piece).colour())
    });

    group.bench_function("is_none", |b| {
        b.iter(|| std::hint::black_box(piece).is_none())
    });

    group.bench_function("is_sliding", |b| {
        b.iter(|| std::hint::black_box(piece).is_sliding())
    });

    group.bench_function("as_char", |b| {
        b.iter(|| std::hint::black_box(piece).as_char())
    });

    group.bench_function("flip_colour", |b| {
        b.iter(|| std::hint::black_box(piece).flip_colour())
    });

    group.bench_function("with_colour", |b| {
        b.iter(|| std::hint::black_box(piece).with_colour(std::hint::black_box(Colour::Black)))
    });

    group.finish();
}

fn bench_colour_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("colour");

    group.bench_function("other", |b| {
        b.iter(|| std::hint::black_box(Colour::White).other())
    });

    group.bench_function("not", |b| b.iter(|| !std::hint::black_box(Colour::Black)));

    group.bench_function("from_char_upper", |b| {
        b.iter(|| Colour::from(std::hint::black_box('Q')))
    });

    group.bench_function("from_char_lower", |b| {
        b.iter(|| Colour::from(std::hint::black_box('q')))
    });

    group.bench_function("try_from_u8", |b| {
        b.iter(|| Colour::try_from(std::hint::black_box(0u8)))
    });

    group.finish();
}

fn bench_map_indexing(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_indexing");

    let colour_map = ColourMap::from([100i32, 200i32]);
    let piece_map = PieceTypeMap::from([100i32, 300, 330, 500, 900, 20000]);

    group.bench_function("colour_map_index_white", |b| {
        b.iter(|| std::hint::black_box(colour_map)[std::hint::black_box(Colour::White)])
    });

    group.bench_function("colour_map_index_black", |b| {
        b.iter(|| std::hint::black_box(colour_map)[std::hint::black_box(Colour::Black)])
    });

    group.bench_function("piece_type_map_index_queen", |b| {
        b.iter(|| std::hint::black_box(piece_map)[std::hint::black_box(PieceType::Queen)])
    });

    group.bench_function("piece_type_map_iter_sum", |b| {
        b.iter(|| {
            std::hint::black_box(piece_map)
                .iter()
                .map(|(_, &v)| v)
                .sum::<i32>()
        })
    });

    group.bench_function("colour_map_from_iterator", |b| {
        b.iter(|| {
            Colour::ALL
                .iter()
                .map(|&c| (c, std::hint::black_box(c as i32 * 100)))
                .collect::<ColourMap<i32>>()
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_piece_construction,
    bench_piece_decoding,
    bench_colour_operations,
    bench_map_indexing,
);

criterion_main!(benches);
