//!
//! Helpers: point, grid, compressor
//!
//! Approach:
//! - parse every tile to Point2D
//! - for each tiles combination, compute rectangle area
//! - finally, select the max
//!
//! Part 2:
//! - generate grid: since 100_000 * 100_000 is huge,
//!   compress the coordinates to avoid storing useless information
//! - draw perimeter (the vertices of the polygon are already ordered)
//! - flood fill (BFS) to detect outside
//! - compute summed area table to quickly check if rectangle is valid
//!

use advent_of_code::common::{cartesian_pairs, read_input, unordered_pairs};
use advent_of_code::compressor::Compressor;
use advent_of_code::grid::Grid;
use advent_of_code::point::Point2D;
use std::cmp::PartialEq;
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

trait Extension {
    fn draw_perimeter(&mut self, vertices: &[Point2D]);
    fn flood_fill(&mut self, start: Point2D);
    fn summed_area_table(&self) -> Grid<u64>;
}

impl Extension for Grid<Cell> {
    fn draw_perimeter(&mut self, vertices: &[Point2D]) {
        let n = vertices.len();
        for i in 0..n {
            let a = vertices[i];
            let b = vertices[(i + 1).rem_euclid(n)];

            let [start_x, end_x, start_y, end_y] = Point2D::bounds(a, b);

            if start_x == end_x {
                for y in start_y..=end_y {
                    self[Point2D::new(start_x, y)] = Cell::Border;
                }
            } else if start_y == end_y {
                for x in start_x..=end_x {
                    self[Point2D::new(x, start_y)] = Cell::Border;
                }
            } else {
                unreachable!();
            }
        }
    }

    fn flood_fill(&mut self, start: Point2D) {
        let mut queue: VecDeque<Point2D> = VecDeque::new();
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

    fn summed_area_table(&self) -> Grid<u64> {
        let mut table: Grid<u64> = Grid::new(self.width + 1, self.height + 1, 0);

        for (x, y) in cartesian_pairs(self.width, self.height) {
            let point = Point2D::new(x as isize, y as isize);

            let value = (self[point] == Cell::Outside) as u64;
            let above = table[Point2D::new(point.x + 1, point.y)];
            let left = table[Point2D::new(point.x, point.y + 1)];
            let above_left = table[point];

            table[Point2D::new(point.x + 1, point.y + 1)] = value + above + left - above_left;
        }

        table
    }
}

fn part1(input: &str) -> isize {
    let tiles: Vec<Point2D> = input.lines().map(Point2D::parse).collect();

    unordered_pairs(tiles.len())
        .map(|(i, j)| Point2D::rectangle_area(tiles[i], tiles[j]))
        .max()
        .unwrap()
}

fn part2(input: &str) -> isize {
    let tiles: Vec<Point2D> = input.lines().map(Point2D::parse).collect();

    let compressor = Compressor::new(&tiles);
    let compressed_tiles: Vec<Point2D> = tiles
        .iter()
        .map(|&tile| compressor.compress(tile))
        .collect();

    let mut compressed_grid: Grid<Cell> = compressor.grid(Cell::Inside);
    compressed_grid.draw_perimeter(&compressed_tiles);
    compressed_grid.flood_fill(Point2D::new(0, 0));
    let table = compressed_grid.summed_area_table();

    unordered_pairs(tiles.len())
        .filter(|&(i, j)| {
            let [start_x, end_x, start_y, end_y] =
                Point2D::bounds(compressed_tiles[i], compressed_tiles[j]);

            let full = table[Point2D::new(end_x + 1, end_y + 1)];
            let above = table[Point2D::new(end_x + 1, start_y + 1)];
            let left = table[Point2D::new(start_x + 1, end_y + 1)];
            let above_left = table[Point2D::new(start_x + 1, start_y + 1)];

            full + above_left - above - left == 0
        })
        .map(|(i, j)| Point2D::rectangle_area(tiles[i], tiles[j]))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

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
