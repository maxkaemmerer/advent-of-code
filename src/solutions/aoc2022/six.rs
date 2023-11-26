use std::collections::HashSet;

use crate::common::file_to_lines;

fn solve_with_x_distinct(path: &str, distinct_chars: usize) -> usize {
    let skip = distinct_chars - 1;
    let lines: Vec<String> = file_to_lines(path);

    if let Some(data) = lines.iter().next() {
        let indexable_data: Vec<char> = data.chars().collect();

        let results = data
            .chars()
            .enumerate()
            .skip(skip)
            .map(|(index, _)| {
                let set: HashSet<&char> = indexable_data
                    .iter()
                    .skip(index - skip)
                    .take(distinct_chars)
                    .collect();
                println!("{:?}", set);
                (set.len() == distinct_chars, index)
            })
            .filter(|(result, _)| *result)
            .next()
            .map(|(_, index)| index)
            .unwrap_or(0);

        return results + 1;
    }

    0
}

pub fn solve_a(path: &str) -> usize {
    solve_with_x_distinct(path, 4)
}

pub fn solve_b(path: &str) -> usize {
    solve_with_x_distinct(path, 14)
}
