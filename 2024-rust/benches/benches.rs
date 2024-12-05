use aoc2024::day1::{day1_part1_vec, day1_part1_heap, day1_part2, day1_part1_multi_pass_fold, day1_part1_multi_pass_loop};
use aoc2024::input;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day1_bench(c: &mut Criterion) {
    let input = input("resources/day1_input.txt");
    // M1 Mac time:   [50.922 µs 51.235 µs 51.598 µs]
    // G4     time:   [52.104 µs 52.116 µs 52.127 µs]
    c.bench_function("day1_part1_vec", |b| b.iter(|| day1_part1_vec(black_box(&input))));

    // M1 Mac time:   [666.92 µs 669.47 µs 673.81 µs]
    // G4     time:   [859.89 µs 859.95 µs 860.02 µs]
    c.bench_function("day1_part1_multi_pass_fold", |b| b.iter(|| day1_part1_multi_pass_fold(black_box(&input))));

    c.bench_function("day1_part1_multi_pass_loop", |b| b.iter(|| day1_part1_multi_pass_loop(black_box(&input))));


    // M1 Mac time:   [83.837 µs 84.570 µs 85.337 µs]
    // G4     time:   [74.908 µs 74.932 µs 74.961 µs]
    c.bench_function("day1_part1_heap", |b| {
        b.iter(|| day1_part1_heap(black_box(&input)))
    });

    // M1 Mac time:   [79.223 µs 79.690 µs 80.180 µs]
    // G4     time:   [78.985 µs 79.012 µs 79.041 µs]
    c.bench_function("day1_part2", |b| b.iter(|| day1_part2(black_box(&input))));
}

criterion_group!(benches, day1_bench);
criterion_main!(benches);
