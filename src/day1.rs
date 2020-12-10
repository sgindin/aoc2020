use crate::tools;
use std::collections::HashMap;

fn two_sum(numbers: &[i32]) {
    let mut diffs: HashMap<i32, usize> = HashMap::with_capacity(numbers.len());
    for (i, &v) in numbers.iter().enumerate() {
        diffs.insert(2020 - v, i);
    }

    for (i, v) in numbers.iter().enumerate() {
        if let Some(&j) = diffs.get(v) {
            print!("{}", numbers[i]*numbers[j]);
            return;
        }
    }
}

fn three_sum(mut numbers: Vec<i32>) {
    numbers.sort();
    for i in 0..numbers.len() - 2 {
        let mut low = i + 1;
        let mut high = numbers.len() - 1;
        while low < high {
            let sum = numbers[low] + numbers[high] + numbers[i];
            if sum > 2020 {
                high -= 1;
            } else if sum < 2020 {
                low += 1;
            } else {
                println!(", {}", numbers[low]*numbers[i]*numbers[high]);
                high -= 1;
                low += 1;
            }
        }
    }
}

pub(crate) fn solve() {
    let numbers = tools::read_lines("./input/day1.txt")
        .unwrap()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    two_sum(&numbers);
    three_sum(numbers);
}