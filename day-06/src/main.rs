fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let mut map = parse_input(input);
    let (mut pos, mut dir) = (map.start_pos, map.start_dir);

    loop {
        *map.get_mut(pos) |= VISITED;

        let Some(new) = map.step(pos, dir) else {
            break;
        };

        if map.get(new) & PRE_OBSTR != 0 {
            dir = rot_right(dir);
        } else {
            pos = new;
        }
    }

    map.tiles
        .into_iter()
        .filter(|tile| tile & VISITED != 0)
        .count() as u64
}

fn two(input: &str) -> u64 {
    let mut map = parse_input(input);
    let (mut pos, mut dir) = (map.start_pos, map.start_dir);
    let mut changes = Vec::new();

    loop {
        if map.get_and(pos, flag(dir)) {
            break;
        }
        *map.get_mut(pos) |= flag(dir);

        *map.get_mut(pos) |= TEMP_OBSTR;
        if verify_loop(&mut map, &mut changes) {
            *map.get_mut(pos) |= ADD_OBSTR;
        }
        *map.get_mut(pos) &= !TEMP_OBSTR;

        match map.step(pos, dir) {
            Some(new) if !map.get_and_any(new, PRE_OBSTR) => pos = new,
            Some(_) => dir = rot_right(dir),
            None => break,
        }
    }

    map.tiles
        .into_iter()
        .filter(|tile| tile & ADD_OBSTR != 0)
        .count() as u64
}

fn verify_loop(map: &mut Map, changes: &mut Vec<(Pos, Tile)>) -> bool {
    let (mut pos, mut dir) = (map.start_pos, map.start_dir);

    let is_loop = loop {
        if map.get_and(pos, verify_flag(dir)) {
            break true;
        }
        if !map.get_and(pos, verify_flag(dir)) {
            *map.get_mut(pos) |= verify_flag(dir);
            changes.push((pos, verify_flag(dir)));
        }

        match map.step(pos, dir) {
            Some(new) if !map.get_and_any(new, PRE_OBSTR | TEMP_OBSTR) => pos = new,
            Some(_) => dir = rot_right(dir),
            None => break false,
        }
    };

    for (pos, flag) in changes.drain(..) {
        *map.get_mut(pos) &= !flag;
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

type Tile = u16;

const PRE_OBSTR: Tile = 1 << 0;
const ADD_OBSTR: Tile = 1 << 1;
const TEMP_OBSTR: Tile = 1 << 2;
const VISITED: Tile = 1 << 3;

const VISIT_DOWN: Tile = 1 << 4;
const VISIT_RIGHT: Tile = 1 << 5;
const VISIT_UP: Tile = 1 << 6;
const VISIT_LEFT: Tile = 1 << 7;

const VERIFY_DOWN: Tile = 1 << 8;
const VERIFY_RIGHT: Tile = 1 << 9;
const VERIFY_UP: Tile = 1 << 10;
const VERIFY_LEFT: Tile = 1 << 11;

#[derive(Clone)]
struct Map {
    tiles: Vec<Tile>,
    dim: (usize, usize),
    start_pos: Pos,
    start_dir: Dir,
}

type Pos = (usize, usize);
type Dir = (isize, isize);

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

fn verify_flag(dir: Dir) -> Tile {
    match dir {
        (1, 0) => VERIFY_DOWN,
        (0, 1) => VERIFY_RIGHT,
        (-1, 0) => VERIFY_UP,
        (0, -1) => VERIFY_LEFT,
        _ => panic!(),
    }
}
