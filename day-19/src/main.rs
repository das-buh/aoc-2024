use aoc::FxHashSet;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let (towels, max_towel_len, designs, _) = parse_input(input);

    designs
        .into_iter()
        .filter(|design| is_possible(design, &towels, max_towel_len))
        .count() as u64
}

fn two(input: &str) -> u64 {
    let (towels, max_towel_len, designs, max_design_len) = parse_input(input);
    let mut cache = vec![u64::MAX; max_design_len + 1];

    designs
        .into_iter()
        .map(|design| {
            cache.fill(u64::MAX);
            count_possible(design, &towels, max_towel_len, &mut cache)
        })
        .sum()
}

fn is_possible(design: &[u8], towels: &FxHashSet<&[u8]>, max_towel_len: usize) -> bool {
    if design.is_empty() {
        return true;
    }

    for prefix in 0..=design.len().min(max_towel_len) {
        let (prefix, rest) = design.split_at(prefix);

        if towels.contains(prefix) && is_possible(rest, towels, max_towel_len) {
            return true;
        }
    }

    false
}

fn count_possible(
    design: &[u8],
    towels: &FxHashSet<&[u8]>,
    max_towel_len: usize,
    cache: &mut Vec<u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }

    let possible = cache[design.len()];
    if possible != u64::MAX {
        return possible;
    }

    let mut possible = 0;

    for prefix in 0..=design.len().min(max_towel_len) {
        let (prefix, rest) = design.split_at(prefix);

        if towels.contains(prefix) {
            possible += count_possible(rest, towels, max_towel_len, cache);
        }
    }

    cache[design.len()] = possible;
    possible
}

fn parse_input(input: &str) -> (FxHashSet<&[u8]>, usize, Vec<&[u8]>, usize) {
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
        .collect::<Vec<_>>();

    let max_design_len = designs.iter().map(|design| design.len()).max().unwrap();

    (towels, max_towel_len, designs, max_design_len)
}
