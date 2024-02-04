use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map,
    multi::many1,
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};

use crate::{Result, Solution};

pub fn solution() -> Result<Solution<u64>> {
    let input = include_str!("../input/6.txt");

    let part_1 = parse_records(input)
        .iter()
        .map(|record| {
            let (min_time_held, max_time_held) = roots(record.time, record.distance);
            max_time_held - min_time_held + 1
        })
        .product();

    let record = parse_record(input);

    let part_2 = roots(record.time, record.distance);
    let part_2 = part_2.1 - part_2.0 + 1;

    Ok(Solution {
        day: 6,
        part_1,
        part_2,
    })
}

fn roots(time: u64, distance: u64) -> (u64, u64) {
    // t^2 - tT + D = 0
    // Complete the square
    let time_over_2 = time as f64 / 2_f64;
    let distance = distance as f64;
    let root = f64::sqrt(f64::powi(time_over_2, 2) - distance);
    (
        f64::ceil(-root + time_over_2) as u64,
        f64::floor(root + time_over_2) as u64,
    )
}

#[derive(Debug)]
struct Record {
    time: u64,
    distance: u64,
}

fn parse_record(input: &str) -> Record {
    let (_, record) = map(times_and_distances, |(times, distances)| {
        let time = times.join("").parse().unwrap();
        let distance = distances.join("").parse().unwrap();
        Record { time, distance }
    })(input)
    .finish()
    .unwrap();
    record
}

fn parse_records(input: &str) -> Vec<Record> {
    let (_, records) = map(times_and_distances, |(times, distances)| {
        times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Record {
                time: time.parse().unwrap(),
                distance: distance.parse().unwrap(),
            })
            .collect_vec()
    })(input)
    .finish()
    .unwrap();
    records
}

fn times_and_distances(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    tuple((terminated(times, newline), distances))(input)
}

fn distances(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(tag("Distance:"), numbers)(input)
}

fn times(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(tag("Time:"), numbers)(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<&str>> {
    many1(preceded(space1, digit1))(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> Result<()> {
        assert_eq!(
            solution()?,
            Solution {
                day: 6,
                part_1: 0,
                part_2: 0
            }
        );
        Ok(())
    }

    #[test]
    fn test_roots() {
        assert_eq!(roots(7, 9), (2, 5))
    }

    #[test]
    fn test_times_and_distances() {
        assert_eq!()
    }
}
