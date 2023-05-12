#![allow(dead_code)]
use std::fmt::Display;

use criterion::{criterion_group, criterion_main, Criterion};

use day01;
use day02;
use day03;
use day04;
use day05;
use day06;
use day07;
use day08;
use day09;
use day10;
use day11;
use day12;
use day13;
use day14;
use day15;
// use day16;
// use day17;
// use day18;
// use day19;
use day20;
use day21;
// use day22;
// use day23;
// use day24;
// use day25;

#[path = "../src/main.rs"]
mod main;

fn bench_parts<F1, F2, O1, O2>(c: &mut Criterion, title: &str, part1: F1, part2: F2)
where F1: Fn() -> O1, F2: Fn() -> O2, O1 : Display, O2: Display {
    c.bench_function(format!("{} - Part 1", title).as_str(), |b| {
        b.iter(|| part1())
    });

    c.bench_function(format!("{} - Part 2", title).as_str(), |b| {
        b.iter(|| part2())
    });
}

fn bench_main(c: &mut Criterion) {
    bench_parts(c, "Day 01", day01::part1, day01::part2);
    bench_parts(c, "Day 02", day02::part1, day02::part2);
    bench_parts(c, "Day 03", day03::part1, day03::part2);
    bench_parts(c, "Day 04", day04::part1, day04::part2);
    bench_parts(c, "Day 05", day05::part1, day05::part2);
    bench_parts(c, "Day 06", day06::part1, day06::part2);
    bench_parts(c, "Day 07", day07::part1, day07::part2);
    bench_parts(c, "Day 08", day08::part1, day08::part2);
    bench_parts(c, "Day 09", day09::part1, day09::part2);
    bench_parts(c, "Day 10", day10::part1, day10::part2);
    bench_parts(c, "Day 11", day11::part1, day11::part2);
    bench_parts(c, "Day 12", day12::part1, day12::part2);
    bench_parts(c, "Day 13", day13::part1, day13::part2);
    bench_parts(c, "Day 14", day14::part1, day14::part2);
    bench_parts(c, "Day 15", day15::part1, day15::part2);
    // bench_parts(c, "Day 16", day16::part1, day16::part2);
    // bench_parts(c, "Day 17", day17::part1, day17::part2);
    // bench_parts(c, "Day 18", day18::part1, day18::part2);
    // bench_parts(c, "Day 19", day19::part1, day19::part2);
    bench_parts(c, "Day 20", day20::part1, day20::part2);
    bench_parts(c, "Day 21", day21::part1, day21::part2);
    // bench_parts(c, "Day 22", day22::part1, day22::part2);
    // bench_parts(c, "Day 23", day23::part1, day23::part2);
    // bench_parts(c, "Day 24", day24::part1, day24::part2);
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
