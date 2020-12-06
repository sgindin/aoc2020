use std::ops::RangeInclusive;
use crate::tools;

fn decode(code: &str, r: RangeInclusive<u8>) -> u8 {
    let reduced = code.chars().fold(r, |r, c| {
        match c {
            'F' | 'L' => *r.start()..=*r.start() + (*r.end() - *r.start())/2,
            'B' | 'R' => *r.start() + (*r.end() - *r.start())/2 + 1..=*r.end(),
            _ => unreachable!()
        }
    });
    assert_eq!(reduced.len(), 1);
    *reduced.start()
}

fn get_row(code: &str) -> u8 {
    decode(&code[..7], 0u8..=127)
}

fn get_column(code: &str) -> u8 {
    decode(&code[7..10], 0u8..=7)
}

fn get_seat_id(code: &str) -> usize {
    get_row(code) as usize * 8 + get_column(code) as usize
}

pub fn solve() {
    const MAX_ID: usize = 127*8 + 7;
    let mut ids = vec![false; MAX_ID + 1];
    tools::read_lines("./input/p5.txt")
        .unwrap()
        .map(|line| get_seat_id(line.unwrap().as_str()))
        .for_each(|id| ids[id] = true);
    let my_id = (0..=MAX_ID).find(|&i| {
        i > 0 && i < MAX_ID && ids[i - 1] && !ids[i] && ids[i + 1]
    }).unwrap();
    let max_id = ids.into_iter().enumerate().rfind(|(_, exists)| *exists).unwrap().0;
    println!("{}, {}", max_id, my_id);
}