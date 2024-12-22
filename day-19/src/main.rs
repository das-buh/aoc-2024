use aoc::{FxHashMap, FxHashSet};

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let (towels, max_towel_len, designs) = parse_input(input);

    designs
        .into_iter()
        .filter(|design| is_possible(design, &towels, max_towel_len))
        .count() as u64
}

fn two(input: &str) -> u64 {
    let (towels, max_towel_len, designs) = parse_input(input);

    designs
        .into_iter()
        .map(|design| {
            let mut cache = FxHashMap::default();
            count_possible(design, &towels, max_towel_len, &mut cache)
        })
        .sum()
}

fn is_possible(design: &[u8], towels: &FxHashSet<&[u8]>, max_towel_len: usize) -> bool {
    if design.len() == 0 {
        return true;
    }

    for prefix in 0..=design.len().min(max_towel_len) {
        let (prefix, rest) = design.split_at(prefix);

        if towels.contains(prefix) && is_possible(rest, towels, max_towel_len) {
            return true;
        }
    }

    return false;
}

fn count_possible(
    design: &[u8],
    towels: &FxHashSet<&[u8]>,
    max_towel_len: usize,
    cache: &mut FxHashMap<usize, u64>,
) -> u64 {
    if design.len() == 0 {
        return 1;
    }

    if let Some(possible) = cache.get(&design.len()) {
        return *possible;
    }

    let mut possible = 0;

    for prefix in 0..=design.len().min(max_towel_len) {
        let (prefix, rest) = design.split_at(prefix);

        if towels.contains(prefix) {
            possible += count_possible(rest, towels, max_towel_len, cache);
        }
    }

    cache.insert(design.len(), possible);
    possible
}

fn parse_input(input: &str) -> (FxHashSet<&[u8]>, usize, Vec<&[u8]>) {
    use aoc::parse::*;

    let mut iter = sep_by(", ", word)(input);
    let towels = iter
        .by_ref()
        .map(|(towel, _)| towel.as_bytes())
        .collect::<FxHashSet<_>>();

    let max_towel_len = towels.iter().map(|towel| towel.len()).max().unwrap();

    let (_, input) = seq!("\n\n")(iter.src());

    let designs = sep_by("\n", word)(input)
        .map(|(towel, _)| towel.as_bytes())
        .collect();

    (towels, max_towel_len, designs)
}
