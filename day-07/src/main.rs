fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    parse_and_solve(input, is_possible_eq)
}

fn two(input: &str) -> u64 {
    parse_and_solve(input, is_possible_with_concat)
}

fn is_possible_eq(result: u64, ops: &[u64], acc: u64) -> bool {
    if acc > result {
        return false;
    }

    let Some((next, rest)) = ops.split_first() else {
        return acc == result;
    };
    is_possible_eq(result, rest, acc + next) || is_possible_eq(result, rest, acc * next)
}

fn is_possible_with_concat(result: u64, ops: &[u64], acc: u64) -> bool {
    if acc > result {
        return false;
    }

    let Some((next, rest)) = ops.split_first() else {
        return acc == result;
    };
    is_possible_with_concat(result, rest, acc + next)
        || is_possible_with_concat(result, rest, acc * next)
        || is_possible_with_concat(result, rest, acc * 10_u64.pow(next.ilog10() + 1) + next)
}

fn parse_and_solve(input: &str, predicate: impl Fn(u64, &[u64], u64) -> bool) -> u64 {
    let mut sum = 0;

    for line in input.lines() {
        let (result, ops) = line.split_once(':').unwrap();
        let result = result.parse::<u64>().unwrap();
        let ops = ops
            .split_whitespace()
            .map(|o| o.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        if predicate(result, &ops[1..], ops[0]) {
            sum += result;
        }
    }

    sum
}
