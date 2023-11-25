use crate::common::file_to_lines;
use std::collections::HashSet;

fn to_priority(char: &char) -> usize {
    match char.is_uppercase() {
        // we are casting the char to its position in the ascii table and removing the offset to get it started at 1 and 27 respectively
        true => *char as usize - 64 + 26,
        false => *char as usize - 96,
    }
}

pub fn solve_a(path: &str) -> usize {
    let lines = file_to_lines(path);

    let duplicates = lines.iter().map(|line| -> usize {
        let default = 0;

        let (first, second) = line.split_at(line.len() / 2);

        let first_unique: HashSet<char> = first.chars().collect();
        let second_unique: HashSet<char> = second.chars().collect();

        let intersecting_item_types = first_unique.intersection(&second_unique);

        let misplaced_item = intersecting_item_types
            .into_iter()
            .next()
            .map(|a| to_priority(a))
            .unwrap_or(default);

        misplaced_item
    });

    duplicates.sum()
}

pub fn solve_b(path: &str) -> usize {
    let lines = file_to_lines(path);
    let default = 0usize;

    let group_scores = lines.chunks(3).map(|group| {
        let first_backpack = group[0].chars();
        let second_backpack: HashSet<char> = group[1].chars().collect();
        let third_backpack: HashSet<char> = group[2].chars().collect();

        let items_in_all_backpacks = first_backpack
            .filter(|char| second_backpack.contains(char) && third_backpack.contains(char));

        items_in_all_backpacks
            .last()
            .map(|item| to_priority(&item))
            .unwrap_or(default)
    });

    group_scores.sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::aoc2022::three::to_priority;

    #[test]
    fn should_parse_char_to_index_in_alphabet() {
        assert_eq!(1, to_priority(&'a'));
        assert_eq!(27, to_priority(&'A'));
    }
}
