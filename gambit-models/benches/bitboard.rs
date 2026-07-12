use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use gambit_models::bitboard::Bitboard;
use gambit_models::location::file::File;
use gambit_models::location::rank::Rank;
use gambit_models::location::square::Square;

fn bench_pop_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_iteration");

    for (label, bits) in [
        ("sparse_8", 0x0000_0000_0000_00FF_u64),
        ("mid_32", 0x0000_0000_FFFF_FFFF_u64),
        ("dense_64", u64::MAX),
    ] {
        group.bench_with_input(BenchmarkId::new("pop", label), &bits, |b, &bits| {
            b.iter(|| {
                let mut bb = std::hint::black_box(Bitboard::new(bits));
                let mut n = 0u32;
                while let Some(sq) = bb.pop() {
                    n = n.wrapping_add(sq.bits() as u32);
                }
                n
            })
        });

        group.bench_with_input(BenchmarkId::new("for_in", label), &bits, |b, &bits| {
            b.iter(|| {
                let mut n = 0u32;
                for sq in std::hint::black_box(Bitboard::new(bits)) {
                    n = n.wrapping_add(sq.bits() as u32);
                }
                n
            })
        });
    }
    group.finish();
}

fn bench_carry_rippler(c: &mut Criterion) {
    let mut group = c.benchmark_group("carry_rippler");

    for n in [4u32, 6, 8, 10, 12] {
        let mask = Bitboard::new((1u64 << n) - 1);
        group.bench_with_input(BenchmarkId::new("subsets", n), &mask, |b, &mask| {
            b.iter(|| {
                let mut count = 0u32;
                for subset in std::hint::black_box(mask).carry_rippler() {
                    count = count.wrapping_add(subset.bits().count_ones());
                }
                count
            })
        });
    }
    group.finish();
}

fn bench_set_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_operations");

    let a = Bitboard::from_rank(Rank::One);
    let b = Bitboard::from_file(File::A);

    group.bench_function("intersection", |bench| {
        bench.iter(|| std::hint::black_box(a).intersection(std::hint::black_box(b)))
    });
    group.bench_function("union", |bench| {
        bench.iter(|| std::hint::black_box(a).union(std::hint::black_box(b)))
    });
    group.bench_function("difference", |bench| {
        bench.iter(|| std::hint::black_box(a).difference(std::hint::black_box(b)))
    });
    group.bench_function("negation", |bench| {
        bench.iter(|| std::hint::black_box(a).negation())
    });
    group.bench_function("symmetric_difference", |bench| {
        bench.iter(|| std::hint::black_box(a).symmetric_difference(std::hint::black_box(b)))
    });

    group.finish();
}

fn bench_contains(c: &mut Criterion) {
    let bb = Bitboard::UNIVERSE;
    c.bench_function("contains", |b| {
        b.iter(|| std::hint::black_box(bb).contains(std::hint::black_box(Square::E4)))
    });
}

fn bench_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("construction");

    group.bench_function("from_square", |b| {
        b.iter(|| Bitboard::from_square(std::hint::black_box(Square::E4)))
    });
    group.bench_function("from_rank", |b| {
        b.iter(|| Bitboard::from_rank(std::hint::black_box(Rank::Four)))
    });
    group.bench_function("from_file", |b| {
        b.iter(|| Bitboard::from_file(std::hint::black_box(File::E)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_pop_iteration,
    bench_carry_rippler,
    bench_set_operations,
    bench_contains,
    bench_construction,
);

criterion_main!(benches);
