use aoc::Regex;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> i64 {
    let claws = parse_input(input);
    claws
        .into_iter()
        .filter_map(|(ax, ay, bx, by, px, py)| find_cost(ax, ay, bx, by, px, py))
        .sum()
}

fn two(input: &str) -> i64 {
    let claws = parse_input(input);
    const ADD: i64 = 10000000000000;
    claws
        .into_iter()
        .filter_map(|(ax, ay, bx, by, px, py)| find_cost(ax, ay, bx, by, px + ADD, py + ADD))
        .sum()
}

#[derive(Clone, Copy)]
enum Solution {
    Found(i64),
    Indeterminate,
    None,
}

fn find_sol(fx: i64, fy: i64, ox: i64, oy: i64, px: i64, py: i64) -> Solution {
    let p = px * oy - py * ox;
    let q = fx * oy - fy * ox;

    match (p, q) {
        (0, 0) => Solution::Indeterminate,
        (p, q) if p % q == 0 => Solution::Found(p / q),
        _ => Solution::None,
    }
}

fn find_cost(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> Option<i64> {
    let a = find_sol(ax, ay, bx, by, px, py);
    let b = find_sol(bx, by, ax, ay, px, py);

    match (a, b) {
        (Solution::Found(a), Solution::Found(b)) => Some(3 * a + b),
        (Solution::Found(a), Solution::Indeterminate) => {
            let p = px - ax * a;
            (p % bx == 0).then(|| 3 * a + p / bx)
        }
        (Solution::Indeterminate, Solution::Found(b)) => {
            let p = px - bx * b;
            (p % ax == 0).then(|| 3 * p / ax + b)
        }
        (Solution::Indeterminate, Solution::Indeterminate) => Some(0),
        (Solution::None, _) | (_, Solution::None) => None,
    }
}

type Claw = (i64, i64, i64, i64, i64, i64);

fn parse_input(input: &str) -> Vec<Claw> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let mut claws = Vec::new();
    for (_, groups) in re.captures_iter(input).map(|c| c.extract()) {
        let [ax, ay, bx, by, px, py] = groups.map(|g| g.parse().unwrap());
        claws.push((ax, ay, bx, by, px, py))
    }

    claws
}
