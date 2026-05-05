//
// Approach:
// - check if region is large enough to fit all presents without interlocking
// - check if region is too small to fit all presents even with interlocking
// - if neither of the previous works, try to fit the presents manually
//

use advent_of_code::common::read_input;

fn main() {
    let input = read_input(2025, 12);

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    input
        .rsplit("\n\n")
        .next()
        .map(|regions| regions.lines())
        .expect("no regions")
        .map(|region_str| {
            let (size, amounts) = region_str.split_once(':').expect("failed to parse region");
            let (width, height) = size.split_once('x').expect("failed to parse region");
            let width: usize = width.parse().expect("failed to parse width");
            let height: usize = height.parse().expect("failed to parse height");
            let total_amount: usize = amounts
                .split_whitespace()
                .map(|amount| amount.parse::<usize>().expect("failed to parse number"))
                .sum();
            (width, height, total_amount)
        })
        .filter(|&(width, height, total_amount)| {
            if (width / 3) * (height / 3) >= total_amount {
                return true;
            }
            if width * height < 9 * total_amount {
                return false;
            }
            unreachable!()
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
        assert_eq!(part1(EXAMPLE), 7);
    }
}
