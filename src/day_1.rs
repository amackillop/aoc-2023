use crate::{input_lines, Result, Solution};
use std::ops::Add;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take,
    character::complete::i32,
    combinator::value,
    combinator::{map_parser, opt},
    IResult,
};

pub fn solution() -> Result<Solution<i32>> {
    let day = 1;
    let part_1 = sum_of_calibration_values(day, part_1_parser)?;
    let part_2 = sum_of_calibration_values(day, part_2_parser)?;
    Ok(Solution {
        day,
        part_1,
        part_2,
    })
}

fn sum_of_calibration_values(day: u8, line_parser: impl Fn(&str) -> Vec<i32>) -> Result<i32> {
    input_lines(day)?
        .filter_map_ok(|line| {
            let digits: Vec<i32> = line_parser(&line);
            let first = digits.first()?;
            let last = digits.last().unwrap_or(first);
            Some(10 * first + last)
        })
        .fold_ok(0, Add::add)
        .map_err(Into::into)
}

fn part_1_parser(input: &str) -> Vec<i32> {
    find_all(single_digit)(input).unwrap().1
}

fn part_2_parser(input: &str) -> Vec<i32> {
    find_all(spelled_or_literal_digit)(input).unwrap().1
}

fn spelled_or_literal_digit(input: &str) -> IResult<&str, i32> {
    alt((spelled_digit, single_digit))(input)
}

fn single_digit(input: &str) -> IResult<&str, i32> {
    single(i32)(input)
}

fn spelled_digit(input: &str) -> IResult<&str, i32> {
    alt((
        value(0, tag("zero")),
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

pub fn find_all<'a, T>(
    parser: impl Fn(&'a str) -> IResult<&str, T>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<T>> {
    move |input| {
        let mut digits = vec![];
        let mut input = input;

        while let Ok((remainder, maybe_digit)) = opt(&parser)(input) {
            if let Some(digit) = maybe_digit {
                digits.push(digit);
            }
            if remainder.is_empty() {
                break;
            }
            input = &input[1..];
        }
        Ok(("", digits))
    }
}

pub fn single<'a, T>(
    parser: impl Fn(&'a str) -> IResult<&str, T>,
) -> impl FnMut(&'a str) -> IResult<&'a str, T> {
    map_parser(take(1_u8), parser)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_parser() {
        let inputs = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let outputs = vec![vec![1, 2], vec![3, 8], vec![1, 2, 3, 4, 5], vec![7]];
        for (input, output) in inputs.into_iter().zip(outputs.into_iter()) {
            assert_eq!(part_1_parser(input), output);
        }
    }

    #[test]
    fn test_part_2_parser() {
        let inputs = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
            "seven3oneightp",
        ];
        let outputs = vec![
            vec![2, 1, 9],
            vec![8, 2, 3],
            vec![1, 2, 3],
            vec![2, 1, 3, 4],
            vec![4, 9, 8, 7, 2],
            vec![1, 8, 2, 3, 4],
            vec![7, 6],
            vec![7, 3, 1, 8],
        ];
        for (input, output) in inputs.into_iter().zip(outputs.into_iter()) {
            assert_eq!(part_2_parser(input), output);
        }
    }

    #[test]
    fn test_solution() -> Result<()> {
        assert_eq!(
            solution()?,
            Solution {
                day: 1,
                part_1: 55477,
                part_2: 54431
            }
        );
        Ok(())
    }
}
