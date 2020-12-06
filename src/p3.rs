use crate::tools;

pub fn solve() {
    let mut slopes = vec![(1, 1, 0), (3, 1, 0), (5, 1, 0), (7, 1, 0), (1, 2, 0)];
    let answer = tools::read_lines("./input/p3.txt")
        .unwrap()
        .enumerate()
        .fold(vec![0, 0, 0, 0, 0], |acc, (i, line)| {
            let line = line.unwrap();
            let mut acc = acc;
            for j in 0..slopes.len() {
                if i - slopes[j].2 == slopes[j].1 {
                    slopes[j].2 = i;
                    let target = ((i / slopes[j].1) * slopes[j].0) % line.len();
                    if line.chars().nth(target).unwrap() == '#' {
                        acc[j] += 1;
                    }
                }
            }
            acc
        });
    println!("{}, {}", answer[1], answer.iter().fold(1usize, |acc, i| acc*i ))
}