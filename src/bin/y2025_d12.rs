//!
//! Helpers: dancing_links_x
//!
//! Approach:
//! - check if region is large enough to fit all presents without interlocking
//! - check if region is too small to fit all presents even with perfect interlocking
//! - if neither of the previous works, try to fit the presents manually:
//!   variant of Knuth's Algorithm X + Dancing Links
//!   2 types of columns: required (shapes copies) and optional (grid cells)
//!   each row consist of a possible placement of a shape
//!

use advent_of_code::common::{cartesian_pairs, read_input};
use advent_of_code::dancing_links_x::DancingLinksX;
use advent_of_code::grid::Grid;
use advent_of_code::point::Point2D;
use std::clone::Clone;
use std::collections::HashSet;
use std::iter::{once, repeat_n};

fn main() {
    let input = read_input(2025, 12);

    println!("Part 1: {}", part1(&input));
}

const SIZE: usize = 3;
const FULL: char = '#';

#[derive(Clone, PartialEq, Eq, Hash)]
struct Shape {
    index: usize,
    points: Grid<char>,
}

impl Shape {
    fn parse(value: &str) -> Self {
        let (index, points) = value.split_once(":\n").unwrap();
        let index = index.parse::<usize>().unwrap();
        let points = Grid::parse(points);

        Self { index, points }
    }

    fn flip_horizontally(&self) -> Self {
        let mut out = self.clone();
        for (x, y) in cartesian_pairs(self.points.width, self.points.height) {
            let point = Point2D::new(x as isize, y as isize);
            let target = Point2D::new((self.points.width - 1 - x) as isize, y as isize);
            out.points[target] = self.points[point];
        }
        out
    }

    fn flip_vertically(&self) -> Self {
        let mut out = self.clone();
        for (x, y) in cartesian_pairs(self.points.width, self.points.height) {
            let point = Point2D::new(x as isize, y as isize);
            let target = Point2D::new(x as isize, (self.points.height - 1 - y) as isize);
            out.points[target] = self.points[point];
        }
        out
    }

    fn rotate(&self) -> Self {
        let mut out = self.clone();
        for (x, y) in cartesian_pairs(self.points.width, self.points.height) {
            let point = Point2D::new(x as isize, y as isize);
            let target = Point2D::new(y as isize, (self.points.width - 1 - x) as isize);
            out.points[target] = self.points[point];
        }
        out
    }

    fn full(&self) -> impl Iterator<Item = Point2D> {
        cartesian_pairs(self.points.width, self.points.height)
            .map(|(x, y)| Point2D::new(x as isize, y as isize))
            .filter(|&point| self.points[point] == FULL)
    }
}

struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

impl Region {
    fn parse(value: &str) -> Self {
        let (size, presents) = value.split_once(':').unwrap();
        let (width, height) = size.split_once('x').unwrap();

        let width = width.parse::<usize>().unwrap();
        let height = height.parse::<usize>().unwrap();
        let presents = presents
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self {
            width,
            height,
            presents,
        }
    }

    fn fit(&self, shapes: &[Vec<Shape>]) -> bool {
        let total_presents = self.presents.iter().sum::<usize>();
        if (self.width / SIZE) * (self.height / SIZE) >= total_presents {
            return true;
        }

        let min_required_space = self
            .presents
            .iter()
            .zip(shapes)
            .map(|(&count, transformed)| count * transformed[0].full().count())
            .sum::<usize>();
        if self.width * self.height < min_required_space {
            return false;
        }

        let mut dlx = DancingLinksX::new(total_presents + self.width * self.height, total_presents);
        self.presents
            .iter()
            .enumerate()
            .flat_map(|(index, &count)| repeat_n(index, count))
            .enumerate()
            .for_each(|(copy, index)| {
                for shape in &shapes[index] {
                    for (x, y) in cartesian_pairs(self.width - SIZE + 1, self.height - SIZE + 1) {
                        let row = once(copy)
                            .chain(shape.full().map(move |point| {
                                total_presents
                                    + (y + point.y as usize) * self.width
                                    + (x + point.x as usize)
                            }))
                            .map(|col| col + 1);
                        dlx.add_row(row);
                    }
                }
            });

        dlx.solve().is_some()
    }
}

const ROTATIONS: [fn(&Shape) -> Shape; 4] = [
    |shape: &Shape| shape.clone(),
    |shape: &Shape| shape.rotate(),
    |shape: &Shape| shape.rotate().rotate(),
    |shape: &Shape| shape.rotate().rotate().rotate(),
];

const FLIPS: [fn(&Shape) -> Shape; 4] = [
    |shape: &Shape| shape.clone(),
    |shape: &Shape| shape.flip_horizontally(),
    |shape: &Shape| shape.flip_vertically(),
    |shape: &Shape| shape.flip_horizontally().flip_vertically(),
];

fn part1(input: &str) -> usize {
    let (shapes, regions) = input.rsplit_once("\n\n").unwrap();

    let original = shapes.split("\n\n").map(Shape::parse).collect::<Vec<_>>();
    let transformed = original
        .iter()
        .map(|shape| {
            cartesian_pairs(ROTATIONS.len(), FLIPS.len())
                .map(move |(r, f)| ROTATIONS[r](&FLIPS[f](shape)))
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    regions
        .lines()
        .filter(|&region| Region::parse(region).fit(&transformed))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 2);
    }
}
