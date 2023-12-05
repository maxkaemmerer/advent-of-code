use std::ops::{RangeInclusive};
use crate::common::file_to_lines;
use crate::tokens::{parse_token_value_before, Token};

#[derive(Debug)]
struct RangeMapping {
    source_range: RangeInclusive<usize>,
    destination_range: RangeInclusive<usize>,
}

impl RangeMapping {
    fn source_to_destination(&self, key: usize) -> Option<usize> {
        if self.source_range.contains(&key) {
            return Some(self.destination_range.start() + (key - self.source_range.start()));
        }

        None
    }

    fn destination_to_source(&self, key: usize) -> Option<usize> {
        if self.destination_range.contains(&key) {
            return Some(self.source_range.start() + (key - self.destination_range.start()));
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
        let mut range_mappings: Vec<RangeMapping> = lines
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
                    let source_range = RangeInclusive::new(source_start, source_start + length);
                    let destination_range = RangeInclusive::new(destination_start, destination_start + length);

                    return RangeMapping {
                        source_range,
                        destination_range,
                    };
                } else {
                    panic!("Invalid input, mapping row does not consist of three numbers");
                }
            })
            .collect();

        range_mappings.sort_by(|a, b| a.destination_range.start().partial_cmp(&b.destination_range.start()).expect("What?!"));

        Mapper {
            _name: name,
            range_mappings,
            decorated_mapper: next,
        }
    }

    pub fn find_corresponding_value(&self, key: usize) -> usize {
        println!("find {:?}", self._name);
        let mut mapped_value = None;

        for range_mapping in self.range_mappings.iter() {
            if let Some(matched_value) = range_mapping.source_to_destination(key) {
                // println!("find source {:?}", range_mapping.source_range);
                // println!("find destination {:?}", range_mapping.destination_range);
                // println!("forward {} key {} to {}", self._name, key, matched_value);
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


    pub fn reverse_find_corresponding_value(&self, key: usize) -> Option<usize> {
        let mut mapped_value = None;

        for range_mapping in self.range_mappings.iter() {
            if let Some(matched_value) = range_mapping.destination_to_source(key) {
                // println!("reverse {} key {} to {}", self._name, key, matched_value);
                mapped_value = Some(matched_value);
                break;
            }
        }

       if let (Some(decorated_mapper), Some(value)) = (self.decorated_mapper.as_ref(), mapped_value) {
            return decorated_mapper.reverse_find_corresponding_value(value);
        }

        if let Some(decorated_mapper) = self.decorated_mapper.as_ref() {
            return decorated_mapper.reverse_find_corresponding_value(key);
        }

        Some(key)
    }


    pub fn reverse_search_with_smallest_value(&self, seeds: Vec<RangeInclusive<usize>>, forward_mapper: Option<Box<Mapper>>) -> (usize, usize) {
        println!("{}", self._name);


        for range_mapping in self.range_mappings.iter() {
            for lowest_value in range_mapping.destination_range.clone() {
                if let Some(key) = self.reverse_find_corresponding_value(lowest_value) {
                    for seed in seeds.iter() {
                        let found = if let Some(mapper) = &forward_mapper {mapper.find_corresponding_value(key)} else { self.find_corresponding_value(key) };
                        if seed.contains(&key) && found == lowest_value {
                            // println!("reverse source {:?}", range_mapping.source_range);
                            // println!("reverse destination {:?}", range_mapping.destination_range);
                            // println!("{} lowest value {} key {} in seed {}..{}", self._name, lowest_value, key, seed.start(), seed.end());
                            return (key, lowest_value);
                        }
                    }
                }
            }
        }

        println!("nothing in {}", self._name);

        if let Some(a) = &self.decorated_mapper {
            return a.reverse_search_with_smallest_value(seeds, forward_mapper.and_then(|mapper| mapper.decorated_mapper));
        }

        panic!("Could not reverse map from smallest value");
    }
}

pub fn solve_with_seeds_and_maps(seeds: Vec<RangeInclusive<usize>>, maps: Vec<Vec<String>>) -> usize {
    let first_reverse_map: Box<Mapper> = maps
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


    let mut maps = maps.clone();
    maps.reverse();

    let first_forward_map: Option<Box<Mapper>> = maps
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
        });

    let result = first_reverse_map.reverse_search_with_smallest_value(seeds, first_forward_map);

    println!("seed {} location {}", result.0, result.1);
    result.1
}

