use crate::common::file_to_lines;
use std::collections::HashSet;
use std::str::Chars;

trait Unique {
    fn to_hash_set(self) -> HashSet<char>;
}

impl<'a> Unique for Chars<'a> {
    fn to_hash_set(self) -> HashSet<char> {
        let mut set = HashSet::new();
        self.for_each(|char| {
            set.insert(char);
            ()
        });

        return set;
    }
}

fn to_priority(char: char) -> u128 {
    return match char.is_uppercase() {
        true => char as u128 - 64 + 26,
        false => char as u128 - 96,
    };
}

pub fn solve_a(path: &str) -> u128 {
    let lines = file_to_lines(path);

    let duplicates = lines.iter().map(|line| -> u128 {
        let default = 0;

        let (first, second) = line.split_at(line.len() / 2);

        let first_unique = first.chars().to_hash_set();
        let second_unique = second.chars().to_hash_set();

        let intersecting_item_types = first_unique.intersection(&second_unique);

        let misplaced_item = intersecting_item_types
            .into_iter()
            .next()
            .map(|a| to_priority(a.clone()))
            .unwrap_or(default);

        misplaced_item
    });

    duplicates.sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::aoc2022::three::to_priority;

    #[test]
    fn should_parse_char_to_index_in_alphabet() {
        assert_eq!(1, to_priority('a'));
        assert_eq!(27, to_priority('A'));
    }
}
