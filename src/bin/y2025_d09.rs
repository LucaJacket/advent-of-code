//
// Helpers: point, grid
//
// Approach:
// - parse every tile to Point2D
// - for each tiles combination, compute rectangle area
// - finally, select the max
//
// Part 2:
// - generate grid: since 100_000 * 100_000 is huge,
//   compress the coordinates to avoid storing useless information
// - draw perimeter (the vertices of the polygon are already ordered)
// - flood fill (BFS) to detect outside
// - compute summed area table to quickly check if rectangle is valid
// - to speed up: compute all areas, then sort by decreasing area, then return the first valid one
//

use advent_of_code::common::{combinations2, read_input};
use advent_of_code::grid::Grid;
use advent_of_code::point::Point2D;
use std::cmp::{PartialEq, Reverse};
use std::collections::VecDeque;

fn main() {
    let input = read_input(2025, 9);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Inside,
    Border,
    Outside,
}

struct Compressor {
    unique_x: Vec<isize>,
    unique_y: Vec<isize>,
}

impl Compressor {
    fn new(points: &[Point2D]) -> Self {
        let mut unique_x: Vec<_> = points
            .iter()
            .flat_map(|point| [point.x - 1, point.x, point.x + 1])
            .collect();
        unique_x.sort_unstable();
        unique_x.dedup();

        let mut unique_y: Vec<_> = points
            .iter()
            .flat_map(|point| [point.y - 1, point.y, point.y + 1])
            .collect();
        unique_y.sort_unstable();
        unique_y.dedup();

        Self { unique_x, unique_y }
    }

    fn compress(&self, point: Point2D) -> Point2D {
        Point2D::new(
            self.unique_x.binary_search(&point.x).unwrap() as isize,
            self.unique_y.binary_search(&point.y).unwrap() as isize,
        )
    }

    fn grid(&self) -> Grid<Cell> {
        Grid::new(
            self.unique_x.len() as isize,
            self.unique_y.len() as isize,
            Cell::Inside,
        )
    }
}

trait Extension {
    fn perimeter(&mut self, vertices: &[Point2D]);
    fn flood_fill(&mut self, start: Point2D);
    fn summed_area(&self) -> Grid<u64>;
}

impl Extension for Grid<Cell> {
    fn perimeter(&mut self, vertices: &[Point2D]) {
        for i in 0..vertices.len() {
            let a = vertices[i];
            let b = vertices[(i + 1).rem_euclid(vertices.len())];
            let [start_x, end_x, start_y, end_y] = Point2D::bounds(a, b);
            for y in start_y..=end_y {
                self[Point2D::new(start_x, y)] = Cell::Border;
            }
            for x in start_x..=end_x {
                self[Point2D::new(x, start_y)] = Cell::Border;
            }
        }
    }

    fn flood_fill(&mut self, start: Point2D) {
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(point) = queue.pop_front() {
            if self[point] != Cell::Inside {
                continue;
            }
            self[point] = Cell::Outside;
            for neighbor in self.neighbors4(point) {
                queue.push_back(neighbor);
            }
        }
    }

    fn summed_area(&self) -> Grid<u64> {
        let mut table = Grid::new(self.width + 1, self.height + 1, 0);
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = (self[Point2D::new(x, y)] == Cell::Outside) as u64;
                let top = table[Point2D::new(x + 1, y)];
                let left = table[Point2D::new(x, y + 1)];
                let top_left = table[Point2D::new(x, y)];
                table[Point2D::new(x + 1, y + 1)] = cell + top + left - top_left;
            }
        }
        table
    }
}

fn part1(input: &str) -> isize {
    let tiles: Vec<_> = input.lines().map(Point2D::parse).collect();
    combinations2(tiles.len())
        .map(|(i, j)| Point2D::rectangle_area(tiles[i], tiles[j]))
        .max()
        .expect("no tiles")
}

fn part2(input: &str) -> isize {
    let tiles: Vec<_> = input.lines().map(Point2D::parse).collect();

    let compressor = Compressor::new(&tiles);
    let compressed_tiles: Vec<_> = tiles
        .iter()
        .map(|&tile| compressor.compress(tile))
        .collect();

    let mut grid = compressor.grid();
    grid.perimeter(&compressed_tiles);
    grid.flood_fill(Point2D::new(0, 0));
    let table = grid.summed_area();

    let mut areas: Vec<_> = combinations2(tiles.len())
        .map(|(i, j)| (i, j, Point2D::rectangle_area(tiles[i], tiles[j])))
        .collect();
    areas.sort_unstable_by_key(|&(_, _, area)| Reverse(area));
    areas
        .into_iter()
        .find(|&(i, j, _)| {
            let [start_x, end_x, start_y, end_y] =
                Point2D::bounds(compressed_tiles[i], compressed_tiles[j]);
            let full = table[Point2D::new(end_x + 1, end_y + 1)];
            let top = table[Point2D::new(end_x + 1, start_y + 1)];
            let left = table[Point2D::new(start_x + 1, end_y + 1)];
            let top_left = table[Point2D::new(start_x + 1, start_y + 1)];
            full + top_left - top - left == 0
        })
        .map(|(_, _, area)| area)
        .expect("no tiles")
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 50);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 24);
    }
}
