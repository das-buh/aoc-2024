use aoc::grid::Grid;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let map = parse_input(input);

    hike(&map, Vec::new, |pos, seen| {
        if seen.contains(&pos) {
            0
        } else {
            seen.push(pos);
            1
        }
    })
}

fn two(input: &str) -> u64 {
    let map = parse_input(input);
    hike(&map, || (), |_, _| 1)
}

fn parse_input(input: &str) -> Grid<u64> {
    Grid::from_iter(
        input
            .lines()
            .map(|line| line.chars().map(|char| char.to_digit(10).unwrap() as u64)),
    )
}

fn hike<S: Clone>(
    map: &Grid<u64>,
    mut init_state: impl FnMut() -> S,
    visit_top: impl Fn((usize, usize), &mut S) -> u64 + Copy,
) -> u64 {
    let mut result = 0;

    for (pos, height) in map.iter() {
        if *height == 0 {
            let mut state = init_state();
            result += recurse(pos, map, &mut state, visit_top);
        }
    }

    result
}

fn recurse<S>(
    pos: (usize, usize),
    map: &Grid<u64>,
    state: &mut S,
    visit_top: impl Fn((usize, usize), &mut S) -> u64 + Copy,
) -> u64 {
    if map[pos] == 9 {
        return visit_top(pos, state);
    }

    let mut result = 0;

    for neighbor in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let Some(neighbor) = map.translate(pos, neighbor) else {
            continue;
        };
        if map[neighbor] == map[pos] + 1 {
            result += recurse(neighbor, map, state, visit_top);
        }
    }

    result
}
