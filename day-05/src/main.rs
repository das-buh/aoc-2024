use aoc::{ArrayVec, FxHashMap};
use std::cmp::Ordering;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let (ordering, updates) = parse_input(input);
    let mut sum = 0;

    for update in updates {
        if update_in_order(&update, &ordering) {
            sum += update[update.len() / 2] as u64;
        }
    }

    sum
}

fn two(input: &str) -> u64 {
    let (ordering, updates) = parse_input(input);
    let mut sum = 0;

    for mut update in updates {
        let mut in_order = true;

        update.sort_by(|a, b| {
            if pair_in_order(*a, *b, &ordering) {
                Ordering::Less
            } else {
                in_order = false;
                Ordering::Greater
            }
        });

        if !in_order {
            sum += update[update.len() / 2] as u64;
        }
    }

    sum
}

type PageOrdering = FxHashMap<u8, ArrayVec<u8, 24>>;

fn parse_input(input: &str) -> (PageOrdering, impl Iterator<Item = ArrayVec<u8, 24>> + '_) {
    let mut lines = input.lines();
    let mut ordering = PageOrdering::default();

    for rule in lines.by_ref() {
        if rule.is_empty() {
            break;
        }

        let (page, after) = rule.split_once('|').unwrap();
        let (page, after) = (page.parse().unwrap(), after.parse().unwrap());

        ordering.entry(page).or_default().push(after);
    }

    let updates = lines.map(|update| {
        update
            .split(',')
            .map(|p| p.parse().unwrap())
            .collect::<ArrayVec<u8, 24>>()
    });

    (ordering, updates)
}

fn pair_in_order(page: u8, after: u8, ordering: &PageOrdering) -> bool {
    ordering.get(&page).is_some_and(|a| a.contains(&after))
}

fn update_in_order(pages: &[u8], ordering: &PageOrdering) -> bool {
    pages.windows(2).all(|window| {
        let [page, after] = window else { panic!() };
        pair_in_order(*page, *after, ordering)
    })
}
