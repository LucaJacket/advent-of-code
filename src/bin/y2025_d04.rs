//!
//! Helpers: grid, point
//!
//! Approach:
//! - parse grid
//! - for every position, check if there are less than 4 neighbors
//!
//! Part 2:
//! - collect all the accessible rolls, then remove them
//! - repeat until there are no rolls to remove
//!

use advent_of_code::common::{cartesian_pairs, read_input};
use advent_of_code::grid::Grid;
use advent_of_code::point::Point2D;
use std::iter::repeat_with;

fn main() {
    let input = read_input(2025, 4);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

const ROLL: char = '@';
const EMPTY: char = '.';
const THRESHOLD: usize = 4;

trait Extension {
    fn accessible(&self) -> impl Iterator<Item = Point2D>;
    fn remove(&mut self) -> usize;
}

impl Extension for Grid<char> {
    fn accessible(&self) -> impl Iterator<Item = Point2D> {
        let has_few_neighbors = |point: Point2D| {
            self.neighbors8(point)
                .filter(|&neighbor| self[neighbor] == ROLL)
                .take(THRESHOLD)
                .count()
                < THRESHOLD
        };
        cartesian_pairs(self.width, self.height)
            .map(|(x, y)| Point2D::new(x as isize, y as isize))
            .filter(move |&point| self[point] == ROLL && has_few_neighbors(point))
    }

    fn remove(&mut self) -> usize {
        let accessible: Vec<Point2D> = self.accessible().collect();
        for &point in &accessible {
            self[point] = EMPTY;
        }
        accessible.len()
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::parse(input);

    grid.accessible().count()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::parse(input);

    repeat_with(move || grid.remove())
        .take_while(|&removed| removed > 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 43);
    }
}
