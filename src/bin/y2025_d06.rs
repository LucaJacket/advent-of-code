//!
//! Helpers: grid
//!
//! Approach:
//! - treat operators row differently
//! - parse table based on split_whitespace
//! - pair each column of operands with its corresponding operator
//! - for each pair, solve
//! - finally, sum
//!
//! Part 2:
//! - parse table
//! - parse each column as an operand: an empty column should fail parsing, so it is a sentinel of
//!   where to split the groups of operands
//! - for each operator, group the operands and solve
//! - finally, sum
//!

use advent_of_code::common::read_input;
use advent_of_code::grid::Grid;

fn main() {
    let input = read_input(2025, 6);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

const SUM: &str = "+";
const PRODUCT: &str = "*";

fn part1(input: &str) -> u64 {
    let (operands, operators) = input.rsplit_once('\n').unwrap();
    let operators = operators.split_whitespace();
    let table: Grid<&str> = Grid::split_whitespace(operands);

    let cols = (0..table.width as isize).map(|x| {
        table
            .iter_col(x)
            .map(|&operand| operand.parse::<u64>().unwrap())
    });
    cols.zip(operators)
        .map(|(operands, operator)| match operator {
            SUM => operands.sum::<u64>(),
            PRODUCT => operands.product::<u64>(),
            _ => {
                println!("{}", operator);
                unreachable!()
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let (operands, operators) = input.rsplit_once('\n').unwrap();
    let operators = operators.split_whitespace();
    let table: Grid<char> = Grid::from_chars(operands);

    let mut cols = (0..table.width as isize).map(|x| {
        table
            .iter_col(x)
            .collect::<String>()
            .trim()
            .parse::<u64>()
            .ok()
    });
    operators
        .map(|operator| {
            let operands = cols.by_ref().map_while(|operand| operand);
            match operator {
                SUM => operands.sum::<u64>(),
                PRODUCT => operands.product::<u64>(),
                _ => {
                    println!("{}", operator);
                    unreachable!()
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

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
