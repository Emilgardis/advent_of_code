#![feature(test)]

use aoc::Solver;
use y2020_day1_report_repair::Solution;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

criterion_group!(benches, bench_solution);
criterion_main!(benches);

fn bench_solution(c: &mut Criterion) {
    let aoc = aoc::Aoc::new(&2020, &1).unwrap();

    let mut group = c.benchmark_group("Report Repair");

    let input = Solution::generate_input(&aoc.input).unwrap();

    group.bench_with_input(BenchmarkId::from_parameter("Solution"), &input, |b, &i| {
        b.iter(|| { Solution::solve(i) }.unwrap()),
    });

    group.finish();
}
