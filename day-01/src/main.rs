use aoc::FxHashMap;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    }

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right.into_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum::<u64>()
}

fn two(input: &str) -> u64 {
    let mut left = Vec::new();
    let mut right = FxHashMap::<u64, u64>::default();

    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap());
        left.push(nums.next().unwrap());
        *right.entry(nums.next().unwrap()).or_default() += 1;
    }

    left.into_iter()
        .map(|num| num * right.get(&num).copied().unwrap_or_default())
        .sum::<u64>()
}
