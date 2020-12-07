use crate::tools;
use regex::{Regex, Captures};
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"([a-z]+\s[a-z]+) bags contain (.+)").unwrap();
    static ref CONTAINS_RE: Regex = Regex::new(r"(no|[\d+]) ([a-z]+\s[a-z]+) bags?[,\.]").unwrap();
}

const TARGET_COLOR: &'static str = "shiny gold";

pub fn solve() {
    let mut contained_by: HashMap<String, HashSet<String>> = HashMap::new();
    let mut contains: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    tools::read_lines("./input/day7.txt")
        .unwrap()
        .for_each(|line| {
            let line = line.unwrap();
            let captures: Captures = RULE_RE.captures(&line).unwrap();
            let containing_color = captures.get(1).unwrap().as_str();
            if !contains.contains_key(containing_color) {
                contains.insert(containing_color.to_owned(), vec![]);
            }
            let contained_colors = contains.get_mut(containing_color).unwrap();
            CONTAINS_RE.captures_iter(captures.get(2).unwrap().as_str())
                .for_each(|captures: Captures| {
                    let amount = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let color = captures.get(2).unwrap().as_str();
                    contained_colors.push((color.to_owned(), amount));
                    if let Some(colors) = contained_by.get_mut(color) {
                        colors.insert(containing_color.to_owned());
                    } else {
                        let mut colors = HashSet::new();
                        colors.insert(containing_color.to_owned());
                        contained_by.insert(color.to_owned(), colors);
                    }
                });
        });

    let mut colors_to_check = vec![TARGET_COLOR].iter().cloned().collect::<HashSet<_>>();
    let mut containing_colors = HashSet::new();
    while !colors_to_check.is_empty() {
        let color = colors_to_check.iter().next().cloned().unwrap();
        colors_to_check.remove(color);
        containing_colors.insert(color);
        if let Some(colors) = contained_by.get(color) {
            colors.iter().for_each(|c| { colors_to_check.insert(c.as_str()); } );
        }
    }
    let containing_bags_count = containing_colors.len() - 1; // subtract 1 for TARGET_COLOR

    fn count_nested_bags(color: &str, contains: &HashMap<String, Vec<(String, i32)>>) -> i32 {
        contains.get(color).unwrap().iter()
            .map(|(c, a)| (1 + count_nested_bags(c, contains)) * a)
            .sum()
    }
    let nested_bags_count = count_nested_bags(TARGET_COLOR, &contains);

    println!("{}, {}", containing_bags_count, nested_bags_count)
}