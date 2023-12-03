use crate::common::file_to_lines;
use std::collections::HashSet;

fn valid_numbers_on_line(
    line: &str,
    line_before: Option<&str>,
    line_after: Option<&str>,
) -> Vec<usize> {
    let mut numbers_on_line: Vec<(usize, String)> = vec![];
    let mut current_number: (usize, String) = (0, String::from(""));
    let mut special_chars_on_line = vec![];
    let mut special_chars_before = vec![];
    let mut special_chars_after = vec![];
    line.chars().enumerate().for_each(|(index, char)| {
        if char.is_numeric() {
            if current_number.1.is_empty() {
                current_number.0 = index;
            }
            current_number.1.push(char);
            return;
        }

        if char != '.' {
            special_chars_on_line.push(index);
        }

        if !current_number.1.is_empty() {
            numbers_on_line.push(current_number.clone());
            current_number = (0, String::from(""));
        }
    });

    if !current_number.1.is_empty() {
        numbers_on_line.push(current_number.clone());
    }

    if let Some(the_line) = line_before {
        the_line.chars().enumerate().for_each(|(index, char)| {
            if char != '.' && !char.is_numeric() {
                special_chars_before.push(index);
            }
        });
    }

    if let Some(the_line) = line_after {
        the_line.chars().enumerate().for_each(|(index, char)| {
            if char != '.' && !char.is_numeric() {
                special_chars_after.push(index);
            }
        });
    }

    let special_set_before: HashSet<usize> = HashSet::from_iter(special_chars_before);
    let special_set_after: HashSet<usize> = HashSet::from_iter(special_chars_after);
    let special_set_on_line: HashSet<usize> = HashSet::from_iter(special_chars_on_line);

    let mut valid_numbers = vec![];

    numbers_on_line.iter().for_each(|(index, number)| {
        let cloned_index = index.clone();
        let start = if cloned_index == 0 {
            cloned_index
        } else {
            cloned_index - 1
        };
        let end = if cloned_index == line.len() - 1 {
            cloned_index + number.len()
        } else {
            cloned_index + number.len() + 1
        };
        let range = start..end;
        let set: HashSet<usize> = range.collect();

        if let Some(parsed_number) = number.parse::<usize>().ok() {
            if set.intersection(&special_set_before).count() != 0 {
                valid_numbers.push(parsed_number);
                return;
            }
            if set.intersection(&special_set_after).count() != 0 {
                valid_numbers.push(parsed_number);
                return;
            }
            if set.intersection(&special_set_on_line).count() != 0 {
                valid_numbers.push(parsed_number);
                return;
            }
        }
    });

    valid_numbers
}

fn gear_score_on_line(
    line: &str,
    line_before: Option<&str>,
    line_after: Option<&str>,
) -> Vec<usize> {
    let mut numbers_on_line: Vec<(usize, String)> = vec![];
    let mut current_number: (usize, String) = (0, String::from(""));
    let mut gears_on_line = vec![];
    let mut numbers_before: Vec<(usize, String)> = vec![];
    let mut numbers_after: Vec<(usize, String)> = vec![];
    line.chars().enumerate().for_each(|(index, char)| {
        if char.is_numeric() {
            if current_number.1.is_empty() {
                current_number.0 = index;
            }
            current_number.1.push(char);
            return;
        }

        if char == '*' {
            gears_on_line.push(index);
        }

        if !current_number.1.is_empty() {
            numbers_on_line.push(current_number.clone());
            current_number = (0, String::from(""));
        }
    });

    if !current_number.1.is_empty() {
        numbers_on_line.push(current_number.clone());
    }

    if let Some(the_line) = line_before {
        the_line.chars().enumerate().for_each(|(index, char)| {
            if char.is_numeric() {
                if current_number.1.is_empty() {
                    current_number.0 = index;
                }
                current_number.1.push(char);
                return;
            }

            if !current_number.1.is_empty() {
                numbers_before.push(current_number.clone());
                current_number = (0, String::from(""));
            }
        });
    }

    if !current_number.1.is_empty() {
        numbers_on_line.push(current_number.clone());
    }

    if let Some(the_line) = line_after {
        the_line.chars().enumerate().for_each(|(index, char)| {
            if char.is_numeric() {
                if current_number.1.is_empty() {
                    current_number.0 = index;
                }
                current_number.1.push(char);
                return;
            }

            if !current_number.1.is_empty() {
                numbers_after.push(current_number.clone());
                current_number = (0, String::from(""));
            }
        });
    }

    if !current_number.1.is_empty() {
        numbers_on_line.push(current_number.clone());
    }

    let gear_set_on_line: HashSet<usize> = HashSet::from_iter(gears_on_line);

    let mut valid_numbers = vec![];

    gear_set_on_line.iter().for_each(|index| {
        let mut adjacent_numbers = vec![];

        numbers_before.iter().for_each(|number| {
            if is_adjacent_to(number, index) {
                adjacent_numbers.push(number.1.clone())
            }
        });

        numbers_on_line.iter().for_each(|number| {
            if is_adjacent_to(number, index) {
                adjacent_numbers.push(number.1.clone())
            }
        });

        numbers_after.iter().for_each(|number| {
            if is_adjacent_to(number, index) {
                adjacent_numbers.push(number.1.clone())
            }
        });

        if adjacent_numbers.len() == 2 {
            let gear_score = adjacent_numbers.iter().fold(1, |init, number| {
                init * number.parse::<usize>().unwrap_or_default()
            });
            // println!("{index} is adjacent to {:?} with score {gear_score} on line {line}", adjacent_numbers);
            valid_numbers.push(gear_score);
        }
    });

    valid_numbers
}

