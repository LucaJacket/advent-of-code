use std::fs::read_to_string;
use std::path::Path;

pub fn read_input(year: u16, day: u8) -> String {
    let filename = format!("y{:04}_d{:02}.txt", year, day);
    let path = Path::new("input").join(filename);
    read_to_string(path).unwrap()
}

pub fn parse_from_digits(digits: impl IntoIterator<Item = char>) -> u64 {
    digits
        .into_iter()
        .collect::<String>()
        .trim()
        .parse::<u64>()
        .unwrap()
}

pub fn cartesian_pairs(n: usize, m: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..n).flat_map(move |i| (0..m).map(move |j| (i, j)))
}

pub fn unordered_pairs(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..n).flat_map(move |i| (i + 1..n).map(move |j| (i, j)))
}
