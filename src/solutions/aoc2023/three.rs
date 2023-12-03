use crate::common::file_to_lines;
use std::collections::HashSet;

fn valid_numbers_on_line(
    line: &str,
    line_before: Option<&str>,
    line_after: Option<&str>,
) -> Vec<usize> {
    let numbers_on_line: Vec<Number> = numbers_in_line(Some(line));
    let special_set_on_line = find_occurrences_on_line_by_comparison(Some(line), |char| {
        char != &'.' && !char.is_numeric()
    });
    let special_set_before = find_occurrences_on_line_by_comparison(line_before, |char| {
        char != &'.' && !char.is_numeric()
    });
    let special_set_after = find_occurrences_on_line_by_comparison(line_after, |char| {
        char != &'.' && !char.is_numeric()
    });

    let mut valid_numbers = vec![];

    numbers_on_line.iter().for_each(|(index, number)| {
        let set: HashSet<usize> = search_set(number, index);

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

fn search_set(number: &str, index: &usize) -> HashSet<usize> {
    let cloned_index = index.clone();
    let start = if cloned_index == 0 {
        cloned_index
    } else {
        cloned_index - 1
    };
    let end = cloned_index + number.len() + 1;
    let range = start..end;

    range.collect()
}

fn gear_score_on_line(
    line: &str,
    line_before: Option<&str>,
    line_after: Option<&str>,
) -> Vec<usize> {
    let gears_on_line = find_occurrences_on_line_by_comparison(Some(line), |char| char == &'*');

    let numbers_on_line = numbers_in_line(Some(line));
    let numbers_before = numbers_in_line(line_before);
    let numbers_after = numbers_in_line(line_after);

    let mut valid_numbers = vec![];

    gears_on_line.iter().for_each(|index| {
        let mut adjacent_numbers: Vec<String> = vec![];
        adjacent_numbers.append(&mut find_adjacent_numbers(numbers_before.clone(), index));
        adjacent_numbers.append(&mut find_adjacent_numbers(numbers_after.clone(), index));
        adjacent_numbers.append(&mut find_adjacent_numbers(numbers_on_line.clone(), index));

        if adjacent_numbers.len() == 2 {
            let gear_score = adjacent_numbers.iter().fold(1, |init, number| {
                init * number.parse::<usize>().unwrap_or_default()
            });
            valid_numbers.push(gear_score);
        }
    });

    valid_numbers
}

type Number<'a> = (usize, String);

fn find_adjacent_numbers(numbers: Vec<Number>, index: &usize) -> Vec<String> {
    numbers
        .iter()
        .filter(|number| is_adjacent_to(number, index))
        .map(|number| number.1.clone())
        .collect()
}

fn numbers_in_line(line: Option<&str>) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];
    let mut current_number = (0, String::from(""));
    if let Some(the_line) = line {
        the_line.chars().enumerate().for_each(|(index, char)| {
            if char.is_numeric() {
                if current_number.1.is_empty() {
                    current_number.0 = index;
                }
                current_number.1.push(char);
                return;
            }

            if !current_number.1.is_empty() {
                numbers.push(current_number.clone());
                current_number = (0, String::from(""));
            }
        });
    }

    if !current_number.1.is_empty() {
        numbers.push(current_number.clone());
    }

    numbers
}

fn is_adjacent_to(number: &Number, index: &usize) -> bool {
    let set: HashSet<usize> = search_set(&number.1, &number.0);

    set.contains(index)
}

fn find_occurrences_on_line_by_comparison(
    line: Option<&str>,
    comparison: fn(char: &char) -> bool,
) -> HashSet<usize> {
    if let Some(line) = line {
        return HashSet::from_iter(
            line.chars()
                .enumerate()
                .filter(|(index, char)| comparison(char))
                .map(|(index, _)| index),
        );
    }

    HashSet::new()
}

type Solver = fn(line: &str, line_before: Option<&str>, line_after: Option<&str>) -> Vec<usize>;

fn solve_with(path: &str, solver: Solver) -> usize {
    let lines = file_to_lines(path);

    lines
        .iter()
        .enumerate()
        .map(|(index, line)| -> usize {
            let line_before = if index > 0 {
                lines.get(index - 1).map(|line_before| line_before.as_str())
            } else {
                None
            };
            let line_after = lines.get(index + 1).map(|line_after| line_after.as_str());

            solver(line, line_before, line_after).iter().sum()
        })
        .sum()
}

pub fn solve_a(path: &str) -> usize {
    solve_with(path, valid_numbers_on_line)
}

pub fn solve_b(path: &str) -> usize {
    solve_with(path, gear_score_on_line)
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
