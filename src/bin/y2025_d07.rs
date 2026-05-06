//
// Approach:
// - one-hot encode the 1st line (S = true)
// - for each next line, clean next and for each index:
//   if current[i] contains a beam and row[i] contains a splitter, increment counter and split
//   else if current[i] contains a beam, go through
//   finally, set current = next
//
// Part 2:
// - change the encoding from bool to usize to address multiplicity counting
// - simulate again
// - finally, sum all the multiplicities in the final state
//

use advent_of_code::common::read_input;
use std::mem::swap;

fn main() {
    let input = read_input(2025, 7);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

const START: u8 = b'S';
const SPLITTER: u8 = b'^';

fn part1(input: &str) -> usize {
    let mut lines = input.lines().map(str::as_bytes);
    let mut current = lines
        .next()
        .unwrap()
        .iter()
        .map(|&x| x == START)
        .collect::<Vec<_>>();
    let mut next = vec![false; current.len()];
    let mut splittings = 0;
    for splitters in lines {
        next.fill(false);
        for i in 0..splitters.len() {
            if current[i] {
                if splitters[i] == SPLITTER {
                    splittings += 1;
                    next[i - 1] = current[i];
                    next[i + 1] = current[i];
                } else {
                    next[i] = current[i];
                }
            }
        }
        swap(&mut current, &mut next);
    }
    splittings
}

fn part2(input: &str) -> usize {
    let mut rows = input.lines().map(str::as_bytes);
    let mut current = rows
        .next()
        .unwrap()
        .iter()
        .map(|&x| (x == START) as usize)
        .collect::<Vec<_>>();
    let mut next = vec![0; current.len()];
    for row in rows {
        next.fill(0);
        for i in 0..row.len() {
            if current[i] > 0 {
                if row[i] == SPLITTER {
                    next[i - 1] += current[i];
                    next[i + 1] += current[i];
                } else {
                    next[i] += current[i];
                }
            }
        }
        swap(&mut current, &mut next);
    }
    current.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 40);
    }
}
