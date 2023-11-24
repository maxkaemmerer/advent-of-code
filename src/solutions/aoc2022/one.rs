use crate::common;

fn find_highest_x(lines: Vec<String>, x: usize) -> u128 {
    let groups = lines.split(String::is_empty);

    let mut sums: Vec<u128> = groups.map(sum_strings).into_iter().collect();
    sums.sort();
    sums.reverse();

    sums.iter().take(x).sum()
}

fn sum_strings(strings: &[String]) -> u128 {
    strings
        .into_iter()
        .map(|string| string.parse::<u128>().unwrap())
        .sum()
}

pub fn solve_a(path: &str) -> u128 {
    let lines = common::file_to_lines(path);
    return find_highest_x(lines, 1);
}

pub fn solve_b(path: &str) -> u128 {
    let lines = common::file_to_lines(path);
    return find_highest_x(lines, 3);
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
            find_highest_x(
                vec![
                    "321".to_string(),
                    "321".to_string(),
                    "".to_string(),
                    "600".to_string(),
                ],
                1
            ),
            642
        );
    }

    #[test]
    pub fn test_find_highest_three() {
        assert_eq!(
            find_highest_x(
                vec![
                    "1000".to_string(),
                    "2000".to_string(),
                    "3000".to_string(),
                    "".to_string(),
                    "4000".to_string(),
                    "".to_string(),
                    "5000".to_string(),
                    "6000".to_string(),
                    "".to_string(),
                    "7000".to_string(),
                    "8000".to_string(),
                    "9000".to_string(),
                    "".to_string(),
                    "10000".to_string(),
                ],
                3
            ),
            45000
        );
    }
}
