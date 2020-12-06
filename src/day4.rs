use crate::tools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref REQUIRED_KEYS: HashSet<&'static str> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned().collect();
    static ref BYR_RN: std::ops::RangeInclusive<u32> = 1920..=2002;
    static ref IYR_RN: std::ops::RangeInclusive<u32> = 2010..=2020;
    static ref EYR_RN: std::ops::RangeInclusive<u32> = 2020..=2030;
    static ref HGT_RE: Regex = Regex::new(r"([0-9]+)(cm|in)").unwrap();
    static ref HGT_RN: HashMap<&'static str, std::ops::RangeInclusive<u32>> =
        [("cm", 150..=193), ("in", 59..=76)].iter().cloned().collect();
    static ref HCL_RE: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    static ref ECL_SET: HashSet<&'static str> =
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();
    static ref PID_RE: Regex = Regex::new(r"[0-9]{9}").unwrap();
}

fn is_valid(key: &str, value: &str) -> bool {
    match key {
        "byr" => BYR_RN.contains(&value.parse::<u32>().unwrap()),
        "iyr" => IYR_RN.contains(&value.parse::<u32>().unwrap()),
        "eyr" => EYR_RN.contains(&value.parse::<u32>().unwrap()),
        "hgt" => {
            if let Some(captures) = HGT_RE.captures(value) {
                let height = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let units = captures.get(2).unwrap().as_str();
                HGT_RN[units].contains(&height)
            } else {
                false
            }
        },
        "hcl" => HCL_RE.is_match(value),
        "ecl" => ECL_SET.contains(value),
        "pid" => PID_RE.is_match(value),
        _ => unreachable!()
    }
}

fn check(passport: &str) -> (bool, bool) {
    let (keys, all_values_are_valid) = passport
        .split_whitespace()
        .map(|str| {
            let mut tokens = str.split(':');
            (tokens.next().unwrap(), tokens.next().unwrap())
        })
        .filter(|(key, _)| *key != "cid")
        .fold((HashSet::new(), true), |(mut keys, values_are_valid), (key, value)| {
            keys.insert(key);
            (keys, values_are_valid && is_valid(key, value))
        });
    (keys == *REQUIRED_KEYS, all_values_are_valid)
}

pub fn solve() {
    let (valid_count_1, valid_count_2, _) = tools::read_lines("./input/day4.txt")
        .unwrap()
        .chain(std::iter::once(Ok(String::default())))
        .fold((0, 0, String::default()), |(mut valid_count_1,
                                                  mut valid_count_2,
                                                  mut passport),
                                                  line| {
            let line = line.unwrap();
            if line.is_empty() {
                let (has_required_keys, all_values_are_valid) = check(&passport);
                if has_required_keys {
                    valid_count_1 += 1;
                    if all_values_are_valid {
                        valid_count_2 += 1;
                    }
                }
                passport.clear();
            } else {
                passport.push_str(&line);
                passport.push(' ');
            }
            (valid_count_1, valid_count_2, passport)
        });
    println!("{}, {}", valid_count_1, valid_count_2)
}
