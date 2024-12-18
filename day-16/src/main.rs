use aoc::{Grid, CARDINAL_DIRS};
use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let (start, end, mut maze) = parse_input(input);
    let mut queue = BinaryHeap::from([Reverse(Reindeer {
        pos: start,
        dir: (0, 1),
        score: 0,
    })]);
    maze[start].kind = Kind::Seen;

    loop {
        let Reindeer { pos, dir, score } = queue.pop().unwrap().0;

        if pos == end {
            break score;
        }

        for (dir, cost) in [(dir, 1), ((-dir.1, dir.0), 1001), ((dir.1, -dir.0), 1001)] {
            let pos = maze.translate(pos, dir).unwrap();

            if maze[pos].kind == Kind::Empty {
                let score = score + cost;
                queue.push(Reverse(Reindeer { pos, dir, score }));
                maze[pos].kind = Kind::Seen;
            }
        }
    }
}

fn two(input: &str) -> u64 {
    let (start, end, mut maze) = parse_input(input);
    let mut queue = BinaryHeap::from([Reverse(Reindeer {
        pos: start,
        dir: (0, 1),
        score: 0,
    })]);

    let mut best = u64::MAX;

    while let Some(Reverse(Reindeer { pos, dir, score })) = queue.pop() {
        if score > best {
            break;
        }

        let dir_i = dir_to_i(dir);
        if score > maze[pos].best[dir_i] {
            continue;
        }
        maze[pos].best[dir_i] = score;

        if pos == end {
            best = best.min(score);
            continue;
        }

        for (dir, cost) in [(dir, 1), ((-dir.1, dir.0), 1001), ((dir.1, -dir.0), 1001)] {
            let pos = maze.translate(pos, dir).unwrap();
            let dir_i = dir_to_i(dir);
            let score = score + cost;

            if maze[pos].kind != Kind::Wall && score <= maze[pos].best[dir_i] {
                if maze[pos].prev[(dir_i + 2) % 4] && score < 30_000 {
                    dbg!(
                        pos,
                        dir_i,
                        score,
                        maze[pos].best[dir_i],
                        maze[pos].best[(dir_i + 2) % 4]
                    );
                }
                if maze[pos].prev[(dir_i + 2) % 4]
                    && score > maze[pos].best[(dir_i + 2) % 4]
                    && score < maze[pos].best[(dir_i + 2) % 4] + 1000
                {
                    continue;
                }

                queue.push(Reverse(Reindeer { pos, dir, score }));
                // queue.push(Reverse(Reindeer {
                //     pos,
                //     dir: (-dir.1, dir.0),
                //     score: score + 1000,
                // }));
                // queue.push(Reverse(Reindeer {
                //     pos,
                //     dir: (dir.1, -dir.0),
                //     score: score + 1000,
                // }));
                // queue.push(Reverse(Reindeer {
                //     pos,
                //     dir: (-dir.0, -dir.1),
                //     score: score + 2000,
                // }));
                maze[pos].best[dir_i] = score;
                maze[pos].best[(dir_i + 1) % 4] = maze[pos].best[(dir_i + 1) % 4].min(score + 1000);
                maze[pos].best[(dir_i + 2) % 4] = maze[pos].best[(dir_i + 2) % 4].min(score + 2000);
                maze[pos].best[(dir_i + 3) % 4] = maze[pos].best[(dir_i + 3) % 4].min(score + 1000);
                maze[pos].prev[dir_i] = true;
            }
        }
    }

    maze[end].kind = Kind::Seen;
    let count = count_best(end, start, &mut maze);

    for (pos, tile) in maze.iter() {
        match tile.kind {
            Kind::Wall => print!("."),
            Kind::Empty => print!(" "),
            Kind::Seen => print!("O"),
            // _ => print!("{}", tile.prev.iter().filter(|s| **s).count()),
        }
        if pos.1 + 1 == maze.dim().1 {
            println!()
        }
    }

    for (pos, tile) in maze.iter() {
        match tile.kind {
            Kind::Wall => print!(" "),
            // Kind::Empty => print!(" "),
            // Kind::Seen => print!("O"),
            _ => print!("{}", tile.prev.iter().filter(|s| **s).count()),
        }
        if pos.1 + 1 == maze.dim().1 {
            println!()
        }
    }

    dbg!(best);
    count + 1
}

fn count_best(pos: (usize, usize), start: (usize, usize), maze: &mut Grid<Tile>) -> u64 {
    if pos == start {
        return 0;
    }

    let mut count = 0;

    for dir in CARDINAL_DIRS {
        if maze[pos].prev[dir_to_i(dir)] {
            let pos = maze.translate(pos, (-dir.0, -dir.1)).unwrap();
            if maze[pos].kind == Kind::Empty {
                maze[pos].kind = Kind::Seen;
                count += count_best(pos, start, maze) + 1;
            }
        }
    }

    count
}

struct Tile {
    kind: Kind,
    best: [u64; 4],
    prev: [bool; 4],
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            kind: Kind::Empty,
            best: [u64::MAX; 4],
            prev: [false; 4],
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Wall,
    Empty,
    Seen,
}

#[derive(Clone, Copy, Eq, Ord)]
struct Reindeer {
    pos: (usize, usize),
    dir: (isize, isize),
    score: u64,
}

impl PartialEq for Reindeer {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

fn dir_to_i(dir: (isize, isize)) -> usize {
    match dir {
        (-1, 0) => 0,
        (0, 1) => 1,
        (1, 0) => 2,
        (0, -1) => 3,
        _ => panic!(),
    }
}

fn parse_input(input: &str) -> ((usize, usize), (usize, usize), Grid<Tile>) {
    let mut maze = Grid::builder();
    let (mut start, mut end) = (None, None);

    for line in input.lines() {
        for char in line.chars() {
            maze.tile(Tile {
                kind: match char {
                    '#' => Kind::Wall,
                    '.' => Kind::Empty,
                    'S' => {
                        start = Some(maze.pos());
                        Kind::Empty
                    }
                    'E' => {
                        end = Some(maze.pos());
                        Kind::Empty
                    }
                    _ => panic!(),
                },
                ..Default::default()
            });
        }
        maze.finish_line();
    }

    (start.unwrap(), end.unwrap(), maze.finish_grid())
}