pub fn solve_a<'a>(path: &str) -> usize {
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

    let seeds: Vec<RangeInclusive<usize>> = seeds.iter()
        .map(|seed| {
            RangeInclusive::new(seed.clone(), seed.clone())
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

    let seeds: Vec<RangeInclusive<usize>> = seed_definitions.chunks(2).map(|chunk| {
        if let [start, length] = chunk {
            return RangeInclusive::new(start.clone(), start + length);
        }

        panic!("Invalid input. seed ranges do not match")
    }).collect();


    let maps = maps_from_lines(lines);

    solve_with_seeds_and_maps(seeds, maps)
}

fn maps_from_lines(lines: Vec<String>) -> Vec<Vec<String>> {
    let remaining: Vec<String> = lines.iter().skip(1).map(|line| line.clone()).collect();
    let maps = chunks_by(remaining, "".to_string());
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
//
// #[cfg(test)]
// mod tests {
//     use std::ops::{Range, RangeInclusive};
//     use crate::solutions::aoc2023::five::{Mapper, RangeMapping};
//
//     #[test]
//     fn should_correctly_map_one_level() {
//         let location_mapper = Mapper {
//             _name: "location".to_string(),
//             range_mappings: vec![
//                 RangeMapping {
//                     source_range: RangeInclusive::new(1, 10),
//                     destination_range: RangeInclusive::new(11, 20),
//                 }
//             ],
//             decorated_mapper: None,
//         };
//
//         let seeds: Vec<RangeInclusive<usize>> = vec![RangeInclusive::new(7, 7), RangeInclusive::new(2, 2), RangeInclusive::new(32, 32)];
//
//         assert_eq!((2, 12), location_mapper.reverse_search_with_smallest_value(seeds));
//     }
//
//     #[test]
//     fn should_correctly_map_two_levels() {
//         let soil_mapper = Mapper {
//             _name: "soil".to_string(),
//             range_mappings: vec![
//                 RangeMapping {
//                     source_range: RangeInclusive::new(3, 10),
//                     destination_range: RangeInclusive::new(3, 15),
//                 }
//             ],
//             decorated_mapper: None,
//         };
//
//
//         let location_mapper = Mapper {
//             _name: "location".to_string(),
//             range_mappings: vec![
//                 RangeMapping {
//                     source_range: RangeInclusive::new(1, 10),
//                     destination_range: RangeInclusive::new(11, 20),
//                 }
//             ],
//             decorated_mapper: Some(Box::new(soil_mapper)),
//         };
//
//         let seeds: Vec<RangeInclusive<usize>> = vec![RangeInclusive::new(7, 7), RangeInclusive::new(2, 2), RangeInclusive::new(32, 32)];
//
//         assert_eq!((0, 0), location_mapper.reverse_search_with_smallest_value(seeds));
//     }
//
//     #[test]
//     fn should_reverse_map() {
//         let soil_mapper = Mapper::from_lines_and_next(
//             "location".to_string(),
//             vec![
//                 "3 3 7".to_string(),
//             ],
//             None
//         );
//
//         let location_mapper = Mapper::from_lines_and_next(
//             "location".to_string(),
//                   vec![
//                       "11 1 10".to_string(),
//                       "18 1 10".to_string(),
//                   ],
//             Some(Box::new(soil_mapper))
//         );
//
//         // not in the lowest level source range so None
//         // assert_eq!(None, location_mapper.reverse_find_corresponding_value(12));
//         // in the lowest level source range so 3
//         // assert_eq!(Some(3), location_mapper.reverse_find_corresponding_value(13));
//
//         // assert_eq!(Some(3), location_mapper.find_corresponding_value());
//
//         // the lowest value that can map up to a seed is 17
//         let seeds: Vec<RangeInclusive<usize>> = vec![RangeInclusive::new(7, 7), RangeInclusive::new(2, 2), RangeInclusive::new(32, 32)];
//         assert_eq!((7, 17), location_mapper.reverse_search_with_smallest_value(seeds));
//     }
// }
