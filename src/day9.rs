use crate::tools;
use std::collections::VecDeque;

fn is_valid(x: u64, preamble: &VecDeque<u64>) -> bool {
    let mut numbers = preamble.iter().cloned().collect::<Vec<_>>();
    numbers.sort();
    let mut low = 0;
    let mut high = numbers.len() - 1;
    while low < high {
        let sum = numbers[low] + numbers[high];
        if sum > x {
            high -= 1;
        } else if sum < x {
            low += 1;
        } else {
            return true
        }
    }
    false
}

fn solve_part1(numbers: &Vec<u64>) -> Option<u64> {
    let mut preamble = (0..25).map(|i| numbers[i]).collect::<VecDeque<_>>();
    for i in 25..numbers.len() {
        let x = numbers[i];
        if is_valid(x, &preamble) {
            preamble.pop_front();
            preamble.push_back(x);
        } else {
            return Some(x);
        }
    }
    None
}

fn solve_part2(numbers: &Vec<u64>, target: u64) -> Option<u64> {
    if numbers.len() < 2 {
        return None
    }

    let mut low = 0usize;
    let mut high = 1usize;
    let mut sum = numbers[low] + numbers[high];

    while sum != target {
        if sum < target {
            if high == numbers.len() - 1 {
                return None
            }
            high += 1;
            sum += numbers[high];
        } else { // sum > target
            if low == high - 1 {
                if high == numbers.len() - 1 {
                    return None
                }
                low += 1;
                high += 1;
                sum = numbers[low] + numbers[high];
            } else {
                sum -= numbers[low];
                low += 1;
            }
        }
    }
    let r = &numbers[low..=high];
    Some(r.iter().max().unwrap() + r.iter().min().unwrap())
}

pub fn solve() {
    let numbers = tools::read_lines("./input/day9.txt")
        .unwrap()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part1 = solve_part1(&numbers).unwrap();
    let part2 = solve_part2(&numbers, part1).unwrap();

    println!("{}, {}", part1, part2)
}