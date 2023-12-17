use criterion::{criterion_group, criterion_main, Criterion};
use day_17::*;

fn criterion_benchmark_part1(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");

    let mut group = c.benchmark_group("day_17::part1");
    group.bench_with_input("part1", input, |b, input| b.iter(|| part1::part1(input)));

    group.finish();
}

fn criterion_benchmark_part1_astar(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");

    let mut group = c.benchmark_group("day_17::part1_astar");
    group.bench_with_input("part1", input, |b, input| {
        b.iter(|| part1_astar::part1(input))
    });

    group.finish();
}

fn criterion_benchmark_part1_astar_with_skips(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");

    let mut group = c.benchmark_group("day_17::part1_astar_with_skips");
    group.bench_with_input("part1", input, |b, input| {
        b.iter(|| part1_astar_with_skips::part1(input))
    });

    group.finish();
}

fn criterion_benchmark_part1_vector_astar_with_skips(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");

    let mut group = c.benchmark_group("day_17::part1_vector_astar_with_skips");
    group.bench_with_input("part1", input, |b, input| {
        b.iter(|| part1_vector_astar_with_skips::part1(input))
    });

    group.finish();
}

fn criterion_benchmark_part2(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");

    let mut group = c.benchmark_group("day_17::part2");
    group.bench_with_input("part2", input, |b, input| b.iter(|| part2::part2(input)));

    group.finish();
}

fn criterion_benchmark_part2_astar_with_skips(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");

    let mut group = c.benchmark_group("day_17::part2_astar_with_skips");
    group.bench_with_input("part2", input, |b, input| {
        b.iter(|| part2_astar_with_skips::part2(input))
    });

    group.finish();
}

fn criterion_benchmark_part2_vector_astar_with_skips(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");

    let mut group = c.benchmark_group("day_17::part2_vector_astar_with_skips");
    group.bench_with_input("part2", input, |b, input| {
        b.iter(|| part2_vector_astar_with_skips::part2(input))
    });

    group.finish();
}

criterion_group!(
    benches,
    criterion_benchmark_part1,
    criterion_benchmark_part2,
    criterion_benchmark_part1_astar,
    criterion_benchmark_part1_astar_with_skips,
    criterion_benchmark_part2_astar_with_skips,
    criterion_benchmark_part1_vector_astar_with_skips,
    criterion_benchmark_part2_vector_astar_with_skips
);
criterion_main!(benches);
