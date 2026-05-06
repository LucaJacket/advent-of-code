//
// Helpers: grid, point
//
// Approach:
// - parse table
// - from width to 0, find all the columns containing + or * on the last row
// - each column containing an operator is start_x, while end_x = width
// - extract operands by row:
//   for each row, extract the columns between start_x and end_x, then parse the number
// - apply the operator
// - update end_x = start_x - 1
//
// Part 2:
// - just extract operands by column:
//   for each column between start_x and end_x, extract the rows, then parse the number
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
    fn solve<F, I>(&self, operands: F) -> u64
    where
        F: Fn(isize, isize) -> I,
        I: Iterator<Item = u64>;
    fn solve_by_rows(&self) -> u64;
    fn solve_by_columns(&self) -> u64;
}

impl Extension for Grid<u8> {
    fn solve<F, I>(&self, operands: F) -> u64
    where
        F: Fn(isize, isize) -> I,
        I: Iterator<Item = u64>,
    {
        (0..self.width)
            .rev()
            .map(|x| {
                let operator = self[Point2D::new(x, self.height - 1)];
                (x, operator)
            })
            .filter(|&(_, operator)| operator == SUM || operator == MULTIPLY)
            .scan(self.width, |end_x, (start_x, operator)| {
                let operands = operands(start_x, *end_x);
                let result = match operator {
                    SUM => operands.sum(),
                    MULTIPLY => operands.product(),
                    _ => 0,
                };
                *end_x = start_x - 1;
                Some(result)
            })
            .sum()
    }

    fn solve_by_rows(&self) -> u64 {
        let operands_by_row = |start_x, end_x| {
            (0..self.height - 1)
                .map(move |y| {
                    (start_x..end_x)
                        .map(move |x| self[Point2D::new(x, y)])
                        .filter(|&digit| digit.is_ascii_digit())
                })
                .map(parse_from_digits)
        };
        self.solve(operands_by_row)
    }

    fn solve_by_columns(&self) -> u64 {
        let operands_by_column = |start_x, end_x| {
            (start_x..end_x)
                .map(move |x| {
                    (0..self.height - 1)
                        .map(move |y| self[Point2D::new(x, y)])
                        .filter(|&digit| digit.is_ascii_digit())
                })
                .map(parse_from_digits)
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
