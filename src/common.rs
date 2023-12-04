use std::fs;

pub fn file_to_lines(path: &str) -> Vec<String> {
    let input = fs::read(path).expect("File not found");

    let parse_result: String = String::from_utf8_lossy(&input)
        .parse()
        .expect("Could not parse file");

    let lines: Vec<String> = parse_result.lines().map(&str::to_string).collect();

    return lines;
}

pub fn remove_multiple_whitespaces(string: &str) -> String {
    let mut new_string = string.to_string();
    while new_string.contains("  ") {
        new_string = new_string.replace("  ", " ");
    }

    new_string
}
