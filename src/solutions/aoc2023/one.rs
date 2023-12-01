use crate::common::file_to_lines;

type Valid<'a> = (&'a str, &'a str, usize);

fn solve(path: &str, convert_string_numbers: bool) -> usize {
    let lines = file_to_lines(path);

    let result = lines
        .iter()
        .map(|line| numbers_in_string(line, convert_string_numbers))
        .map(|nums: Vec<usize>| {
            let res = match nums[..] {
                [first, .., last] => first * 10 + last,
                [one] => one * 10 + one,
                _ => panic!("Invalid input, line does not contain numbers"),
            };
            println!("{:?} becomes {:?}", nums, res);
            res
        })
        .sum();

    println!("{}", result);

    result
}

pub fn solve_a(path: &str) -> usize {
    solve(path, false)
}

pub fn solve_b(path: &str) -> usize {
    solve(path, true)
}

fn occurrences_in_string(valid: &Valid, string: &str, include_string_numbers: bool) -> Vec<usize> {
    let mut indices = vec![];

    if include_string_numbers {
        indices.append(find_all_in_string(valid.0.to_string(), string.to_string()).as_mut());
    }

    indices.append(find_all_in_string(valid.1.to_string(), string.to_string()).as_mut());

    indices
}

fn find_all_in_string(needle: String, haystack: String) -> Vec<usize> {
    let mut indices = vec![];
    let mut search_string = haystack;
    let mut previous_find = 0;
    loop {
        let res2 = search_string.find(needle.as_str());
        if let Some(found) = res2 {
            let index_from = found + 1;
            previous_find = previous_find + index_from;
            indices.push(previous_find.clone());
            println!(
                "Found {needle} at {previous_find} in {}",
                search_string.clone()
            );

            search_string = search_string[index_from..].to_string();
        } else {
            break;
        }
    }

    indices
}

fn numbers_in_string(string: &str, include_string_numbers: bool) -> Vec<usize> {
    let valid_values: Vec<Valid> = vec![
        ("one", "1", 1),
        ("two", "2", 2),
        ("three", "3", 3),
        ("four", "4", 4),
        ("five", "5", 5),
        ("six", "6", 6),
        ("seven", "7", 7),
        ("eight", "8", 8),
        ("nine", "9", 9),
    ];

    let mut found_digits = vec![];

    valid_values.iter().for_each(|valid_value| {
        occurrences_in_string(valid_value, string, include_string_numbers)
            .iter()
            .for_each(|index| {
                found_digits.push((index.clone(), valid_value.2.clone()));
            });
    });

    found_digits.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("Could not sort values"));

    let numbers = found_digits.iter().map(|digit| digit.1).collect();

    println!("{string} to {:?}", numbers);

    numbers
}

#[cfg(test)]
mod tests {
    use crate::solutions::aoc2023::one::numbers_in_string;

    #[test]
    pub fn should_get_overlappinng_numbers() {
        assert_eq!(vec![6, 1, 8], numbers_in_string("6oneight", true));
        assert_eq!(vec![6], numbers_in_string("6oneight", false));
        assert_eq!(vec![7, 8, 7], numbers_in_string("a7e8e7g", true));
        assert_eq!(vec![1], numbers_in_string("1", true));
        assert_eq!(
            vec![5, 8, 8],
            numbers_in_string("seven588sevenxffivethreehkrczrlm", false)
        );
        assert_eq!(
            vec![7, 7, 5],
            numbers_in_string("nsvkljgpfn77pvfour5j", false)
        );
    }
}
