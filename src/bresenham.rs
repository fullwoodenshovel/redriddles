//! This is largely ripped out of the library, but some changes made for my use case
use core::iter::Iterator;

/// Convenient typedef for two integers
pub type Point = [i16; 2];

/// Line-drawing iterator
pub struct Bresenham {
    x: i16,
    y: i16,
    dx: i16,
    dy: i16,
    x1: i16,
    diff: i16,
    octant: Octant,
}

struct Octant(u8);

impl Octant {
    /// adapted from http://codereview.stackexchange.com/a/95551
    #[inline]
    fn from_points(start: Point, end: Point) -> Octant {
        let mut dx = end[0] - start[0];
        let mut dy = end[1] - start[1];

        let mut octant = 0;

        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }

        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2
        }

        if dx < dy {
            octant += 1
        }

        Octant(octant)
    }

    #[inline]
    fn to_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => [p[0], p[1]],
            1 => [p[1], p[0]],
            2 => [p[1], -p[0]],
            3 => [-p[0], p[1]],
            4 => [-p[0], -p[1]],
            5 => [-p[1], -p[0]],
            6 => [-p[1], p[0]],
            7 => [p[0], -p[1]],
            _ => unreachable!(),
        }
    }

    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => [p[0], p[1]],
            1 => [p[1], p[0]],
            2 => [-p[1], p[0]],
            3 => [-p[0], p[1]],
            4 => [-p[0], -p[1]],
            5 => [-p[1], -p[0]],
            6 => [p[1], -p[0]],
            7 => [p[0], -p[1]],
            _ => unreachable!(),
        }
    }
}

impl Bresenham {
    /// Creates a new iterator. Yields intermediate points
    /// between `start` and `end`. Includes both.
    #[inline]
    pub fn new(start: Point, end: Point) -> Bresenham {
        let octant = Octant::from_points(start, end);

        let start = octant.to_octant0(start);
        let end = octant.to_octant0(end);

        let dx = end[0] - start[0];
        let dy = end[1] - start[1];

        Bresenham {
            x: start[0],
            y: start[1],
            dx,
            dy,
            x1: end[0],
            diff: 2 * dy - dx,
            octant,
        }
    }
}

impl Iterator for Bresenham {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.x1 {
            return None;
        }

        let p = [self.x, self.y];

        if self.diff > 0 {
            self.y += 1;
            self.diff -= 2 * self.dx;
        }

        self.diff += 2 * self.dy;
        self.x += 1;

        Some(self.octant.from_octant0(p))
    }
}