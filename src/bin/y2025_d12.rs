//
// Approach:
// - check if region is large enough to fit all presents without interlocking
// - check if region is too small to fit all presents even with perfect interlocking
// - if neither of the previous works, try to fit the presents manually:
//   DFS search + backtracking
//

use advent_of_code::common::{cartesian_pairs, read_input};
use advent_of_code::grid::Grid;
use advent_of_code::point::Point2D;
use std::clone::Clone;
use std::collections::HashSet;

fn main() {
    let input = read_input(2025, 12);

    println!("Part 1: {}", part1(&input));
}

const SIZE: usize = 3;
const FULL: u8 = b'#';
const EMPTY: u8 = b'.';

#[derive(Clone, PartialEq, Eq, Hash)]
struct Shape {
    index: usize,
    points: Grid<u8>,
}

impl Shape {
    fn parse(value: &str) -> Self {
        let (index, points) = value.split_once(":\n").unwrap();
        let index = index.parse::<usize>().unwrap();
        let points = Grid::parse(points);

        Self { index, points }
    }

    fn required_space(&self) -> usize {
        cartesian_pairs(self.points.width, self.points.height)
            .map(|(x, y)| Point2D::new(x as isize, y as isize))
            .filter(|&point| self.points[point] == FULL)
            .count()
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
}

struct Region {
    presents: Vec<usize>,
    points: Grid<u8>,
}

impl Region {
    fn parse(value: &str) -> Self {
        let (size, presents) = value.split_once(':').unwrap();
        let (width, height) = size.split_once('x').unwrap();
        let width = width.parse::<usize>().unwrap();
        let height = height.parse::<usize>().unwrap();
        let points = Grid::new(width, height, EMPTY);
        let presents = presents
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self { presents, points }
    }

    fn fit(&mut self, original: &[Shape], transformed: &[Shape]) -> bool {
        let total_presents = self.presents.iter().sum();
        if (self.points.width / SIZE) * (self.points.height / SIZE) >= total_presents {
            return true;
        }
        let min_required_space = original
            .iter()
            .zip(self.presents.iter())
            .map(|(shape, &count)| shape.required_space() * count)
            .sum();
        if self.points.width * self.points.height < min_required_space {
            return false;
        }
        self.dfs(transformed)
    }

    fn dfs(&mut self, shapes: &[Shape]) -> bool {
        if self.presents.iter().sum::<usize>() == 0 {
            return true;
        }

        let w = self.points.width - SIZE + 1;
        let h = self.points.height - SIZE + 1;
        for (x, y) in cartesian_pairs(w, h) {
            let position = Point2D::new(x as isize, y as isize);
            for shape in shapes.iter() {
                if self.presents[shape.index] == 0 {
                    continue;
                }
                if self.can_place(shape, position) {
                    self.place(shape, position);
                    if self.dfs(shapes) {
                        return true;
                    }
                    self.unplace(shape, position);
                }
            }
        }

        false
    }

    fn can_place(&self, shape: &Shape, position: Point2D) -> bool {
        cartesian_pairs(shape.points.width, shape.points.height)
            .map(|(x, y)| Point2D::new(x as isize, y as isize))
            .all(|point| self.points[point + position] == EMPTY || shape.points[point] == EMPTY)
    }

    fn place(&mut self, shape: &Shape, position: Point2D) {
        for (x, y) in cartesian_pairs(shape.points.width, shape.points.height) {
            let point = Point2D::new(x as isize, y as isize);
            if shape.points[point] == FULL {
                self.points[point + position] = FULL;
            }
        }
        self.presents[shape.index] -= 1;
    }

    fn unplace(&mut self, shape: &Shape, position: Point2D) {
        for (x, y) in cartesian_pairs(shape.points.width, shape.points.height) {
            let point = Point2D::new(x as isize, y as isize);
            if shape.points[point] == FULL {
                self.points[point + position] = EMPTY;
            }
        }
        self.presents[shape.index] += 1;
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
        .flat_map(|shape| {
            cartesian_pairs(ROTATIONS.len(), FLIPS.len())
                .map(move |(r, f)| ROTATIONS[r](&FLIPS[f](&shape)))
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    regions
        .lines()
        .filter(|&region| {
            let mut region = Region::parse(region);
            region.fit(&original, &transformed)
        })
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
