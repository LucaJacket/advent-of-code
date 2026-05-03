use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const NORTH: Point2D = Point2D { x: 0, y: -1 };
pub const NORTH_EAST: Point2D = Point2D { x: 1, y: -1 };
pub const EAST: Point2D = Point2D { x: 1, y: 0 };
pub const SOUTH_EAST: Point2D = Point2D { x: 1, y: 1 };
pub const SOUTH: Point2D = Point2D { x: 0, y: 1 };
pub const SOUTH_WEST: Point2D = Point2D { x: -1, y: 1 };
pub const WEST: Point2D = Point2D { x: -1, y: 0 };
pub const NORTH_WEST: Point2D = Point2D { x: -1, y: -1 };

pub const DIRS4: [Point2D; 4] = [NORTH, EAST, SOUTH, WEST];
pub const DIRS8: [Point2D; 8] = [
    NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
];

#[derive(Clone, Copy)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

impl Point2D {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point2D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Point2D {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
