use crate::tools;
use regex::{Regex, Captures};

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"^([[:alpha:]\s]+):\s(\d+)-(\d+)\sor\s(\d+)-(\d+)").unwrap();
}

pub fn solve() {
    let mut lines = tools::read_lines("./input/day16.txt").unwrap();

    let mut rules = vec![];
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // parse rules
        let captures: Captures = RULE_RE.captures(&line).unwrap();
        let rule = captures.get(1).unwrap().as_str().to_owned();
        let range_1 =
            captures.get(2).unwrap().as_str().parse::<u32>().unwrap()..=
                captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let range_2 =
            captures.get(3).unwrap().as_str().parse::<u32>().unwrap()..=
                captures.get(4).unwrap().as_str().parse::<u32>().unwrap();
        rules.push((rule, range_1, range_2))
    }

    if lines.next().unwrap().unwrap() != "your ticket:" {
        unreachable!();
    }

    // parse my ticket
    let _my_ticket = lines.next().unwrap().unwrap()
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

    let error_rate = nearby_tickets
        .into_iter()
        .map(|ticket| ticket.into_iter()
            .map(|number| {
                let valid = rules.iter()
                    .any(|rule| rule.1.contains(&number) || rule.2.contains(&number));
                if valid { None } else { Some(number) }
            })
            .flatten()
            .sum::<u32>())
        .sum::<u32>();

    println!("{}", error_rate)
}