const INPUT: [u32; 6] = [0, 13, 1, 8, 6, 15];
const PART1_GOAL: usize = 2020;
const PART2_GOAL: usize = 30000000;

pub fn solve() {
    let mut last_spoken_number = *INPUT.last().unwrap();
    let mut spoken_numbers = vec![0u32; u32::max_value() as usize + 1];
    INPUT.iter().enumerate()
        .for_each(|(i, &n)| { spoken_numbers[n as usize] = i as u32 + 1; });

    let mut play = |start: usize, goal: usize| {
        for turn in start+1..=goal {
            let mut new_last_spoken_number = 0;
            let earlier_turn = &mut spoken_numbers[last_spoken_number as usize];
            if *earlier_turn != 0 {
                new_last_spoken_number = turn as u32 - 1 - *earlier_turn;
                *earlier_turn = turn as u32 - 1;
            }
            if spoken_numbers[new_last_spoken_number as usize] == 0 {
                spoken_numbers[new_last_spoken_number as usize] = turn as u32;
            }
            last_spoken_number = new_last_spoken_number;
        }
        print!("{}", last_spoken_number);
    };

    play(INPUT.len(), PART1_GOAL);
    print!(", ");
    play(PART1_GOAL, PART2_GOAL);
    println!();
}