use aoc::{ArrayVec, FxHashMap};

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
        if update_in_order(&update, &ordering) {
            continue;
        }

        for i in 0..update.len() {
            let correct = (i..update.len())
                .find(|page| {
                    let page = update[*page];
                    update[i..]
                        .iter()
                        .copied()
                        .all(|other| !pair_in_order(other, page, &ordering))
                })
                .unwrap();

            update.swap(i, correct);
        }

        sum += update[update.len() / 2] as u64;
    }

    sum
}

type Ordering = FxHashMap<u8, ArrayVec<u8, 24>>;

fn parse_input<'a>(input: &'a str) -> (Ordering, impl Iterator<Item = ArrayVec<u8, 24>> + 'a) {
    let mut lines = input.lines();
    let mut ordering = Ordering::default();

    while let Some(rule) = lines.next() {
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

fn pair_in_order(page: u8, after: u8, ordering: &Ordering) -> bool {
    ordering.get(&page).is_some_and(|a| a.contains(&after))
}

fn update_in_order(pages: &[u8], ordering: &Ordering) -> bool {
    pages.windows(2).all(|window| {
        let [page, after] = window else { panic!() };
        pair_in_order(*page, *after, ordering)
    })
}
