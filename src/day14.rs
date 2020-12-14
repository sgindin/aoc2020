use crate::tools;
use regex::{Regex, Captures};
use std::collections::{HashMap};
use std::fmt::{Debug, Formatter, Result};

lazy_static! {
    static ref LHS_RE: Regex = Regex::new(r"^(mask|mem)(?:\[([0-9]+)\])?$").unwrap();
}

struct Mask {
    ones: u64,
    zeros: u64,
}

impl Debug for Mask {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("{:#038b}\n{:#038b}", self.ones, self.zeros))
    }
}

impl Mask {
    fn new() -> Mask {
        Mask {
            ones: 0,
            zeros: 0xFFFFFFFFF,
        }
    }

    fn new_from_str(mask: &str) -> Mask {
        mask.chars().enumerate().fold(Mask::new(), |mask, (i, c)| {
            match c {
                '1' => mask.set_one(35 - i),
                '0' => mask.set_zero(35 - i),
                'X' => mask,
                _ => unreachable!(),
            }
        })
    }


    fn set_one(mut self, bit: usize) -> Mask {
        self.ones |= 1 << bit;
        self
    }

    fn set_zero(mut self, bit: usize) -> Mask {
        self.zeros &= !(1 << bit);
        self
    }

    fn apply_to(&self, value: u64) -> u64 {
        (value | self.ones) & self.zeros
    }
}

pub fn solve() {
    let (_, memory) = tools::read_lines("./input/day14.txt")
        .unwrap()
        .fold((Mask::new(), HashMap::new()), |(mask, mut memory), line| {
            let line = line.unwrap();
            let mut tokens = line.split(" = ");
            let lhs = tokens.next().unwrap();
            let rhs = tokens.next().unwrap();
            let captures: Captures = LHS_RE.captures(lhs).unwrap();
            let command = captures.get(1).unwrap().as_str();
            match command {
                "mask" => (Mask::new_from_str(rhs), memory),
                "mem" => {
                    let address = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
                    let value = rhs.parse::<u64>().unwrap();
                    memory.insert(address, mask.apply_to(value));
                    (mask, memory)
                },
                _ => unreachable!(),
            }
        });

    println!("{}", memory.values().sum::<u64>())
}