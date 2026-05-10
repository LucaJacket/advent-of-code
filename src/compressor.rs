use crate::grid::Grid;
use crate::point::Point2D;

pub struct Compressor {
    pub unique_x: Vec<isize>,
    pub unique_y: Vec<isize>,
}

impl Compressor {
    pub fn new(points: &[Point2D]) -> Self {
        let mut unique_x = points
            .iter()
            .flat_map(|point| [point.x - 1, point.x, point.x + 1])
            .collect::<Vec<_>>();
        unique_x.sort_unstable();
        unique_x.dedup();

        let mut unique_y = points
            .iter()
            .flat_map(|point| [point.y - 1, point.y, point.y + 1])
            .collect::<Vec<_>>();
        unique_y.sort_unstable();
        unique_y.dedup();

        Self { unique_x, unique_y }
    }

    pub fn compress(&self, point: Point2D) -> Point2D {
        Point2D::new(
            self.unique_x.binary_search(&point.x).unwrap() as isize,
            self.unique_y.binary_search(&point.y).unwrap() as isize,
        )
    }

    pub fn grid<T>(&self, default: T) -> Grid<T>
    where
        T: Clone,
    {
        Grid::new(
            self.unique_x.len() as isize,
            self.unique_y.len() as isize,
            default,
        )
    }
}
