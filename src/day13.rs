use crate::tools;

pub fn solve() {
    let mut lines = tools::read_lines("./input/day13.txt").unwrap();
    let start_time = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let bus_ids = lines.next().unwrap().unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, token)| token != "x")
        .map(|(offset, token)| (offset, token.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    let part1 = bus_ids.iter()
        .map(|&(_, id)| (id, id - start_time % id))
        .min_by(|&l, &r| l.1.cmp(&r.1))
        .map(|(id, delay)| id * delay)
        .unwrap();

    let mut part2 = 0;
    loop {
        let (count, step) = bus_ids.iter()
            .filter(|&(offset, id)| (part2 + offset) % id == 0)
            .fold((0, 1), |(count, product), &(_, id)| (count + 1, product * id));
        if count == bus_ids.len() {
            break
        }
        part2 += step;
    }

    println!("{}, {}", part1, part2);
}