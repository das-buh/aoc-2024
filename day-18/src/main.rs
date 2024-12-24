use aoc::FxHashSet;
use std::{cmp::Ordering, collections::VecDeque};

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let (dim, count, bytes) = parse_input(input);

    let mut cannot_visit = FxHashSet::from_iter(bytes.into_iter().take(count));
    cannot_visit.insert((0, 0));

    let mut queue = vec![(0, 0)];
    let mut queue_new = Vec::new();

    let mut steps = 1;

    loop {
        for pos in queue.drain(..) {
            for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let pos = (pos.0 + dir.0, pos.1 + dir.1);

                if out_of_bounds(pos, dim, &cannot_visit) {
                    continue;
                }

                if pos == (dim, dim) {
                    return steps;
                }

                cannot_visit.insert(pos);
                queue_new.push(pos);
            }
        }

        queue.extend(queue_new.drain(..));
        steps += 1;
    }
}

fn two(input: &str) -> (i64, i64) {
    let (dim, _, bytes) = parse_input(input);
    let bytes = bytes.into_iter().enumerate().collect::<Vec<_>>();

    let mut cannot_visit = FxHashSet::default();
    let mut queue = VecDeque::new();

    let prevent = bytes.binary_search_by(|(i, _)| {
        cannot_visit.clear();
        cannot_visit.extend(bytes[..*i].iter().map(|(_, byte)| byte));
        cannot_visit.insert((0, 0));

        queue.clear();
        queue.push_back((0, 0));

        while let Some(pos) = queue.pop_front() {
            for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let pos = (pos.0 + dir.0, pos.1 + dir.1);

                if out_of_bounds(pos, dim, &cannot_visit) {
                    continue;
                }

                if pos == (dim, dim) {
                    return Ordering::Less;
                }

                cannot_visit.insert(pos);
                queue.push_back(pos);
            }
        }

        Ordering::Greater
    });

    match prevent {
        Ok(_) => panic!(),
        Err(i) => bytes[i - 1].1,
    }
}

fn out_of_bounds(pos: (i64, i64), dim: i64, cannot_visit: &FxHashSet<(i64, i64)>) -> bool {
    pos.0 < 0 || pos.0 > dim || pos.1 < 0 || pos.1 > dim || cannot_visit.contains(&pos)
}

fn parse_input(input: &str) -> (i64, usize, Vec<(i64, i64)>) {
    use aoc::parse::*;

    let (dim, input) = seq!("dim=", uint, "\n")(input);
    let (one_count, input) = seq!("one_count=", uint, "\n")(input);
    let bytes = sep_by("\n", seq!(uint, ",", uint))(input)
        .map(|((x, y), _)| (x as i64, y as i64))
        .collect();

    (dim as i64, one_count as usize, bytes)
}
