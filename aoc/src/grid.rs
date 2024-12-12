use std::ops::{Index, IndexMut};

pub struct Grid<T> {
    tiles: Vec<T>,
    dim: (usize, usize),
}

impl<T> Grid<T> {
    pub fn from_iter<Row: Iterator<Item = T>>(tiles: impl Iterator<Item = Row>) -> Self {
        let mut grid = Grid {
            tiles: Vec::new(),
            dim: (0, 0),
        };

        for row in tiles {
            for tile in row {
                grid.tiles.push(tile);
            }
            grid.dim.1 += 1;
        }

        grid.dim.0 = grid.tiles.len() / grid.dim.1;
        grid
    }

    pub fn from_str(str: &str, map_tile: fn(char) -> T) -> Self {
        Self::from_iter(str.lines().map(|line| line.chars().map(map_tile)))
    }

    pub fn tiles(&self) -> &[T] {
        &self.tiles
    }

    pub fn tiles_mut(&mut self) -> &mut [T] {
        &mut self.tiles
    }

    pub fn dim(&self) -> (usize, usize) {
        self.dim
    }

    pub fn in_bounds(&self, pos: (usize, usize)) -> bool {
        pos.0 < self.dim.0 && pos.1 < self.dim.1
    }

    pub fn pos_to_idx(&self, pos: (usize, usize)) -> usize {
        pos.0 * self.dim.1 + pos.1
    }

    pub fn get(&self, pos: (usize, usize)) -> Option<&T> {
        let idx = self.pos_to_idx(pos);
        self.in_bounds(pos).then(|| &self.tiles[idx])
    }

    pub fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut T> {
        let idx = self.pos_to_idx(pos);
        self.in_bounds(pos).then(|| &mut self.tiles[idx])
    }

    pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        let dim = self.dim;
        (0..dim.0).flat_map(move |i| (0..dim.1).map(move |j| (i, j)))
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &'_ T)> {
        self.positions().map(|pos| (pos, &self[pos]))
    }

    pub fn translate(
        &self,
        pos: (usize, usize),
        translation: (isize, isize),
    ) -> Option<(usize, usize)> {
        let pos = (
            pos.0.checked_add_signed(translation.0)?,
            pos.1.checked_add_signed(translation.1)?,
        );
        self.in_bounds(pos).then_some(pos)
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

pub const CARDINAL_DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    UP = 0b0001,
    RIGHT = 0b0010,
    DOWN = 0b0100,
    LEFT = 0b1000,
}

impl Direction {
    pub fn to_components(self) -> (isize, isize) {
        match self {
            Self::UP => (-1, 0),
            Self::RIGHT => (0, 1),
            Self::DOWN => (1, 0),
            Self::LEFT => (0, -1),
        }
    }

    pub fn from_components(components: (isize, isize)) -> Self {
        match components {
            (-1, 0) => Self::UP,
            (0, 1) => Self::RIGHT,
            (1, 0) => Self::DOWN,
            (0, -1) => Self::LEFT,
            _ => panic!("bad direction components"),
        }
    }
}
