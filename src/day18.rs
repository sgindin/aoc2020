use crate::tools;

fn eval_postfix(mut ops: Vec<char>, mut vals: Vec<i64>) -> i64 {
    while !ops.is_empty() {
        let op = ops.pop().unwrap();
        let left = vals.pop().unwrap();
        let right = vals.pop().unwrap();
        let res = match op {
            '+' => left + right,
            '*' => left * right,
            _ => unreachable!(),
        };
        vals.push(res);
    }
    assert_eq!(vals.len(), 1);
    *vals.first().unwrap()
}

fn unwind(ops: &mut Vec<char>, vals: &mut Vec<i64>, add_over_mul: bool) {
    let mut o = Vec::new();
    let mut v = Vec::new();
    while !ops.is_empty() {
        let op = ops.pop().unwrap();
        match op {
            '(' => break,
            '+' if add_over_mul => {
                let left = vals.pop().unwrap();
                let right = vals.pop().unwrap();
                vals.push(left + right);
            },
            '+'|'*' => {
                o.push(op);
                v.push(vals.pop().unwrap());
            },
            _ => unreachable!(),
        }
    }
    if !o.is_empty() {
        v.push(vals.pop().unwrap());
        vals.push(eval_postfix(o, v));
    }
}

fn evaluate(expression: &str, add_over_mul: bool) -> i64 {
    let mut ops = Vec::new();
    let mut vals = Vec::new();
    let mut val = String::default();

    for c in expression.chars().chain(std::iter::once(' ')) {
        if c.is_digit(10) {
            val.push(c);
            continue
        }
        if !val.is_empty() {
            vals.push(val.parse::<i64>().unwrap());
            val.clear();
        }
        match c {
            '*'|'+'|'(' => ops.push(c),
            ')' => unwind(&mut ops, &mut vals, add_over_mul),
            ' ' => (),
            _ => unreachable!(),
        }
    }
    unwind(&mut ops, &mut vals, add_over_mul);
    *vals.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(evaluate("1 + 2 * 3", false), 9);
    }

    #[test]
    fn test_simple_2() { assert_eq!(evaluate("1 + 2 * 3", true), 9); }

    #[test]
    fn test_braces_beg() {
        assert_eq!(evaluate("(1 + 2) * 3", false), 9);
    }

    #[test]
    fn test_braces_beg_2() {
        assert_eq!(evaluate("(1 + 2) * 3", true), 9);
    }

    #[test]
    fn test_braces_end() {
        assert_eq!(evaluate("1 + (2 * 3)", false), 7);
    }

    #[test]
    fn test_braces_end_2() {
        assert_eq!(evaluate("1 + (2 * 3)", true), 7);
    }

    #[test]
    fn test_braces_mid() {
        assert_eq!(evaluate("1 + (2 * 3) + 4", false), 11);
    }

    #[test]
    fn test_braces_mid_2() {
        assert_eq!(evaluate("1 + (2 * 3) + 4", true), 11);
    }

    #[test]
    fn test_braces_nested() {
        assert_eq!(evaluate("1 + (2 + (3 * 4) * 2)", false), 29);
    }

    #[test]
    fn test_braces_nested_2() { assert_eq!(evaluate("1 + (2 + (3 * 4) * 2)", true), 29); }

    #[test]
    fn test_aoc_2() {
        assert_eq!(evaluate("1 + (2 * 3) + (4 * (5 + 6))", true), 51);
        assert_eq!(evaluate("2 * 3 + (4 * 5)", true), 46);
        assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", true), 1445);
        assert_eq!(evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", true), 669060);
        assert_eq!(evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true), 23340);
    }
}

pub fn solve() {
    let (part1, part2) = tools::read_lines("./input/day18.txt")
        .unwrap()
        .fold((0, 0), |(part1, part2), line| {
            let line = line.unwrap();
            (part1 + evaluate(&line, false),
                part2 + evaluate(&line, true))
        });
    println!("{}, {}", part1, part2)
}