use aoc::{Grid, Slab};
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
        id: 0,
    })]);
    maze[start].kind = Kind::Seen;

    loop {
        let Reindeer {
            pos, dir, score, ..
        } = queue.pop().unwrap().0;

        if pos == end {
            break score;
        }

        for (dir, cost) in [(dir, 1), ((-dir.1, dir.0), 1001), ((dir.1, -dir.0), 1001)] {
            let pos = maze.translate(pos, dir).unwrap();

            if maze[pos].kind == Kind::Empty {
                queue.push(Reverse(Reindeer {
                    pos,
                    dir,
                    score: score + cost,
                    id: 0,
                }));
                maze[pos].kind = Kind::Seen;
            }
        }
    }
}

fn two(input: &str) -> u64 {
    let (start, end, mut maze) = parse_input(input);
    let mut prevs = Slab::new();
    let mut queue = BinaryHeap::from([Reverse(Reindeer {
        pos: start,
        dir: (0, 1),
        score: 0,
        id: prevs.insert((start, (0, 1), usize::MAX)),
    })]);

    let mut best = u64::MAX;
    let mut count = 0;

    while let Some(Reverse(Reindeer {
        pos,
        dir,
        score,
        id,
    })) = queue.pop()
    {
        if score > best {
            break;
        }

        if pos == end {
            best = best.min(score);

            let mut curr = id;
            while let Some((pos, _, prev)) = prevs.get(curr) {
                if maze[*pos].kind == Kind::Empty {
                    maze[*pos].kind = Kind::Seen;
                    count += 1;
                }
                curr = *prev;
            }

            continue;
        }

        for (dir, cost) in [(dir, 1), ((-dir.1, dir.0), 1001), ((dir.1, -dir.0), 1001)] {
            let pos = maze.translate(pos, dir).unwrap();
            let dir_i = dir_to_i(dir);
            let score = score + cost;

            if maze[pos].kind != Kind::Wall && score <= maze[pos].best[dir_i] {
                queue.push(Reverse(Reindeer {
                    pos,
                    dir,
                    score,
                    id: prevs.insert((pos, dir, id)),
                }));
                maze[pos].best[dir_i] = score;
            }
        }
    }

    count
}

struct Tile {
    kind: Kind,
    best: [u64; 4],
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Wall,
    Empty,
    Seen,
}

#[derive(Clone, Copy, Eq)]
struct Reindeer {
    pos: (usize, usize),
    dir: (isize, isize),
    score: u64,
    id: usize,
}

impl PartialEq for Reindeer {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
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
                best: [u64::MAX; 4],
            });
        }
        maze.finish_line();
    }

    (start.unwrap(), end.unwrap(), maze.finish_grid())
}
