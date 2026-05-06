use crate::point::{DIRS4, DIRS8, Point2D};
use std::ops::{Index, IndexMut};

pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: isize,
    pub height: isize,
}

impl<T> Grid<T> {
    pub fn new(width: isize, height: isize, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn coords(&self) -> impl Iterator<Item = Point2D> {
        (0..self.height * self.width).map(|i| Point2D {
            x: i.rem_euclid(self.width),
            y: i.div_euclid(self.width),
        })
    }

    pub fn is_in_bounds(&self, point: Point2D) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    fn neighbors(&self, point: Point2D, dirs: &[Point2D]) -> impl Iterator<Item = Point2D> {
        dirs.iter()
            .map(move |&dir| dir + point)
            .filter(|&neighbor| self.is_in_bounds(neighbor))
    }

    pub fn neighbors4(&self, point: Point2D) -> impl Iterator<Item = Point2D> {
        self.neighbors(point, &DIRS4)
    }

    pub fn neighbors8(&self, point: Point2D) -> impl Iterator<Item = Point2D> {
        self.neighbors(point, &DIRS8)
    }
}

impl<T> Index<Point2D> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point2D) -> &Self::Output {
        &self.data[(index.y * self.width + index.x) as usize]
    }
}

impl<T> IndexMut<Point2D> for Grid<T> {
    fn index_mut(&mut self, index: Point2D) -> &mut Self::Output {
        &mut self.data[(index.y * self.width + index.x) as usize]
    }
}

impl Grid<u8> {
    pub fn parse(value: &str) -> Self {
        let raw = value.lines().map(str::as_bytes).collect::<Vec<_>>();

        let data = raw.concat();
        let width = raw[0].len() as isize;
        let height = raw.len() as isize;

        Self {
            data,
            width,
            height,
        }
    }
}
