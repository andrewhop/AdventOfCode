use aoc2024::day1::{day1_part1_vec, day1_part1_heap, day1_part2, day1_part1_multi_pass};
use aoc2024::input;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day1_bench(c: &mut Criterion) {
    let input = input("resources/day1_input.txt");
    // M1 Mac time:   [50.922 µs 51.235 µs 51.598 µs]
    c.bench_function("day1_part1_vec", |b| b.iter(|| day1_part1_vec(black_box(&input))));

    // M1 Mac time:   [679.25 µs 681.34 µs 683.46 µs]
    c.bench_function("day1_part1_multi_pass", |b| b.iter(|| day1_part1_multi_pass(black_box(&input))));

    // M1 Mac time:   [83.837 µs 84.570 µs 85.337 µs]
    c.bench_function("day1_part1_heap", |b| {
        b.iter(|| day1_part1_heap(black_box(&input)))
    });

    // M1 Mac time:   [79.223 µs 79.690 µs 80.180 µs]
    c.bench_function("day1_part2", |b| b.iter(|| day1_part2(black_box(&input))));
}

criterion_group!(benches, day1_bench);
criterion_main!(benches);
