use aoc2025::{day1::{day1_part1, day1_part1_lowlevel}, input};
use divan::{Bencher, black_box};

fn main() {
    divan::main();
}

#[divan::bench]
fn day1_part1_idiomatic_bench(bencher: Bencher) {
    let input = input("resources/day01_input.txt");

    bencher.bench_local(move || {
        day1_part1(black_box(&input));
    });
}

#[divan::bench]
fn day1_part1_lowlevel_bench(bencher: Bencher) {
    let input = input("resources/day01_input.txt");

    bencher.bench_local(move || {
        day1_part1_lowlevel(black_box(&input));
    });
}
