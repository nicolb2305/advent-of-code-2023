use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offset {
    pub dx: isize,
    pub dy: isize,
}

impl Offset {
    pub fn new(dx: isize, dy: isize) -> Self {
        Self { dx, dy }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Sub for Coordinate {
    type Output = Offset;

    fn sub(self, rhs: Self) -> Self::Output {
        let x1: isize = self.x.try_into().unwrap();
        let x2: isize = rhs.x.try_into().unwrap();
        let y1: isize = self.y.try_into().unwrap();
        let y2: isize = rhs.y.try_into().unwrap();
        Offset::new(x1 - x2, y1 - y2)
    }
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn offset(self, offset: Offset) -> Option<Self> {
        let x = if offset.dx >= 0 {
            self.x.checked_add(offset.dx.try_into().ok()?)?
        } else {
            self.x.checked_sub(offset.dx.abs().try_into().ok()?)?
        };
        let y = if offset.dy >= 0 {
            self.y.checked_add(offset.dy.try_into().ok()?)?
        } else {
            self.y.checked_sub(offset.dy.abs().try_into().ok()?)?
        };

        Some(Self { x, y })
    }

    pub fn iter(self, diagonal: bool) -> CoordinateIterator {
        CoordinateIterator {
            center: self,
            i: 0,
            diagonal,
        }
    }

    pub fn manhatten_distance(self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub struct CoordinateIterator {
    center: Coordinate,
    i: usize,
    diagonal: bool,
}

impl Iterator for CoordinateIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.diagonal && self.i >= 4 {
            return None;
        }

        let offset = match self.i {
            0 => Offset::new(-1, 0),
            1 => Offset::new(0, -1),
            2 => Offset::new(1, 0),
            3 => Offset::new(0, 1),
            4 => Offset::new(-1, -1),
            5 => Offset::new(-1, 1),
            6 => Offset::new(1, -1),
            7 => Offset::new(1, 1),
            _ => return None,
        };

        self.i += 1;
        if let Some(new) = self.center.offset(offset) {
            Some(new)
        } else {
            self.next()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn get(&self, coordinate: Coordinate) -> Option<&T> {
        self.0
            .get(coordinate.y)
            .and_then(|row| row.get(coordinate.x))
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, item: &T) -> Option<Coordinate> {
        self.0
            .iter()
            .enumerate()
            .find_map(|(y, row)| row.iter().position(|x| x == item).map(|x| (x, y)))
            .map(Into::into)
    }
}
