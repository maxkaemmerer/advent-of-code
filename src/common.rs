use std::{fs};

pub fn file_to_lines(path: &str) -> Vec<String>{
    let input = fs::read(path);

    if let Ok(content) = input {
        let parse_result: Result<String, _> = String::from_utf8_lossy(&content).parse();

        if let Ok(parsed_content) = parse_result {
            let lines: Vec<String> = parsed_content.lines().map(&str::to_string).collect();


            return lines
        }
    }

    return vec![]
}



pub fn validate_result(result: u128, expectation: u128, test: &str) {
    assert!(result == expectation, "Failed {}: {} should be {}", test, result, expectation);
    println!("Success {}: {}", test, result);
}