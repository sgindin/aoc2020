use regex::Regex;
use crate::tools;

pub(crate) fn solve() {
    let re = Regex::new(r"(\d+)-(\d+)\s([a-z]):\s([a-z]+)").unwrap();
    let (count1, count2) = tools::read_lines("./input/day2.txt").unwrap().fold((0, 0), |acc, l| {
        let line = l.unwrap();
        let captures = re.captures(&line).unwrap();
        let low = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let high = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let c = captures.get(3).unwrap().as_str().chars().next().unwrap();
        let pwd = captures.get(4).unwrap().as_str();
        let count = pwd.chars().fold(0, |acc, cc| acc + if c == cc { 1 } else { 0 });
        let acc1 = if count >= low && count <= high { acc.0 + 1 } else { acc.0 };
        let chars = pwd.chars().collect::<Vec<_>>();
        let acc2 = if high <= chars.len() &&
           ((chars[low - 1] == c && chars[high - 1] != c) ||
            (chars[low - 1] != c && chars[high - 1] == c)) {
            acc.1 + 1
        } else {
            acc.1
        };
        (acc1, acc2)
    });
    println!("{}, {}", count1, count2)
}