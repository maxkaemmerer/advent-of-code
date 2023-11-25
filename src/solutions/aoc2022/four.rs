use crate::common;
use std::collections::HashSet;
use std::ops::RangeInclusive;

trait Contains<T> {
    fn contains_range(&self, other: &T) -> bool;
}

impl<T: Ord> Contains<RangeInclusive<T>> for RangeInclusive<T> {
    fn contains_range(&self, other: &RangeInclusive<T>) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }
}

pub fn solve_with_comparison(
    path: &str,
    comparison: fn(RangeInclusive<usize>, RangeInclusive<usize>) -> bool,
) -> usize {
    let lines = common::file_to_lines(path);

    lines
        .iter()
        .filter(|line| {
            let mut sections = line.split(',').take(2).map(|section| {
                let mut ends = section.split('-').take(2);
                if let (Some(start), Some(end)) = (
                    ends.next().and_then(|c| c.parse::<usize>().ok()),
                    ends.next().and_then(|c| c.parse::<usize>().ok()),
                ) {
                    // we use an inclusive range here, otherwise the end is excluded in intersection in part b later
                    return RangeInclusive::new(start, end);
                }

                RangeInclusive::new(0, 0)
            });

            if let [Some(section_a), Some(section_b)] = [sections.next(), sections.next()] {
                return comparison(section_a, section_b);
            }

            false
        })
        .count() as usize
}

pub fn solve_a(path: &str) -> usize {
    solve_with_comparison(path, |section_a, section_b| {
        section_a.contains_range(&section_b) || section_b.contains_range(&section_a)
    })
}
pub fn solve_b(path: &str) -> usize {
    solve_with_comparison(path, |section_a, section_b| {
        let set_a: HashSet<usize> = section_a.collect();
        let set_b: HashSet<usize> = section_b.collect();

        let intersection = set_a.intersection(&set_b);

        intersection.count() > 0
    })
}
