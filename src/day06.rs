use crate::tools;

pub fn solve() {
    let (total_anyone_yes_count, total_everyone_yes_count, _) =
        tools::read_lines("./input/day06.txt")
        .unwrap()
        .chain(std::iter::once(Ok(String::default())))
        .fold((0, 0, vec![(false, true); 26]), |(mut group_anyone_yes_count,
                                                        mut group_everyone_yes_count,
                                                        mut group_answers),
                                                       line| {
            let line = line.unwrap();
            if line.is_empty() {
                group_anyone_yes_count += group_answers.iter().filter(|x| x.0).count();
                group_everyone_yes_count += group_answers.iter().filter(|x| x.1).count();
                group_answers = vec![(false, true); 26];
            } else {
                let mut person_answers = vec![false; 26];
                line.chars().for_each(|c| person_answers[c as usize - 'a' as usize] = true);
                group_answers.iter_mut().zip(person_answers)
                    .for_each(|(group_answer, person_answer)| {
                        (*group_answer).0 |= person_answer;
                        (*group_answer).1 &= person_answer;
                    });
            }
            (group_anyone_yes_count, group_everyone_yes_count, group_answers)
        });
    println!("{}, {}", total_anyone_yes_count, total_everyone_yes_count)
}