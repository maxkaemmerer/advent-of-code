use std::ops::Range;
use crate::common::file_to_lines;
use crate::tokens::{parse_token_value_before, Token};

#[derive(Debug)]
struct RangeMapping {
    source_range: Range<usize>,
    destination_range: Range<usize>,
}

impl RangeMapping {
    fn source_to_destination(&self, key: usize) -> Option<usize> {
        if self.source_range.contains(&key) {
            return Some(self.destination_range.start + (key - self.source_range.start));
        }

        None
    }
}

#[derive(Debug)]
struct Mapper {
    _name: String,
    range_mappings: Vec<RangeMapping>,
    decorated_mapper: Option<Box<Mapper>>,
}

impl Mapper {
    fn from_lines_and_next(name: String, lines: Vec<String>, next: Option<Box<Mapper>>) -> Self {
        let range_mappings: Vec<RangeMapping> = lines
            .iter()
            .map(|line| {
                let data_points: Vec<usize> = line
                    .split_whitespace()
                    .map(|data_point| {
                        data_point
                            .parse::<usize>()
                            .expect("invalid input, map data point is not a number")
                    })
                    .collect();
                if let [destination_start, source_start, length] = data_points[..] {
                    let source_range = source_start..source_start + length;
                    let destination_range = destination_start..destination_start + length;

                    return RangeMapping {
                        source_range,
                        destination_range,
                    };
                } else {
                    panic!("Invalid input, mapping row does not consist of three numbers");
                }
            })
            .collect();
        Mapper {
            _name: name,
            range_mappings,
            decorated_mapper: next,
        }
    }

    pub fn find_corresponding_value(&self, key: usize) -> usize {
        let mut mapped_value = None;

        for range_mapping in self.range_mappings.iter() {
            if let Some(matched_value) = range_mapping.source_to_destination(key) {
                mapped_value = Some(matched_value);
                break;
            }
        }

        let mapped_value = mapped_value.unwrap_or(key);

        self.decorated_mapper
            .as_ref()
            .map(|map| map.find_corresponding_value(mapped_value))
            .unwrap_or(mapped_value)
    }
}

pub fn solve_with_seeds_and_maps(seeds: Vec<Range<usize>>, maps: Vec<Vec<String>>) -> usize {
    let first_map: Box<Mapper> = maps
        .iter()
        .fold(None, |previous, map| {
            if let [heading, data @ ..] = &map[..] {
                let name: Token<String> = parse_token_value_before(heading, "map", "", " ")
                    .expect("missing heading for map");
                return Some(Box::new(Mapper::from_lines_and_next(
                    name.1,
                    data.to_owned()
                        .iter()
                        .map(|line| String::from(line.as_str()))
                        .collect(),
                    previous,
                )));
            }

            panic!("Invalid map, insufficient lines given");
        })
        .expect("Invalid input, should have at least one map");

    println!("{:?}", first_map);

    let mut results = vec![];

    seeds.iter().for_each(|seed_range| {
        for seed in seed_range.start.clone()..seed_range.end.clone() {
            results.push(first_map.find_corresponding_value(seed))
        }
    });

    results.iter().min().unwrap_or(&0).clone()
}

pub fn solve_a<'a>(path: &str) -> usize {
    let range_modified = 1usize;
    let lines = file_to_lines(path);

    let seed_line = lines
        .iter()
        .take(1)
        .last()
        .expect("invalid input, does not have a single line");
    let seeds: Vec<usize> = seed_line
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|seed| {
            seed.parse::<usize>()
                .expect("Invalid seed, should be number")
        })
        .collect();

    let seeds: Vec<Range<usize>> = seeds.iter()
        .map(|seed| {
            seed.clone()..(seed + &1)
        })
        .collect();

    let maps = maps_from_lines(lines);

    solve_with_seeds_and_maps(seeds, maps)
}

pub fn solve_b(path: &str) -> usize {
    let lines = file_to_lines(path);

    let seed_line = lines
        .iter()
        .take(1)
        .last()
        .expect("invalid input, does not have a single line");
    let seed_definitions: Vec<usize> = seed_line
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|seed| {
            seed.parse::<usize>()
                .expect("Invalid seed, should be number")
        })
        .collect();

    let seeds: Vec<Range<usize>> = seed_definitions.chunks(2).map(|chunk| {
        if let [start, length] = chunk {
            return (start.clone()..(start + length)).clone();
        }

        panic!("Invalid input. seed ranges do not match")
    }).collect();


    let maps = maps_from_lines(lines);

    solve_with_seeds_and_maps(seeds, maps)
}

fn maps_from_lines(lines: Vec<String>) -> Vec<Vec<String>> {
    let remaining: Vec<String> = lines.iter().skip(1).map(|line| line.clone()).collect();
    let mut maps = chunks_by(remaining, "".to_string());
    maps.reverse();
    maps
}

fn chunks_by(vec: Vec<String>, delimiter: String) -> Vec<Vec<String>> {
    let mut result = vec![];
    let mut chunk = vec![];

    vec.iter().for_each(|item| {
        if item == &delimiter {
            if !chunk.is_empty() {
                result.push(chunk.clone());
                chunk = vec![];
            }
        } else {
            chunk.push(item.clone());
        }
    });

    if !chunk.is_empty() {
        result.push(chunk.clone());
    }

    result
}
