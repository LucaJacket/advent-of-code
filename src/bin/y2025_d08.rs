//
// Helpers: point, union_find
//
// Approach:
// - parse every junction to Point3D
// - build connections
// - use UnionFind to track circuits:
//   each junction is a node in a graph and each connection is a weighted edge (by distance)
// - consider only connection_count edges
// - compute circuit sizes via nodes, since linked nodes all share the same root
//
// Part 2:
// - consider all edges
// - if union was useful, update result
//

use advent_of_code::common::{read_input, unordered_pairs};
use advent_of_code::point::Point3D;
use advent_of_code::union_find::UnionFind;
use std::cmp::Reverse;

fn main() {
    let input = read_input(2025, 8);

    println!("Part 1: {}", part1(&input, 1000));
    println!("Part 2: {}", part2(&input));
}

fn build_connections(junctions: &[Point3D]) -> Vec<(usize, usize)> {
    let mut connections = unordered_pairs(junctions.len()).collect::<Vec<_>>();
    connections
        .sort_unstable_by_key(|&(i, j)| Point3D::distance_squared(junctions[i], junctions[j]));

    connections
}

fn part1(input: &str, connection_count: usize) -> usize {
    let junctions = input.lines().map(Point3D::parse).collect::<Vec<_>>();
    let connections = build_connections(&junctions);

    let mut union_find = UnionFind::new(junctions.len());
    for (i, j) in connections.into_iter().take(connection_count) {
        union_find.union(i, j);
    }
    let mut sizes = vec![0; junctions.len()];
    for i in 0..junctions.len() {
        let root = union_find.find(i);
        sizes[root] += 1;
    }

    sizes.sort_unstable_by_key(|&size| Reverse(size));
    sizes.into_iter().take(3).product()
}

fn part2(input: &str) -> isize {
    let junctions = input.lines().map(Point3D::parse).collect::<Vec<_>>();
    let connections = build_connections(&junctions);

    let mut result = 0;
    let mut union_find = UnionFind::new(junctions.len());

    for (i, j) in connections.into_iter() {
        if union_find.union(i, j) {
            result = junctions[i].x * junctions[j].x;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE, 10), 40);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 25272);
    }
}
