//
// Helpers: grid, point
//
// Approach:
// - parse table
// - split into sub-tables: horizontal bounds are given by operators on last row
// - for each subtable: extract operator, extract operands (by row), compute result
// - finally, sum
//
// Part 2:
// - change extract operands (by col)
//

use advent_of_code::common::{parse_from_digits, read_input};
use advent_of_code::grid::Grid;
use advent_of_code::point::Point2D;

fn main() {
    let input = read_input(2025, 6);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

const SUM: u8 = b'+';
const MULTIPLY: u8 = b'*';

trait Extension {
    fn solve<F, I>(&self, extract_operands: F) -> u64
    where
        F: Fn(isize, isize) -> I,
        I: Iterator<Item = u64>;
    fn solve_by_rows(&self) -> u64;
    fn solve_by_columns(&self) -> u64;
}

impl Extension for Grid<u8> {
    fn solve<F, I>(&self, extract_operands: F) -> u64
    where
        F: Fn(isize, isize) -> I,
        I: Iterator<Item = u64>,
    {
        (0..self.width)
            .rev()
            .filter(|&x| matches!(self[Point2D::new(x, self.height - 1)], SUM | MULTIPLY))
            .scan(self.width, |right_bound, left_bound| {
                let horizontal_bounds = (left_bound, *right_bound);
                *right_bound = left_bound - 1;
                Some(horizontal_bounds)
            })
            .map(|(left_bound, right_bound)| {
                let operator = self[Point2D::new(left_bound, self.height - 1)];
                let operands = extract_operands(left_bound, right_bound);
                match operator {
                    SUM => operands.sum::<u64>(),
                    MULTIPLY => operands.product::<u64>(),
                    _ => unreachable!(),
                }
            })
            .sum()
    }

    fn solve_by_rows(&self) -> u64 {
        let operands_by_row = |left_bound, right_bound| {
            (0..self.height - 1).map(move |y| {
                let digits = (left_bound..right_bound)
                    .map(move |x| self[Point2D::new(x, y)])
                    .filter(|&digit| digit.is_ascii_digit());
                parse_from_digits(digits)
            })
        };
        self.solve(operands_by_row)
    }

    fn solve_by_columns(&self) -> u64 {
        let operands_by_column = |left_bound, right_bound| {
            (left_bound..right_bound).map(move |x| {
                let digits = (0..self.height - 1)
                    .map(move |y| self[Point2D::new(x, y)])
                    .filter(|&digit| digit.is_ascii_digit());
                parse_from_digits(digits)
            })
        };
        self.solve(operands_by_column)
    }
}

fn part1(input: &str) -> u64 {
    let table = Grid::parse(input);
    table.solve_by_rows()
}

fn part2(input: &str) -> u64 {
    let table = Grid::parse(input);
    table.solve_by_columns()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 3263827);
    }
}
