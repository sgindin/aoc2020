mod day1;
mod day2;
mod tools;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

extern crate regex;

#[macro_use]
extern crate lazy_static;

macro_rules! format_title { () => { "\n=====================\n{}\n=====================" }; }

fn main() {
    println!(format_title!(), "p1");
    day1::solve();

    println!(format_title!(), "p2");
    day2::solve();

    println!(format_title!(), "p3");
    day3::solve();

    println!(format_title!(), "p4");
    day4::solve();

    println!(format_title!(), "p5");
    day5::solve();

    println!(format_title!(), "p6");
    day6::solve();

    println!(format_title!(), "p7");
    day7::solve();

    println!(format_title!(), "p8");
    day8::solve();

    println!(format_title!(), "p9");
    day9::solve();
}
