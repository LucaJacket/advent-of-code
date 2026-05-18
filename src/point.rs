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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

impl Point2D {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn parse(value: &str) -> Self {
        let mut coords = value.splitn(2, ',');
        let x = coords.next().unwrap().parse::<isize>().unwrap();
        let y = coords.next().unwrap().parse::<isize>().unwrap();
        Self { x, y }
    }

    pub fn bounds(a: Self, b: Self) -> [isize; 4] {
        [a.x.min(b.x), a.x.max(b.x), a.y.min(b.y), a.y.max(b.y)]
    }

    pub fn rectangle_area(a: Self, b: Self) -> isize {
        let base = (a.x - b.x).abs() + 1;
        let height = (a.y - b.y).abs() + 1;
        base * height
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point3D {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn parse(value: &str) -> Self {
        let mut coords = value.splitn(3, ',');
        let x = coords.next().unwrap().parse::<isize>().unwrap();
        let y = coords.next().unwrap().parse::<isize>().unwrap();
        let z = coords.next().unwrap().parse::<isize>().unwrap();
        Self { x, y, z }
    }

    pub fn distance_squared(a: Self, b: Self) -> isize {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dz = a.z - b.z;
        dx * dx + dy * dy + dz * dz
    }
}
