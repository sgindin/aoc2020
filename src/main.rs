#![allow(dead_code)]

mod tools;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;

extern crate regex;

#[macro_use]
extern crate lazy_static;

extern crate itertools;

macro_rules! title_format {
    () => { "\n=====================\nDay {} (part1, part2)\n=====================" };
}

fn main() {
    let puzzles = [
        // day01::solve,
        // day02::solve,
        // day03::solve,
        // day04::solve,
        // day05::solve,
        // day06::solve,
        // day07::solve,
        // day08::solve,
        // day09::solve,
        // day10::solve,
        // day11::solve,
        // day12::solve,
        // day13::solve,
        // day14::solve,
        // day15::solve,
        // day16::solve,
        // day17::solve,
        day18::solve,
    ];

    puzzles.iter().enumerate().for_each(|(i, solve)| {
        println!(title_format!(), i + 1);
        solve();
    });
}
