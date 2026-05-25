use crate::point::{DIRS4, DIRS8, Point2D};
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

    fn neighbors(
        &self,
        point: Point2D,
        dirs: impl IntoIterator<Item = Point2D>,
    ) -> impl Iterator<Item = Point2D> {
        let (w, h) = (self.width as isize, self.height as isize);
        dirs.into_iter()
            .map(move |dir| dir + point)
            .filter(move |&neighbor| (0..w).contains(&neighbor.x) && (0..h).contains(&neighbor.y))
    }

    pub fn neighbors4(&self, point: Point2D) -> impl Iterator<Item = Point2D> {
        self.neighbors(point, DIRS4)
    }

    pub fn neighbors8(&self, point: Point2D) -> impl Iterator<Item = Point2D> {
        self.neighbors(point, DIRS8)
    }

    pub fn row(&self, row: isize) -> Vec<T>
    where
        T: Clone,
    {
        let w = self.width as isize;
        (0..w)
            .map(move |x| self[Point2D::new(x, row)].clone())
            .collect()
    }

    pub fn col(&self, col: isize) -> Vec<T>
    where
        T: Clone,
    {
        let h = self.height as isize;
        (0..h)
            .map(move |y| self[Point2D::new(col, y)].clone())
            .collect()
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

impl Grid<char> {
    pub fn parse(value: &str) -> Self {
        let raw: Vec<Vec<char>> = value.lines().map(|line| line.chars().collect()).collect();

        Self {
            data: raw.concat(),
            width: raw[0].len(),
            height: raw.len(),
        }
    }
}
