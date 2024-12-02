use std::cmp::Ordering;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let mut count = 0;
    let mut levels = Vec::new();

    for line in input.lines() {
        levels.clear();
        levels.extend(
            line.split_whitespace()
                .map(|level| level.parse::<u64>().unwrap()),
        );

        let direction = levels[0].cmp(&levels[1]);

        if levels
            .windows(2)
            .all(|pair| pair_is_safe(pair[0], pair[1], direction))
        {
            count += 1;
        }
    }

    count
}

fn pair_is_safe(first: u64, second: u64, direction: Ordering) -> bool {
    first.cmp(&second) == direction && first.abs_diff(second) <= 3
}

fn two(input: &str) -> u64 {
    let mut count = 0;
    let mut levels = Vec::new();

    for line in input.lines() {
        levels.clear();
        levels.extend(
            line.split_whitespace()
                .map(|level| level.parse::<u64>().unwrap()),
        );

        if PartTwo::is_safe_in_direction(&levels, Ordering::Greater)
            || PartTwo::is_safe_in_direction(&levels, Ordering::Less)
        {
            count += 1;
        }
    }

    count
}

#[derive(Clone, Copy)]
struct PartTwo<'a> {
    levels: &'a [u64],
    direction: Ordering,
    tolerate_err: bool,
}

impl<'a> PartTwo<'a> {
    fn is_safe_in_direction(levels: &'a [u64], direction: Ordering) -> bool {
        Self {
            levels,
            direction,
            tolerate_err: true,
        }
        .is_safe_after(0)
    }

    fn is_safe_after(self, idx: usize) -> bool {
        idx + 1 >= self.levels.len()
            || self.is_safe_after_prev(idx + 1, idx)
            || self.is_recoverable_after(idx)
    }

    fn is_safe_after_prev(self, idx: usize, prev: usize) -> bool {
        let prev = self.levels[prev];
        let curr = self.levels[idx];
        pair_is_safe(prev, curr, self.direction) && self.is_safe_after(idx)
    }

    fn is_recoverable_after(mut self, idx: usize) -> bool {
        if self.tolerate_err {
            self.tolerate_err = false;

            idx + 2 >= self.levels.len()
                || self.is_safe_after_prev(idx + 2, idx)
                || idx == 0 && self.is_safe_after(idx + 1)
                || idx > 0 && self.is_safe_after_prev(idx + 1, idx - 1)
        } else {
            false
        }
    }
}
