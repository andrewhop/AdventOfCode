use aoc2024::day1::{day1_part1_vec, day1_part1_heap, day1_part2, day1_part1_multi_pass_fold, day1_part1_multi_pass_loop, day1_part1_radix};
use aoc2024::input;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day1_bench(c: &mut Criterion) {
    let input = input("resources/day1_input.txt");
    // M1 Mac time:   [46.661 µs 46.804 µs 46.990 µs]
    // G4     time:   [52.104 µs 52.116 µs 52.127 µs]
    c.bench_function("day1_part1_vec", |b| b.iter(|| day1_part1_vec(black_box(&input))));

    // M1 Mac time:   [120.38 µs 120.63 µs 120.92 µs]
    // G4
    c.bench_function("day1_part1_radix", |b| b.iter(|| day1_part1_radix(black_box(&input))));

    // M1 Mac time:   [665.17 µs 666.50 µs 668.08 µs]
    // G4     time:   [858.26 µs 858.28 µs 858.31 µs]
    c.bench_function("day1_part1_multi_pass_fold", |b| b.iter(|| day1_part1_multi_pass_fold(black_box(&input))));

    // M1 Mac time:   [673.10 µs 674.39 µs 675.89 µs]
    // G4     time:   [843.05 µs 843.14 µs 843.23 µs]
    c.bench_function("day1_part1_multi_pass_loop", |b| b.iter(|| day1_part1_multi_pass_loop(black_box(&input))));


    // M1 Mac time:   [82.222 µs 82.625 µs 83.029 µs]
    // G4     time:   [74.908 µs 74.932 µs 74.961 µs]
    c.bench_function("day1_part1_heap", |b| {
        b.iter(|| day1_part1_heap(black_box(&input)))
    });

    // M1 Mac time:   [68.220 µs 68.445 µs 68.706 µs]
    // G4     time:   [78.985 µs 79.012 µs 79.041 µs]
    c.bench_function("day1_part2", |b| b.iter(|| day1_part2(black_box(&input))));
}

criterion_group!(benches, day1_bench);
criterion_main!(benches);
