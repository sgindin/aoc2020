use crate::tools;

enum Command {
    Acc,
    Nop,
    Jmp,
}

fn patch(command: &mut Command) -> bool {
    match *command {
        Command::Nop => { *command = Command::Jmp; true }
        Command::Jmp => { *command = Command::Nop; true }
        _ => false
    }
}

fn restore(command: &mut Command) {
    if !patch(command) {
        unreachable!()
    }
}

struct Instruction {
    command: Command,
    parameter: i32,
    executed: bool
}

pub fn solve() {
    // Parse the program
    let mut program = tools::read_lines("./input/day08.txt")
        .unwrap()
        .map(|line| {
            let line = line.unwrap();
            let mut tokens = line.split_whitespace();
            let command = match tokens.next().unwrap() {
                "acc" => Command::Acc,
                "nop" => Command::Nop,
                "jmp" => Command::Jmp,
                _ => unreachable!()
            };
            let parameter = tokens.next().unwrap().parse::<i32>().unwrap();
            Instruction {
                command,
                parameter,
                executed: false,
            }
        })
        .collect::<Vec<_>>();
    // Run the program
    let mut accumulator = 0;
    let mut ip: i64 = 0;
    let mut patched_ip: i64 = -1;
    let mut stack: Vec<(i64, i32)> = vec![];
    loop {
        if !(0..program.len() as i64).contains(&ip) {
            // Program has finished (accumulator holds answer to part 2)
            println!(", {}", accumulator);
            break
        }
        if program[ip as usize].executed {
            if patched_ip == -1 {
                // First IL (accumulator holds answer to part 1)
                print!("{}", accumulator);
            }
            // Unwind stack to the first patchable instruction which we haven't tried yet.
            loop {
                let frame = stack.pop().expect("Ran out of stack!");
                ip = frame.0;
                accumulator = frame.1;
                program[ip as usize].executed = false;
                if patched_ip != -1 {
                    if patched_ip == ip {
                        restore(&mut program[ip as usize].command);
                        patched_ip = -1;
                    }
                } else if patch(&mut program[ip as usize].command) {
                    patched_ip = ip;
                    break
                }
            }
        }
        // Execute current instruction
        stack.push((ip, accumulator));
        program[ip as usize].executed = true;
        match program[ip as usize].command {
            Command::Acc => accumulator += program[ip as usize].parameter,
            Command::Jmp => ip += program[ip as usize].parameter as i64 - 1,
            Command::Nop => (),
        }
        ip += 1;
    }
}