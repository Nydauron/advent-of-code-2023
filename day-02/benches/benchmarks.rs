use criterion::{criterion_group, criterion_main, Criterion};
use day_02::*;

fn criterion_benchmark_part1(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");

    let mut group = c.benchmark_group("day_02::part1");
    group.bench_with_input("part1", input, |b, input| b.iter(|| part1::part1(input)));

    group.finish();
}

fn criterion_benchmark_part1_chumsky(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");

    let mut group = c.benchmark_group("day_02::part1_chumsky");
    group.bench_with_input("part1_chumsky", input, |b, input| {
        b.iter(|| part1_chumsky::part1(input))
    });

    group.finish();
}

fn criterion_benchmark_part2(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");

    let mut group = c.benchmark_group("day_02::part2");
    group.bench_with_input("part2", input, |b, input| b.iter(|| part2::part2(input)));

    group.finish();
}

fn criterion_benchmark_part2_chumsky(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");

    let mut group = c.benchmark_group("day_02::part2_chumsky");
    group.bench_with_input("part2_chumsky", input, |b, input| {
        b.iter(|| part2_chumsky::part2(input))
    });

    group.finish();
}
criterion_group!(
    benches,
    criterion_benchmark_part1,
    criterion_benchmark_part2,
    criterion_benchmark_part1_chumsky,
    criterion_benchmark_part2_chumsky,
);
criterion_main!(benches);
