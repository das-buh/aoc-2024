use aoc::grid::*;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let garden = parse_input(input);
    find_price(garden, |area, perim, _| area * perim)
}

fn two(input: &str) -> u64 {
    let garden = parse_input(input);
    find_price(garden, |area, _, corners| area * corners)
}

#[derive(Clone, Copy)]
struct Plot {
    plant: char,
    seen_by_neighbor: Option<(u64, Edges)>,
    seen_by_self: bool,
}

type Edges = u64;

const VISITING: Edges = 1 << 4;

fn parse_input(input: &str) -> Grid<Plot> {
    Grid::from_str(input, |plant| Plot {
        plant,
        seen_by_neighbor: None,
        seen_by_self: false,
    })
}

fn find_price(mut garden: Grid<Plot>, mut region_price: impl FnMut(u64, u64, u64) -> u64) -> u64 {
    let mut queue = Vec::new();
    let mut region = 0;

    queue.push((0, 0));

    let mut price = 0;

    while let Some(pos) = queue.pop() {
        if garden[pos].seen_by_self {
            continue;
        }
        garden[pos].seen_by_neighbor = Some((region, VISITING));
        garden[pos].seen_by_self = true;

        let (area, region_perim, garden_perim, sides) =
            find_region(pos, region, &mut garden, &mut queue);
        price += region_price(area + 1, region_perim + garden_perim, sides);
        region += 1;
    }

    price
}

fn find_region(
    pos: (usize, usize),
    region: u64,
    garden: &mut Grid<Plot>,
    queue: &mut Vec<(usize, usize)>,
) -> (u64, u64, u64, u64) {
    let plant = garden[pos].plant;

    let (mut area, mut region_perim, mut garden_perim, mut corners) = (0, 0, 0, 0);
    let mut edges = 0;

    for (i, dir) in CARDINAL_DIRS.into_iter().enumerate() {
        let Some(neighbor) = garden.translate(pos, dir) else {
            garden_perim += 1;
            edges |= 1 << i;
            continue;
        };

        if garden[neighbor].plant != plant {
            let new_edges = match garden[neighbor].seen_by_neighbor {
                Some((seen_by, old_edges)) if seen_by == region => {
                    let new_edges = old_edges | (1 << i);
                    corners += corner_count(new_edges) - corner_count(old_edges);
                    new_edges
                }
                Some(_) | None => 1 << i,
            };
            garden[neighbor].seen_by_neighbor = Some((region, new_edges));

            if !garden[neighbor].seen_by_self {
                queue.push(neighbor);
            }

            region_perim += 1;
            edges |= 1 << i;
            continue;
        }

        if garden[neighbor].seen_by_self {
            continue;
        }
        garden[neighbor].seen_by_neighbor = Some((region, VISITING));
        garden[neighbor].seen_by_self = true;
        area += 1;

        let (add_area, add_region_perim, add_garden_perim, add_corners) =
            find_region(neighbor, region, garden, queue);
        area += add_area;
        region_perim += add_region_perim;
        garden_perim += add_garden_perim;
        corners += add_corners;
    }

    corners += corner_count(edges);

    for (i, dir) in CARDINAL_DIRS.into_iter().enumerate() {
        let dir = (dir.0 + dir.1, dir.1 - dir.0);
        let mask = (1 << i) | (1 << ((i + 1) % 4));
        if edges & mask == mask {
            let Some(diag) = garden.translate(pos, dir) else {
                continue;
            };
            let i = garden.translate(pos, (dir.0, 0)).unwrap();
            let j = garden.translate(pos, (0, dir.1)).unwrap();
            if garden[i].plant != plant && garden[j].plant != plant {
                if garden[diag].seen_by_neighbor == Some((region, 0)) {
                    corners -= 2;
                }
            }
        }
    }

    if let Some((region, edges)) = garden[pos].seen_by_neighbor {
        if edges == VISITING {
            garden[pos].seen_by_neighbor = Some((region, 0));
        }
    }

    (area, region_perim, garden_perim, corners)
}

fn corner_count(edges: Edges) -> u64 {
    let edge_count = edges.count_ones() as u64;
    match edge_count {
        2 if edges & 0b0001 == (edges & 0b0100) >> 2 => 0,
        1..=3 => edge_count - 1,
        edge_count => edge_count,
    }
}
