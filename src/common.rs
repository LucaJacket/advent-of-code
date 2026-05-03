pub fn read_input(year: u16, day: u8) -> String {
    let path = std::path::Path::new("input").join(format!("y{:04}_d{:02}.txt", year, day));
    std::fs::read_to_string(path).expect("failed to read file")
}
