#![feature(test)]

use aoc::Solver;
use y{{year}}_day{{day}}_{{title_snake}}::Solution;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

criterion_group!(benches, bench_solution);
criterion_main!(benches);

fn bench_solution(c: &mut Criterion) {
    let aoc = aoc::Aoc::new(&2020, &1).unwrap();

    let mut group = c.benchmark_group("{{title}}");

    let input = Solution::generate_input(&aoc.input).unwrap();

    group.bench_with_input(BenchmarkId::from_parameter("First Solution"), &input, |b, i| {
        b.iter(|| { Solution::solve_first(&i) }.unwrap())
    });

    group.finish();
}
