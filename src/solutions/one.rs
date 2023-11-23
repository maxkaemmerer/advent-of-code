use crate::common;

fn find_highest(lines: Vec<String>) -> u16 {
    let groups = lines.split(String::is_empty);

    let sums = groups.map(sum_strings);

    sums.max().unwrap_or_default()
}

fn sum_strings(strings: &[String]) -> u16 {
    strings
        .into_iter()
        .map(|string| string.parse::<u16>().unwrap())
        .sum()
}

pub fn solve(path: &str) -> u16 {
    let lines = common::file_to_lines(path);
    return find_highest(lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sum() {
        assert_eq!(
            sum_strings(&["3".to_string(), "2".to_string(), "1".to_string()]),
            6
        );
    }

    #[test]
    pub fn test_find_highest() {
        assert_eq!(
            find_highest(vec![
                "321".to_string(),
                "321".to_string(),
                "".to_string(),
                "600".to_string(),
            ]),
            642
        );
    }
}
