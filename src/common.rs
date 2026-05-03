pub fn read_input(year: u16, day: u8) -> String {
    let path = std::path::Path::new("input").join(format!("y{:04}_d{:02}.txt", year, day));
    std::fs::read_to_string(path).expect("failed to read file")
}

pub fn parse_from_digits(digits: impl IntoIterator<Item = u8>) -> u64 {
    digits
        .into_iter()
        .fold(0, |acc, x| acc * 10 + (x - b'0') as u64)
}
