mod p1;
mod p2;
mod tools;
mod p3;
mod p4;
mod p5;
mod p6;

extern crate regex;

#[macro_use]
extern crate lazy_static;

macro_rules! format_title { () => { "\n=====================\n{}\n=====================" }; }

fn main() {
    println!(format_title!(), "p1");
    p1::solve();

    println!(format_title!(), "p2");
    p2::solve();

    println!(format_title!(), "p3");
    p3::solve();

    println!(format_title!(), "p4");
    p4::solve();

    println!(format_title!(), "p5");
    p5::solve();

    println!(format_title!(), "p6");
    p6::solve();
}
