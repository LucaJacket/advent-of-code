use crate::point::{Point2D, DIRS4, DIRS8};
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>, width: usize, height: usize) -> Grid<T> {
        Self {
            data,
            width,
            height,
        }
    }

    pub fn fill(default: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    pub fn find(&self, target: T) -> Option<Point2D>
    where
        T: PartialEq,
    {
        let (w, h) = (self.width as isize, self.height as isize);
        (0..w)
            .flat_map(move |x| (0..h).map(move |y| Point2D::new(x, y)))
            .find(|&point| self[point] == target)
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

    pub fn iter_row(&self, row: isize) -> impl DoubleEndedIterator<Item = &T> {
        let w = self.width as isize;
        (0..w).map(move |x| &self[Point2D::new(x, row)])
    }

    pub fn iter_col(&self, col: isize) -> impl DoubleEndedIterator<Item = &T> {
        let h = self.height as isize;
        (0..h).map(move |y| &self[Point2D::new(col, y)])
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
    pub fn from_chars(value: &str) -> Self {
        let data: Vec<char> = value.lines().flat_map(|line| line.chars()).collect();
        let width = value
            .lines()
            .next()
            .map(|line| line.chars().count())
            .unwrap_or(0);
        let height = if width == 0 { 0 } else { data.len() / width };

        Self::new(data, width, height)
    }
}

impl<'a> Grid<&'a str> {
    pub fn split_whitespace(value: &'a str) -> Self {
        let data: Vec<&str> = value
            .lines()
            .flat_map(|line| line.split_whitespace())
            .collect();
        let width = value
            .lines()
            .next()
            .map(|line| line.split_whitespace().count())
            .unwrap_or(0);
        let height = if width == 0 { 0 } else { data.len() / width };

        Self::new(data, width, height)
    }
}
