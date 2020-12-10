mod tools;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

extern crate regex;

#[macro_use]
extern crate lazy_static;

macro_rules! format_title { () => { "\n=====================\nDay {} (part1, part2)\n=====================" }; }

fn main() {
    let puzzles = [
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
        day6::solve,
        day7::solve,
        day8::solve,
        day9::solve,
        day10::solve,
    ];

    puzzles.iter().enumerate().for_each(|(i, solve)| {
        println!(format_title!(), i + 1);
        solve();
    });
}
