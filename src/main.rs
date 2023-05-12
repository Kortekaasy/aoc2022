use std::fmt::Display;

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

fn main() {
    run_challenge(" 1.1", day01::part1);
    run_challenge(" 1.2", day01::part2);
    run_challenge(" 2.1", day02::part1);
    run_challenge(" 2.2", day02::part2);
    run_challenge(" 3.1", day03::part1);
    run_challenge(" 3.2", day03::part2);
    run_challenge(" 4.1", day04::part1);
    run_challenge(" 4.2", day04::part2);
    run_challenge(" 5.1", day05::part1);
    run_challenge(" 5.2", day05::part2);
    run_challenge(" 6.1", day06::part1);
    run_challenge(" 6.2", day06::part2);
    run_challenge(" 7.1", day07::part1);
    run_challenge(" 7.2", day07::part2);
    run_challenge(" 8.1", day08::part1);
    run_challenge(" 8.2", day08::part2);
    run_challenge(" 9.1", day09::part1);
    run_challenge(" 9.2", day09::part2);
    run_challenge("10.1", day10::part1);
    run_challenge("10.2", day10::part2);
    run_challenge("11.1", day11::part1);
    run_challenge("11.2", day11::part2);
    run_challenge("12.1", day12::part1);
    run_challenge("12.2", day12::part2);
    run_challenge("13.1", day13::part1);
    run_challenge("13.2", day13::part2);
    run_challenge("14.1", day14::part1);
    run_challenge("14.2", day14::part2);
    run_challenge("15.1", day15::part1);
    run_challenge("15.2", day15::part2);
//     run_challenge("16.1", day16::part1);
//     run_challenge("16.2", day16::part2);
//     run_challenge("17.1", day17::part1);
//     run_challenge("17.2", day17::part2);
//     run_challenge("18.1", day18::part1);
//     run_challenge("18.2", day18::part2);
//    run_challenge("19.1", day19::part1);
//    run_challenge("19.2", day19::part2);
   run_challenge("20.1", day20::part1);
   run_challenge("20.2", day20::part2);
   run_challenge("21.1", day21::part1);
   run_challenge("21.2", day21::part2);
//    run_challenge("22.1", day22::part1);
//    run_challenge("22.2", day22::part2);
//    run_challenge("23.1", day23::part1);
//    run_challenge("23.2", day23::part2);
//    run_challenge("24.1", day24::part1);
//    run_challenge("24.2", day24::part2);
//    run_challenge("25.1", day25::part1);
//    run_challenge("25.2", day25::part2);
}

fn run_challenge<F, O>(title: &str, func: F) 
    where F: Fn() -> O, O : Display {
    println!("================= Challenge {} =================", title);
    let string_output = format!("{}", func());
    for l in string_output.lines() {
        println!("| {:^46} |", l);
    }
    println!("==================================================");
}
