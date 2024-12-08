fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let mut map = parse_input(input);
    let mut traversal = map.new_traversal();

    while let Some((pos, _)) = map.traverse_next(&mut traversal) {
        *map.get_mut(pos) |= VISITED;
    }

    map.tiles
        .into_iter()
        .filter(|tile| tile & VISITED != 0)
        .count() as u64
}

fn two(input: &str) -> u64 {
    let mut map = parse_input(input);
    let mut traversal = map.new_traversal();

    while let Some((pos, _)) = map.traverse_next(&mut traversal) {
        *map.get_mut(pos) |= TEMP_OBSTR;
        if verify_loop(&mut map) {
            *map.get_mut(pos) |= ADD_OBSTR;
        }
        *map.get_mut(pos) &= !TEMP_OBSTR;
    }

    map.tiles
        .into_iter()
        .filter(|tile| tile & ADD_OBSTR != 0)
        .count() as u64
}

fn verify_loop(map: &mut Map) -> bool {
    let mut count = 0;

    let mut traversal = map.new_traversal();
    let is_loop = loop {
        let Some((pos, dir)) = map.traverse_next(&mut traversal) else {
            break false;
        };

        if map.get_and(pos, flag(dir)) {
            break true;
        }
        *map.get_mut(pos) |= flag(dir);

        count += 1;
    };

    let mut traversal = map.new_traversal();
    for _ in 0..count {
        let Some((pos, _)) = map.traverse_next(&mut traversal) else {
            break;
        };

        *map.get_mut(pos) &= !VISIT_FLAGS;
    }

    is_loop
}

fn parse_input(input: &str) -> Map {
    let mut map = Map {
        tiles: Vec::new(),
        dim: (0, 0),
        start_pos: (0, 0),
        start_dir: (-1, 0),
    };

    for (i, line) in input.lines().enumerate() {
        map.dim.0 += 1;

        for (j, tile) in line.chars().enumerate() {
            map.tiles.push(match tile {
                '.' => 0,
                '^' => {
                    map.start_pos = (i, j);
                    0
                }
                '#' => PRE_OBSTR,
                _ => panic!(),
            });
        }
    }

    map.dim.1 = map.tiles.len() / map.dim.0;
    map
}

type Tile = u8;

const PRE_OBSTR: Tile = 1 << 0;
const ADD_OBSTR: Tile = 1 << 1;
const TEMP_OBSTR: Tile = 1 << 2;

const VISITED: Tile = 1 << 3;

const VISIT_DOWN: Tile = 1 << 4;
const VISIT_RIGHT: Tile = 1 << 5;
const VISIT_UP: Tile = 1 << 6;
const VISIT_LEFT: Tile = 1 << 7;

const VISIT_FLAGS: Tile = VISIT_DOWN | VISIT_RIGHT | VISIT_UP | VISIT_LEFT;

#[derive(Clone)]
struct Map {
    tiles: Vec<Tile>,
    dim: (usize, usize),
    start_pos: Pos,
    start_dir: Dir,
}

type Pos = (usize, usize);
type Dir = (isize, isize);

struct Traversal(Option<(Pos, Dir)>);

impl Map {
    fn get(&self, pos: Pos) -> Tile {
        self.tiles[pos.0 * self.dim.1 + pos.1]
    }

    fn get_and(&self, pos: Pos, mask: Tile) -> bool {
        self.get(pos) & mask == mask
    }

    fn get_and_any(&self, pos: Pos, mask: Tile) -> bool {
        self.get(pos) & mask != 0
    }

    fn get_mut(&mut self, pos: Pos) -> &mut Tile {
        &mut self.tiles[pos.0 * self.dim.1 + pos.1]
    }

    fn step(&self, pos: Pos, dir: Dir) -> Option<Pos> {
        let pos = (
            pos.0.checked_add_signed(dir.0)?,
            pos.1.checked_add_signed(dir.1)?,
        );
        (pos.0 < self.dim.0 && pos.1 < self.dim.1).then_some(pos)
    }

    fn new_traversal(&self) -> Traversal {
        Traversal(Some((self.start_pos, self.start_dir)))
    }

    fn traverse_next(&self, traversal: &mut Traversal) -> Option<(Pos, Dir)> {
        let next = traversal.0;

        if let Some((pos, dir)) = next {
            traversal.0 = match self.step(pos, dir) {
                Some(pos) if !self.get_and_any(pos, PRE_OBSTR | TEMP_OBSTR) => Some((pos, dir)),
                Some(_) => Some((pos, rot_right(dir))),
                None => None,
            };
        }

        next
    }
}

fn rot_right(dir: Dir) -> Dir {
    (dir.1, -dir.0)
}

fn flag(dir: Dir) -> Tile {
    match dir {
        (1, 0) => VISIT_DOWN,
        (0, 1) => VISIT_RIGHT,
        (-1, 0) => VISIT_UP,
        (0, -1) => VISIT_LEFT,
        _ => panic!(),
    }
}
