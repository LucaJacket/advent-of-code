use crate::point::{Point2D, DIRS4, DIRS8};
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    pub fn is_in_bounds(&self, point: Point2D) -> bool {
        (0..self.width as isize).contains(&point.x) && (0..self.height as isize).contains(&point.y)
    }

    fn neighbors(
        &self,
        point: Point2D,
        dirs: impl IntoIterator<Item = Point2D>,
    ) -> impl Iterator<Item = Point2D> {
        dirs.into_iter()
            .map(move |dir| dir + point)
            .filter(|&neighbor| self.is_in_bounds(neighbor))
    }

    pub fn neighbors4(&self, point: Point2D) -> impl Iterator<Item = Point2D> {
        self.neighbors(point, DIRS4)
    }

    pub fn neighbors8(&self, point: Point2D) -> impl Iterator<Item = Point2D> {
        self.neighbors(point, DIRS8)
    }
}

impl<T> Index<Point2D> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point2D) -> &Self::Output {
        &self.data[index.y as usize * self.width + index.x as usize]
    }
}

impl<T> IndexMut<Point2D> for Grid<T> {
    fn index_mut(&mut self, index: Point2D) -> &mut Self::Output {
        &mut self.data[index.y as usize * self.width + index.x as usize]
    }
}

impl Grid<u8> {
    pub fn parse(value: &str) -> Self {
        let raw = value.lines().map(str::as_bytes).collect::<Vec<_>>();

        let data = raw.concat();
        let width = raw[0].len();
        let height = raw.len();

        Self {
            data,
            width,
            height,
        }
    }
}
