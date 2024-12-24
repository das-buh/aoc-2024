use aoc::{Grid, CARDINAL_DIRS};

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let (end, map) = parse_input(input);
    count_cheats(end, 2, map)
}

// 121459 is too low
// 216357 is too low
fn two(input: &str) -> u64 {
    let (end, map) = parse_input(input);
    count_cheats(end, 20, map)
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
    Wall,
    Empty,
    Seen { dist: u64 },
}

fn count_cheats(end: (usize, usize), cheat_dur: u64, mut map: Grid<Tile>) -> u64 {
    let mut queue = vec![end];
    let mut queue_new = Vec::new();
    map[end] = Tile::Seen { dist: 0 };

    let mut dist = 0;
    let mut count = 0;

    let dur = cheat_dur as isize;
    let quadrants = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
    let cheat_zone = (1..=dur)
        .flat_map(move |dur| (1..dur).map(move |i| (i, (dur - i))))
        .flat_map(|(i, j)| quadrants.into_iter().map(move |(si, sj)| (i * si, j * sj)))
        .chain((1..=dur).map(|i| (i, 0)))
        .chain((1..=dur).map(|i| (0, i)))
        .chain((1..=dur).map(|i| (-i, 0)))
        .chain((1..=dur).map(|i| (0, -i)));

    while !queue.is_empty() {
        for pos in queue.drain(..) {
            for cheat_dir in cheat_zone.clone() {
                let Some(cheat) = map.translate(pos, cheat_dir) else {
                    continue;
                };

                if let Tile::Seen { dist: cheat_dist } = map[cheat] {
                    let dur = (cheat_dir.0.unsigned_abs() + cheat_dir.1.unsigned_abs()) as u64;
                    let cheat_dist = cheat_dist + dur;

                    if dist - cheat_dist >= 100 {
                        count += 1;
                    }
                }
            }

            for dir in CARDINAL_DIRS {
                let pos = map.translate(pos, dir).unwrap();

                if map[pos] == Tile::Empty {
                    map[pos] = Tile::Seen { dist: dist + 1 };
                    queue_new.push(pos);
                }
            }
        }

        queue.append(&mut queue_new);
        dist += 1;
    }

    count
}

fn parse_input(input: &str) -> ((usize, usize), Grid<Tile>) {
    let mut end = None;

    let mut map = Grid::builder();

    for line in input.lines() {
        for char in line.chars() {
            map.tile(match char {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'S' => {
                    // the start doesn't even matter
                    Tile::Empty
                }
                'E' => {
                    end = Some(map.pos());
                    Tile::Empty
                }
                _ => panic!(),
            });
        }
        map.finish_line();
    }

    (end.unwrap(), map.finish_grid())
}
