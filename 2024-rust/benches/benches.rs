use aoc2024::day1::{
    day1_part1_heap, day1_part1_multi_pass_fold, day1_part1_multi_pass_loop, day1_part1_radix,
    day1_part1_radix_one_pass, day1_part1_vec, day1_part2,
};
use aoc2024::day2::{day2_part1_clean, day2_part1_gross};
use aoc2024::input;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day1_bench(c: &mut Criterion) {
    let input = input("resources/day1_input.txt");
    // M1 time:   [46.661 µs 46.804 µs 46.990 µs]
    // G4 time:   [49.900 µs 49.913 µs 49.925 µs]
    c.bench_function("day1_part1_vec", |b| {
        b.iter(|| day1_part1_vec(black_box(&input)))
    });

    // M1 time:   [120.38 µs 120.63 µs 120.92 µs]
    // G4 time:   [124.60 µs 124.63 µs 124.66 µs]
    c.bench_function("day1_part1_radix", |b| {
        b.iter(|| day1_part1_radix(black_box(&input)))
    });

    // M1 time:   [120.38 µs 120.63 µs 120.93 µs]
    // G4 time:   [121.73 µs 121.74 µs 121.76 µs]
    c.bench_function("day1_part1_radix_one_pass", |b| {
        b.iter(|| day1_part1_radix_one_pass(black_box(&input)))
    });

    // M1 time:   [665.17 µs 666.50 µs 668.08 µs]
    // G4 time:   [857.30 µs 857.32 µs 857.34 µs]
    c.bench_function("day1_part1_multi_pass_fold", |b| {
        b.iter(|| day1_part1_multi_pass_fold(black_box(&input)))
    });

    // M1 time:   [673.10 µs 674.39 µs 675.89 µs]
    // G4 time:   [850.81 µs 850.83 µs 850.86 µs]
    c.bench_function("day1_part1_multi_pass_loop", |b| {
        b.iter(|| day1_part1_multi_pass_loop(black_box(&input)))
    });

    // M1 time:   [82.222 µs 82.625 µs 83.029 µs]
    // G4 time:   [71.418 µs 71.444 µs 71.475 µs]
    c.bench_function("day1_part1_heap", |b| {
        b.iter(|| day1_part1_heap(black_box(&input)))
    });

    // M1 time:   [68.220 µs 68.445 µs 68.706 µs]
    // G4 time:   [70.788 µs 70.809 µs 70.830 µs]
    c.bench_function("day1_part2", |b| b.iter(|| day1_part2(black_box(&input))));
}

fn day2_bench(c: &mut Criterion) {
    let input = input("resources/day2_input.txt");

    // M1 time:   [42.557 µs 42.628 µs 42.707 µs]
    // G4 time:
    c.bench_function("day2_part1_gross", |b| {
        b.iter(|| day2_part1_gross(black_box(&input)))
    });

    c.bench_function("day2_part1_clean", |b| {
        b.iter(|| day2_part1_clean(black_box(&input)))
    });
}

criterion_group!(benches, day1_bench, day2_bench);
criterion_main!(benches);
