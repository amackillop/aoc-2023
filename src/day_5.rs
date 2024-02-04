use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
    Finish, IResult,
};

use rayon::prelude::*;
use crate::{Result, Solution};

pub fn solution() -> Result<Solution<u64>> {
    let input = include_str!("../input/5.txt");

    let (remainder, seeds) = seeds(input)?;

    let almanac = Almanac::from(remainder);

    let part_1 = seeds
        .iter()
        .map(|seed| almanac.get_location(*seed))
        .min()
        .unwrap();

    let part_2: u64 = seeds
        .chunks(2)
        .map(|pair| {
            let [range_start, range_length] = pair else { panic!() };
            (*range_start..(*range_start+*range_length)).into_par_iter().map(|seed| {
                almanac.get_location(seed)
            }).min().unwrap()
        }).min().unwrap();

    Ok(Solution {
        day: 5,
        part_1,
        part_2,
    })
}

struct Almanac {
    maps: Vec<CategoryMap>,
}

impl Almanac {
    fn get_location(&self, seed: u64) -> u64 {
        self.maps.iter().fold(seed, |key, map| map.get(key))
    }
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let (_, almanac) = almanac_data(input).finish().unwrap();
        Self {
            maps: almanac
                .maps
                .into_iter()
                .map(|map_rows| CategoryMap { map_rows })
                .collect(),
        }
    }
}

struct CategoryMap {
    map_rows: MapRows,
}

impl CategoryMap {
    fn get(&self, source: u64) -> u64 {
        self.map_rows
            .iter()
            .find_map(|map_row| map_row.get_destination(source))
            .unwrap_or(source)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct MapRow {
    dest_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl MapRow {
    pub fn get_destination(&self, source: u64) -> Option<u64> {
        if self.source_range_start <= source
            && source <= self.source_range_start + self.range_length
        {
            Some(source + self.dest_range_start - self.source_range_start)
        } else {
            None
        }
    }
}

type MapRows = Vec<MapRow>;

struct AlmanacData {
    maps: Vec<MapRows>,
}

fn almanac_data(input: &str) -> IResult<&str, AlmanacData> {
    map(mappings, |maps| AlmanacData { maps })(input)
}

fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    terminated(preceded(tag("seeds: "), numbers), newlines)(input)
}

fn mappings(input: &str) -> IResult<&str, Vec<MapRows>> {
    separated_list1(newlines, mapping)(input)
}

fn mapping(input: &str) -> IResult<&str, MapRows> {
    preceded(map_name, separated_list1(newline, map_row))(input)
}

fn map_name(input: &str) -> IResult<&str, &str> {
    alt((
        tag("seed-to-soil map:\n"),
        tag("soil-to-fertilizer map:\n"),
        tag("fertilizer-to-water map:\n"),
        tag("water-to-light map:\n"),
        tag("light-to-temperature map:\n"),
        tag("temperature-to-humidity map:\n"),
        tag("humidity-to-location map:\n"),
    ))(input)
}

fn map_row(input: &str) -> IResult<&str, MapRow> {
    map(numbers, |numbers| {
        let mut it = numbers.into_iter();
        MapRow {
            dest_range_start: it.next().unwrap(),
            source_range_start: it.next().unwrap(),
            range_length: it.next().unwrap(),
        }
    })(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(char(' '), u64)(input)
}

fn newlines(input: &str) -> IResult<&str, Vec<char>> {
    many1(newline)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> Result<()> {
        assert_eq!(
            solution()?,
            Solution {
                day: 5,
                part_1: 535088217,
                part_2: 0
            }
        );
        Ok(())
    }

    #[test]
    fn test_category_map_get() {
        let map_rows = vec![
            MapRow {
                dest_range_start: 0,
                source_range_start: 15,
                range_length: 37,
            },
            MapRow {
                dest_range_start: 37,
                source_range_start: 52,
                range_length: 2,
            },
            MapRow {
                dest_range_start: 39,
                source_range_start: 0,
                range_length: 15,
            },
        ];
        let category_map = CategoryMap { map_rows };
        assert_eq!(category_map.get(0), 39);
        assert_eq!(category_map.get(14), 53);
        assert_eq!(category_map.get(15), 0);
        assert_eq!(category_map.get(52), 37);
        assert_eq!(category_map.get(54), 39);
        assert_eq!(category_map.get(55), 55);
    }

    #[test]
    fn test_mapping() {
        let input = ["soil-to-fertilizer map:", "0 15 37", "37 52 2", "39 0 15"].join("\n");

        let (_, map_row) = mapping(&input).unwrap();
        assert_eq!(
            map_row,
            vec![
                MapRow {
                    dest_range_start: 0,
                    source_range_start: 15,
                    range_length: 37
                },
                MapRow {
                    dest_range_start: 37,
                    source_range_start: 52,
                    range_length: 2
                },
                MapRow {
                    dest_range_start: 39,
                    source_range_start: 0,
                    range_length: 15
                }
            ]
        );
    }

    #[test]
    fn test_map_row_get_destination() {
        let map_row = MapRow {
            dest_range_start: 0,
            source_range_start: 15,
            range_length: 37,
        };
        assert_eq!(map_row.get_destination(14), None);
        assert_eq!(map_row.get_destination(15), Some(0));
        assert_eq!(map_row.get_destination(16), Some(1));
        assert_eq!(map_row.get_destination(51), Some(36));
        assert_eq!(map_row.get_destination(52), Some(37));
        assert_eq!(map_row.get_destination(53), None);
    }

    #[test]
    fn test_map_row() {
        let input = "50 98 2";
        let (_, map_row) = map_row(input).unwrap();
        assert_eq!(
            map_row,
            MapRow {
                dest_range_start: 50,
                source_range_start: 98,
                range_length: 2
            }
        );
    }
}
