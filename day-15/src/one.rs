use aoc::Grid;

pub fn run(input: &str) -> u64 {
    let (mut pos, mut warehouse, moves) = parse_input(input);

    for dir in moves {
        if try_move(pos, dir, &mut warehouse) {
            pos = warehouse.translate(pos, dir).unwrap();
        }
    }

    warehouse
        .iter()
        .filter(|(_, tile)| **tile == Tile::Box)
        .map(|(pos, _)| 100 * pos.0 as u64 + pos.1 as u64)
        .sum()
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Box,
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
            warehouse.tile(match char {
                'O' => Tile::Box,
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                '@' => {
                    start = Some(warehouse.pos());
                    Tile::Empty
                }
                _ => panic!(),
            });
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

fn try_move(pos: (usize, usize), dir: (isize, isize), warehouse: &mut Grid<Tile>) -> bool {
    let tile = warehouse[pos];
    let new_pos = warehouse.translate(pos, dir).unwrap();

    let can_move = match warehouse[new_pos] {
        Tile::Box => try_move(new_pos, dir, warehouse),
        Tile::Wall => false,
        Tile::Empty => true,
    };

    if can_move {
        warehouse[pos] = Tile::Empty;
        warehouse[new_pos] = tile;
    }

    can_move
}