fn is_adjacent_to(number: &(usize, String), index: &usize) -> bool {
    let cloned_index = number.0.clone();
    let start = if cloned_index == 0 {
        cloned_index
    } else {
        cloned_index - 1
    };
    let end = cloned_index + number.1.len() + 1;
    let range = start..end;
    let set: HashSet<usize> = range.collect();

    set.contains(index)
}

pub fn solve_a(path: &str) -> usize {
    let lines = file_to_lines(path);

    lines
        .iter()
        .enumerate()
        .map(|(index, line)| -> usize {
            // for before we need to check if there is unsigned integer underflow
            let line_before = if index > 0 {
                lines.get(index - 1).map(|line_before| line_before.as_str())
            } else {
                None
            };
            let line_after = lines.get(index + 1).map(|line_after| line_after.as_str());

            valid_numbers_on_line(line, line_before, line_after)
                .iter()
                .sum()
        })
        .sum()
}
pub fn solve_b(path: &str) -> usize {
    let lines = file_to_lines(path);

    lines
        .iter()
        .enumerate()
        .map(|(index, line)| -> usize {
            // for before we need to check if there is unsigned integer underflow
            let line_before = if index > 0 {
                lines.get(index - 1).map(|line_before| line_before.as_str())
            } else {
                None
            };
            let line_after = lines.get(index + 1).map(|line_after| line_after.as_str());

            gear_score_on_line(line, line_before, line_after)
                .iter()
                .sum()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::aoc2023::three::{gear_score_on_line, valid_numbers_on_line};

    #[test]
    fn should_correctly_determine_valid_numbers() {
        assert_eq!(
            vec![467],
            valid_numbers_on_line("467..114..", None, Some("...*......"))
        );

        assert_eq!(
            vec![664, 598],
            valid_numbers_on_line(".664.598..", Some("...$.*...."), None)
        );

        let expectation: Vec<usize> = vec![];
        assert_eq!(
            expectation,
            valid_numbers_on_line(".....+.58.", Some("617*......"), Some("..592....."))
        );

        assert_eq!(
            vec![467, 114],
            valid_numbers_on_line("467....114", None, Some("*........*"))
        );

        assert_eq!(vec![467, 114], valid_numbers_on_line("467*114", None, None));

        assert_eq!(vec![343, 750, 661, 323, 480, 198, 533, 764], valid_numbers_on_line(
            ".......-.............343......750..661....%........+..323.....1..............480.........+..............198.......................533.../764",
            Some("..2...295....../........*336..*......#..185...........*.........502......301...*.......................*...........-.770..-...599..........."),
            Some("..................................................799.....832.......640.................413...392............597.................=...........")
        ));
        assert_eq!(
            vec![24600],
            gear_score_on_line("...*...", Some("123...."), Some("....200"))
        );
    }
}
