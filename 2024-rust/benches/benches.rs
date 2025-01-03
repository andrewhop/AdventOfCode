use aoc2024::day1::{
    day1_part1_heap, day1_part1_multi_pass_fold, day1_part1_multi_pass_loop, day1_part1_radix,
    day1_part1_radix_one_pass, day1_part1_vec, day1_part2,
};
use aoc2024::day2::{day2_part1_clean, day2_part1_gross};
use aoc2024::day22::{
    day22_part1, day22_part2, day22_part2_array, day22_part2_arrayish, day22_part2_std, step,
    step_shift,
};
use aoc2024::day3::{day3_part1_lazy_regex, day3_part1_regex, day3_part2_regex};
use aoc2024::day4::{day4_part1, day4_part2};
use aoc2024::day5::day5_part1;
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

    // M1 time:   [50.909 µs 51.296 µs 51.706 µs]
    // G4 time:
    c.bench_function("day2_part1_clean", |b| {
        b.iter(|| day2_part1_clean(black_box(&input)))
    });
}

fn day3_bench(c: &mut Criterion) {
    let input = input("resources/day3_input.txt");

    // M1 time:   [289.80 µs 292.89 µs 296.03 µs]
    // G4 time:   [362.81 µs 363.83 µs 364.87 µs]
    c.bench_function("day3_part1_regex", |b| {
        b.iter(|| day3_part1_regex(black_box(&input)))
    });

    // M1 time:   [145.63 µs 146.65 µs 147.72 µs]
    // G4 time:   [180.92 µs 180.93 µs 180.95 µs]
    c.bench_function("day3_part1_lazy_regex", |b| {
        b.iter(|| day3_part1_lazy_regex(black_box(&input)))
    });

    // M1 time:   [190.88 µs 191.48 µs 192.12 µs]
    // G4 time:
    c.bench_function("day3_part2_regex", |b| {
        b.iter(|| day3_part2_regex(black_box(&input)))
    });
}

fn day4_bench(c: &mut Criterion) {
    let input = input("resources/day4_input.txt");

    // M1 time:   [203.37 µs 206.28 µs 208.97 µs]
    // G4 time:
    c.bench_function("day4_part1", |b| b.iter(|| day4_part1(black_box(&input))));

    // M1 time:   [300.02 µs 301.25 µs 302.71 µs]
    // G4 time:
    c.bench_function("day4_part2", |b| b.iter(|| day4_part2(black_box(&input))));
}

fn day5_bench(c: &mut Criterion) {
    let input = input("resources/day5_input.txt");

    // M1 time:   [42.557 µs 42.628 µs 42.707 µs]
    // G4 time:
    c.bench_function("day5_part1", |b| b.iter(|| day5_part1(black_box(&input))));
}

fn day22_bench(c: &mut Criterion) {
    let input = input("resources/day22_input.txt");

    // M1 time:   [7.1304 ms 7.1549 ms 7.1811 ms]
    // G4 time:
    c.bench_function("day22_part1", |b| b.iter(|| day22_part1(black_box(&input))));

    // M1 time:   [532.01 ms 536.87 ms 542.09 ms]
    c.bench_function("day22_part2", |b| b.iter(|| day22_part2(black_box(&input))));

    // M1 time:   [565.83 ms 577.63 ms 590.10 ms]
    c.bench_function("day22_part2_std", |b| {
        b.iter(|| day22_part2_std(black_box(&input)))
    });

    // M1 time:   [123.18 ms 123.80 ms 124.45 ms]
    c.bench_function("day22_part2_arrayish", |b| {
        b.iter(|| day22_part2_arrayish(black_box(&input)))
    });

    // M1 time:   [15.171 ms 15.206 ms 15.243 ms]
    c.bench_function("day22_part2_array", |b| {
        b.iter(|| day22_part2_array(black_box(&input)))
    });

    // [471.21 ps 472.68 ps 474.49 ps]
    c.bench_function("day22_step", |b| b.iter(|| step(black_box(1234))));

    // [517.20 ps 518.33 ps 519.58 ps]
    c.bench_function("day22_step_shift", |b| {
        b.iter(|| step_shift(black_box(1234)))
    });
}

criterion_group!(
    benches,
    day1_bench,
    day2_bench,
    day3_bench,
    day4_bench,
    day5_bench,
    day22_bench,
);
criterion_main!(benches);
