use aoc::{FxHashMap, FxHashSet};

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let map = parse_input(input);
    let mut antinodes = FxHashSet::default();

    for antennae in map.antennae.into_values() {
        for ((ai, aj), (bi, bj)) in iter(antennae) {
            let (di, dj) = (bi - ai, bj - aj);
            let (a, b) = ((ai - di, aj - dj), (bi + di, bj + dj));

            if in_bounds(a, map.dim) {
                antinodes.insert(a);
            }
            if in_bounds(b, map.dim) {
                antinodes.insert(b);
            }
        }
    }

    antinodes.len() as u64
}

fn two(input: &str) -> u64 {
    let map = parse_input(input);
    let mut antinodes = FxHashSet::default();

    for antennae in map.antennae.into_values() {
        for ((ai, aj), (bi, bj)) in iter(antennae) {
            let (di, dj) = (bi - ai, bj - aj);
            let gcd = gcd(di, dj);
            let (di, dj) = (di / gcd, dj / gcd);

            let (mut i, mut j) = (ai, aj);
            while in_bounds((i, j), map.dim) {
                antinodes.insert((i, j));
                i += di;
                j += dj;
            }

            let (mut i, mut j) = (ai, aj);
            while in_bounds((i, j), map.dim) {
                antinodes.insert((i, j));
                i -= di;
                j -= dj;
            }
        }
    }

    antinodes.len() as u64
}

fn iter(antennae: Vec<(i64, i64)>) -> impl Iterator<Item = ((i64, i64), (i64, i64))> {
    let indices = 0..antennae.len();
    indices
        .clone()
        .flat_map(move |a| indices.clone().map(move |b| (a, b)))
        .filter(|(a, b)| a != b)
        .map(move |(a, b)| (antennae[a], antennae[b]))
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn in_bounds((ai, aj): (i64, i64), (mi, mj): (i64, i64)) -> bool {
    (0..mi).contains(&ai) && (0..mj).contains(&aj)
}

fn parse_input(input: &str) -> Map {
    let mut map = Map {
        antennae: FxHashMap::default(),
        dim: (0, 0),
    };

    for (i, line) in input.lines().enumerate() {
        map.dim.0 += 1;
        for (j, char) in line.chars().enumerate() {
            map.dim.1 += 1;
            if char != '.' {
                map.antennae
                    .entry(char)
                    .or_default()
                    .push((i as i64, j as i64));
            }
        }
    }

    map.dim.1 /= map.dim.0;
    map
}

struct Map {
    antennae: FxHashMap<char, Vec<(i64, i64)>>,
    dim: (i64, i64),
}
