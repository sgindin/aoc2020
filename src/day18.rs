use crate::tools;

fn eval_ops(mut ops: Vec<char>, mut vals: Vec<i64>) -> i64 {
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

fn evaluate(expression: &str) -> i64 {
    let mut ops = Vec::new();
    let mut vals = Vec::new();
    let mut val = String::default();

    for c in expression.chars().chain(std::iter::once(' ')) {
        if c.is_digit(10) {
            val.push(c);
        } else {
            if !val.is_empty() {
                vals.push(val.parse::<i64>().unwrap());
                val.clear();
            }
            match c {
                '+' | '*' | '(' => ops.push(c),
                ')' => {
                    let mut o = Vec::new();
                    let mut v = Vec::new();
                    while *ops.last().unwrap() != '(' {
                        o.push(ops.pop().unwrap());
                        v.push(vals.pop().unwrap());
                    }
                    v.push(vals.pop().unwrap());
                    vals.push(eval_ops(o, v));
                    assert_eq!(ops.pop().unwrap(), '(');
                },
                ' ' => (),
                _ => unreachable!(),
            }
        }
    }

    ops.reverse();
    vals.reverse();

    eval_ops(ops, vals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(evaluate("1 + 2 * 3"), 9);
    }

    #[test]
    fn test_braces_beg() {
        assert_eq!(evaluate("(1 + 2) * 3"), 9);
    }

    #[test]
    fn test_braces_end() {
        assert_eq!(evaluate("1 + (2 * 3)"), 7);
    }

    #[test]
    fn test_braces_mid() {
        assert_eq!(evaluate("1 + (2 * 3) + 4"), 11);
    }

    #[test]
    fn test_braces_nested() {
        assert_eq!(evaluate("1 + (2 + (3 * 4) * 2)"), 29);
    }
}

pub fn solve() {
    let part1 = tools::read_lines("./input/day18.txt")
        .unwrap()
        .fold(0, |acc, line| {
            acc + evaluate(&line.unwrap())
        });
    println!("{}", part1)
}