use aoc2023::day8::day8_part1_low_level;
use aoc2023::input;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const A: u8 = b'A';
const Z: u8 = b'Z';

// time:   [865.55 ps 870.75 ps 876.74 ps]
fn word_to_num(input: &[u8]) -> u16 {
    (((input[0] - A) as u16) << 10) + (((input[1] - A) as u16) << 5) + ((input[2] - A) as u16)
}

// time:   [906.72 ps 909.95 ps 913.40 ps]
fn word_to_num2(input: &[u8]) -> u16 {
    (((input[0] - A) as u16) * 26 * 26) + (((input[1] - A) as u16) * 26) + ((input[2] - A) as u16)
}

// time:   [2.4659 ns 2.6554 ns 2.8429 ns]
fn word_to_num3(first: u8, second: u8, third: u8) -> u16 {
    (((first - A) as u16) * 26 * 26) + (((second - A) as u16) * 26) + ((third - A) as u16)
}

fn day8_utils(c: &mut Criterion) {
    let a = [A, A, A, Z, Z, Z];
    c.bench_function("word_to_num", |b| {
        b.iter(|| word_to_num(black_box(&a[2..5])))
    });
    c.bench_function("word_to_num2", |b| {
        b.iter(|| word_to_num2(black_box(&a[2..5])))
    });
    c.bench_function("word_to_num3", |b| {
        b.iter(|| word_to_num3(black_box(a[2]), black_box(a[3]), black_box(a[4])))
    });
}

// time:   [31.684 µs 31.721 µs 31.772 µs]
fn day8_part1(c: &mut Criterion) {
    let input = input("resources/day8_input.txt");
    c.bench_function("day8_part1_low_level", |b| {
        b.iter(|| day8_part1_low_level(black_box(&input)))
    });
}

criterion_group!(benches, day8_part1, day8_utils);
criterion_main!(benches);
