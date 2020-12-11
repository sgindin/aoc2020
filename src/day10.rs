use crate::tools;

pub fn solve() {
    let mut numbers = tools::read_lines("./input/day10.txt")
        .unwrap()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    numbers.sort();

    let mut source_j = 0;
    let mut count_1j = 0;
    let mut count_3j = 1; // account for the implicit 3j interval after the last element

    let mut n_minus_3 = 1u64;
    let mut n_minus_2 = if numbers[1] <= 3 {2} else {1};
    let mut n_minus_1 =
        n_minus_2
        + if numbers[2] - numbers[0] <=3 {1} else {0}
        + if numbers[2] == 3 {1} else {0};
    let mut n = 0;

    for i in 0..numbers.len() {
        // Count 1j and 3j intervals
        match numbers[i] - source_j {
            1 => count_1j += 1,
            2 => (),
            3 => count_3j += 1,
            _ => unreachable!(),
        }
        source_j = numbers[i];

        // Count possible connections
        if i > 2 {
            n = n_minus_1;
            if numbers[i] - numbers[i - 2] <= 3 {
                n += n_minus_2;
            }
            if numbers[i] - numbers[i - 3] == 3 {
                n += n_minus_3;
            }
            n_minus_3 = n_minus_2;
            n_minus_2 = n_minus_1;
            n_minus_1 = n;
        }
    }

    println!("{}, {}", count_1j * count_3j, n);
}