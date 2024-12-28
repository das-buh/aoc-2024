fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let (locks, keys) = parse_input(input);
    let mut count = 0;

    for lock in locks {
        for key in keys.iter().copied() {
            if lock & key == 0 {
                count += 1;
            }
        }
    }

    count
}

fn two(_: &str) -> &'static str {
    "when the chronicle is delivered"
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for pat in input.split("\n\n") {
        let mask = pat
            .chars()
            .filter_map(|c| match c {
                '#' => Some(1),
                '.' => Some(0),
                _ => None,
            })
            .enumerate()
            .map(|(i, b)| b << i)
            .sum();

        match pat.chars().next().unwrap() {
            '#' => locks.push(mask),
            '.' => keys.push(mask),
            _ => panic!(),
        }
    }

    (locks, keys)
}
