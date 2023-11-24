use std::fs;

pub fn file_to_lines(path: &str) -> Vec<String> {
    let input = fs::read(path).expect("File not found");

    let parse_result: String = String::from_utf8_lossy(&input)
        .parse()
        .expect("Could not parse file");

    let lines: Vec<String> = parse_result.lines().map(&str::to_string).collect();

    return lines;
}
