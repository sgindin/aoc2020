use crate::tools;
use regex::{Regex, Captures};
use std::collections::{HashMap};

lazy_static! {
    static ref LHS_RE: Regex = Regex::new(r"^(mask|mem)(?:\[([0-9]+)\])?$").unwrap();
}

const SIGNIFICANT_BITS: u64 = 0xFFFFFFFFF;

struct Mask {
    ones: u64,
    zeros: u64,
    xs: u64,
    x_permutations: Vec<u64>,
}

impl Mask {
    fn new() -> Self {
        Mask {
            ones: 0,
            zeros: 0,
            xs: 0,
            x_permutations: vec![0],
        }
    }

    fn new_from_str(mask: &str) -> Self {
        mask.chars().rev().enumerate().fold(Mask::new(), |mask, (bit, c)| {
            match c {
                '1' => mask.set_one(bit),
                '0' => mask.set_zero(bit),
                'X' => mask.set_x(bit),
                _ => unreachable!(),
            }
        })
    }

    fn set_one(mut self, bit: usize) -> Mask {
        self.ones |= 1 << bit;
        self
    }

    fn set_zero(mut self, bit: usize) -> Mask {
        self.zeros |= 1 << bit;
        self
    }

    fn set_x(mut self, bit: usize) -> Mask {
        let x = 1 << bit;
        self.xs |= x;
        let mut new_permutations = vec![0u64; self.x_permutations.len() * 2];
        for (i, s) in self.x_permutations.iter().enumerate() {
            new_permutations[2*i] = s | x;
            new_permutations[2*i + 1] = *s;
        }
        std::mem::swap(&mut self.x_permutations, &mut new_permutations);
        self
    }

    fn apply(&self, value: u64) -> u64 {
        (value | self.ones) & (!self.zeros & SIGNIFICANT_BITS)
    }

    fn apply_x_permutation(&self, value: u64, i: usize) -> u64 {
        ((value | self.ones) & (!self.xs & SIGNIFICANT_BITS)) | self.x_permutations[i]
    }
}

pub fn solve() {
    let (_, memory_1, memory_2) = tools::read_lines("./input/day14.txt")
        .unwrap()
        .fold((Mask::new(), HashMap::new(), HashMap::new()),
              |(mask, mut memory_1, mut memory_2), line| {
                let line = line.unwrap();
                let mut tokens = line.split(" = ");
                let lhs = tokens.next().unwrap();
                let rhs = tokens.next().unwrap();
                let captures: Captures = LHS_RE.captures(lhs).unwrap();
                let command = captures.get(1).unwrap().as_str();
                match command {
                    "mask" => (Mask::new_from_str(rhs), memory_1, memory_2),
                    "mem" => {
                        let address = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
                        let value = rhs.parse::<u64>().unwrap();
                        memory_1.insert(address, mask.apply(value));
                        for i in 0..mask.x_permutations.len() {
                            memory_2.insert(mask.apply_x_permutation(address, i), value);
                        }
                        (mask, memory_1, memory_2)
                    },
                    _ => unreachable!(),
                }
            });

    println!("{}, {}", memory_1.values().sum::<u64>(), memory_2.values().sum::<u64>())
}