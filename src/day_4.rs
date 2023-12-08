use std::{collections::HashSet, iter::repeat};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, space1},
    combinator::map,
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::{Result, Solution};

pub fn solution() -> Result<Solution<i32>> {
    let input = include_str!("../input/4.txt");

    let part_1 = input
        .lines()
        .map(|line| {
            let (_, card) = parse_card(line).unwrap();
            compute_score(&card)
        })
        .sum();

    let part_2 = part_2(input)?;

    Ok(Solution {
        day: 4,
        part_1,
        part_2,
    })
}

fn part_2(input: &str) -> Result<i32> {
    let mut counts: Box<dyn Iterator<Item = i32>> = Box::new(repeat(1_i32));
    let mut total = 0;
    for line in input.lines() {
        let (_, card) = parse_card(line).unwrap();
        let matches = num_matches(&card);
        let count = counts.next().unwrap();

        let new_counts = (&mut counts).take(matches).map(|m| m + count).collect_vec();
        counts = Box::new(new_counts.into_iter().chain(counts));
        total += count;
    }

    Ok(total)
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    your_numbers: HashSet<i32>,
}

fn compute_score(card: &Card) -> i32 {
    let matches = num_matches(card);
    if matches == 0 {
        0
    } else {
        2_i32.pow((matches - 1) as u32)
    }
}

fn num_matches(card: &Card) -> usize {
    card.winning_numbers
        .intersection(&card.your_numbers)
        .count()
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map(
        tuple((
            tag("Card"),
            space1,
            i32,
            tag(":"),
            numbers,
            tag(" |"),
            numbers,
        )),
        |(_, _, id, _, winning_numbers, _, your_numbers)| Card {
            id,
            winning_numbers: winning_numbers.into_iter().collect(),
            your_numbers: your_numbers.clone().into_iter().collect(),
        },
    )(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<i32>> {
    many1(preceded(space1, i32))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers() -> Result<()> {
        let input = " 1 21 53 59 44";
        let (_, numbers) = numbers(input)?;
        assert_eq!(numbers, vec![1, 21, 53, 59, 44]);
        Ok(())
    }

    #[test]
    fn test_parse_card() -> Result<()> {
        let input = "Card  3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let (_, card) = parse_card(input)?;
        assert_eq!(
            card,
            Card {
                id: 3,
                winning_numbers: HashSet::from([1, 21, 53, 59, 44]),
                your_numbers: HashSet::from([69, 82, 63, 72, 16, 21, 14, 1]),
            }
        );
        Ok(())
    }

    #[test]
    fn test_num_matches() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let expected = [4, 2, 2, 1, 0, 0];
        for (input, expected) in input.iter().zip(expected) {
            let (_, card) = parse_card(input).unwrap();
            assert_eq!(num_matches(&card), expected);
        }
    }

    #[test]
    fn test_compute_score() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let expected = [8, 2, 2, 1, 0, 0];
        for (input, expected) in input.iter().zip(expected) {
            let (_, card) = parse_card(input).unwrap();
            assert_eq!(compute_score(&card), expected);
        }
    }

    #[test]
    fn test_solution() -> Result<()> {
        assert_eq!(solution()?, Solution { day: 4, part_1: 23847, part_2: 8570000 });
        Ok(())
    }
}
