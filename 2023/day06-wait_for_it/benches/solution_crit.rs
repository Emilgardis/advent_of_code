use aoc::parts::*;
use aoc::Solver;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use y2023_day6_wait_for_it::Solution;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = aoc::Aoc::new(&2023u32, &6u32).unwrap().input;
    c.bench_with_input(
        criterion::BenchmarkId::new("solve 2023 day 6 part1", "using given input"),
        &input,
        |b, input| {
            b.iter(|| {
                let inp =
                    <Solution as Solver<Year2023, Day6, Part1>>::generate_input(black_box(input))
                        .unwrap();
                let sol =
                    <Solution as Solver<Year2023, Day6, Part1>>::solve(black_box(&inp)).unwrap();
                black_box(sol)
            })
        },
    );

    c.bench_with_input(
        criterion::BenchmarkId::new("solve 2023 day 6 part2", "using given input"),
        &input,
        |b, input| {
            b.iter(|| {
                let inp =
                    <Solution as Solver<Year2023, Day6, Part2>>::generate_input(black_box(input))
                        .unwrap();
                let sol =
                    <Solution as Solver<Year2023, Day6, Part2>>::solve(black_box(&inp)).unwrap();
                black_box(sol)
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
