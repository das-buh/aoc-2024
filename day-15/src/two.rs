use aoc::Grid;

pub fn run(input: &str) -> u64 {
    let (mut pos, mut warehouse, moves) = parse_input(input);

    for dir in moves {
        let can_move = match dir {
            (dir, 0) => try_move_vertical(pos, dir, &mut warehouse),
            (0, dir) => try_move_horizontal(pos, dir, &mut warehouse),
            _ => panic!(),
        };
        if can_move {
            pos = warehouse.translate(pos, dir).unwrap();
        }
    }

    warehouse
        .iter()
        .filter(|(_, tile)| **tile == Tile::BoxLeft)
        .map(|(pos, _)| 100 * pos.0 as u64 + pos.1 as u64)
        .sum()
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    BoxLeft,
    BoxRight,
    Wall,
    Empty,
}

type Moves = Vec<(isize, isize)>;

fn parse_input(input: &str) -> ((usize, usize), Grid<Tile>, Moves) {
    let mut lines = input.lines();

    let mut start = None;
    let mut warehouse = Grid::builder();

    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        for char in line.chars() {
            let (left, right) = match char {
                'O' => (Tile::BoxLeft, Tile::BoxRight),
                '#' => (Tile::Wall, Tile::Wall),
                '.' => (Tile::Empty, Tile::Empty),
                '@' => {
                    start = Some(warehouse.pos());
                    (Tile::Empty, Tile::Empty)
                }
                _ => panic!(),
            };
            warehouse.tile(left);
            warehouse.tile(right);
        }
        warehouse.finish_line();
    }

    let moves = lines
        .flat_map(|line| line.chars())
        .filter(|char| *char != '\n')
        .map(|char| match char {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => panic!(),
        })
        .collect();

    (start.unwrap(), warehouse.finish_grid(), moves)
}

fn try_move_horizontal(pos: (usize, usize), dir: isize, warehouse: &mut Grid<Tile>) -> bool {
    let tile = warehouse[pos];
    let new_pos = warehouse.translate(pos, (0, dir)).unwrap();

    let can_move = match warehouse[new_pos] {
        Tile::BoxLeft | Tile::BoxRight => try_move_horizontal(new_pos, dir, warehouse),
        Tile::Wall => false,
        Tile::Empty => true,
    };

    if can_move {
        warehouse[pos] = Tile::Empty;
        warehouse[new_pos] = tile;
    }

    can_move
}

fn try_move_vertical(pos: (usize, usize), dir: isize, warehouse: &mut Grid<Tile>) -> bool {
    let can_move = can_move_vertical(pos, dir, warehouse);
    if can_move {
        move_vertical(pos, dir, warehouse);
    }
    can_move
}

fn can_move_vertical(pos: (usize, usize), dir: isize, warehouse: &mut Grid<Tile>) -> bool {
    let new_pos = warehouse.translate(pos, (dir, 0)).unwrap();

    match warehouse[new_pos] {
        Tile::BoxLeft => {
            let right = warehouse.translate(new_pos, (0, 1)).unwrap();
            can_move_vertical(new_pos, dir, warehouse) && can_move_vertical(right, dir, warehouse)
        }
        Tile::BoxRight => {
            let left = warehouse.translate(new_pos, (0, -1)).unwrap();
            can_move_vertical(left, dir, warehouse) && can_move_vertical(new_pos, dir, warehouse)
        }
        Tile::Wall => false,
        Tile::Empty => true,
    }
}

fn move_vertical(pos: (usize, usize), dir: isize, warehouse: &mut Grid<Tile>) {
    let tile = warehouse[pos];
    let new_pos = warehouse.translate(pos, (dir, 0)).unwrap();

    match warehouse[new_pos] {
        Tile::BoxLeft => {
            let right = warehouse.translate(new_pos, (0, 1)).unwrap();
            move_vertical(new_pos, dir, warehouse);
            move_vertical(right, dir, warehouse);
        }
        Tile::BoxRight => {
            let left = warehouse.translate(new_pos, (0, -1)).unwrap();
            move_vertical(left, dir, warehouse);
            move_vertical(new_pos, dir, warehouse);
        }
        _ => {}
    }

    warehouse[pos] = Tile::Empty;
    warehouse[new_pos] = tile;
}
