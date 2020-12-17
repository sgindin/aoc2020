use crate::tools;
use regex::{Regex, Captures};
use std::ops::RangeInclusive;

lazy_static! {
    static ref RULE_RE: Regex =
        Regex::new(r"^([[:alpha:]\s]+):\s(\d+)-(\d+)\sor\s(\d+)-(\d+)").unwrap();
}

pub fn solve() {
    let mut lines = tools::read_lines("./input/day16.txt").unwrap();

    // parse rules
    let mut rules = vec![];
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let captures: Captures = RULE_RE.captures(&line).unwrap();
        let rule = captures.get(1).unwrap().as_str().to_owned();
        let range_1 =
            captures.get(2).unwrap().as_str().parse::<u32>().unwrap()..=
                captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let range_2 =
            captures.get(4).unwrap().as_str().parse::<u32>().unwrap()..=
                captures.get(5).unwrap().as_str().parse::<u32>().unwrap();
        rules.push((rule, range_1, range_2));
    }

    if lines.next().unwrap().unwrap() != "your ticket:" {
        unreachable!();
    }

    // parse my ticket
    let my_ticket = lines.next().unwrap().unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    if !lines.next().unwrap().unwrap().is_empty() ||
        lines.next().unwrap().unwrap() != "nearby tickets:" {
        unreachable!();
    }

    // parse nearby tickets
    let nearby_tickets = lines.map(|line| {
        line.unwrap()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    let sat = |rule: &(String, RangeInclusive<u32>, RangeInclusive<u32>), number: &u32| {
        rule.1.contains(number) || rule.2.contains(number)
    };

    let part1 = nearby_tickets.iter()
        .map(|ticket| ticket.iter()
            .map(|number|
                if rules.iter().any(|rule| sat(rule, number)) { None } else { Some(number) })
            .flatten()
            .sum::<u32>())
        .sum::<u32>();

    let mut sat_matrix = nearby_tickets.into_iter()
        .fold(vec![vec![true; rules.len()]; my_ticket.len()], |mut sat_matrix, ticket| {
            for (i, number) in ticket.into_iter().enumerate() {
                let sat_rules = rules.iter()
                    .map(|rule| sat(rule, &number))
                    .collect::<Vec<_>>();
                if sat_rules.iter().any(|x| *x) {
                    sat_matrix[i].iter_mut().zip(sat_rules).for_each(|(x, y)| *x &= y);
                }
            }
            sat_matrix
        })
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>();
    sat_matrix.sort_by_cached_key(|x| x.1.iter().filter(|x| **x).count());
    let part2 = sat_matrix
        .into_iter()
        .fold(vec![], |mut fields, (field, mut sat)| {
            fields.iter().for_each(|(_, x)| sat[*x] = false);
            let rule = sat.iter().enumerate().filter_map(|(i, x)| if *x {Some(i)} else {None}).collect::<Vec<_>>();
            if rule.len() != 1 {
                unreachable!();
            }
            fields.push((field, rule[0]));
            fields
        })
        .iter()
        .map(|&(field, rule)| (rules[rule].0.as_str(), my_ticket[field]))
        .filter(|&(x, _)| x.starts_with("departure"))
        .fold(1u64, |acc, (_, x)| acc * x as u64);

    println!("{}, {}", part1, part2)
}