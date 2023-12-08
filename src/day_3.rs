use std::collections::HashMap;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::opt, IResult,
};

use crate::{Result, Solution};

trait HasRow {
    fn row(&self) -> i32;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Symbol {
    is_asterisk: bool,
    row: i32,
    col: i32,
}

impl HasRow for Symbol {
    fn row(&self) -> i32 {
        self.row
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Number {
    value: i32,
    row: i32,
    col: i32,
}

impl HasRow for Number {
    fn row(&self) -> i32 {
        self.row
    }
}

type Schematic = (Vec<Number>, Vec<Symbol>);

pub fn solution() -> Result<Solution<i32>> {
    let input = include_str!("../input/3.txt");
    let (numbers, symbols) = parse_schematic(input)?.1;

    let num_index = build_index(&numbers);
    let sym_index = build_index(&symbols);

    let part_1: i32 = numbers
        .iter()
        .filter_map(|number| {
            if !adjacent_symbols(&sym_index, number).is_empty() {
                Some(number.value)
            } else {
                None
            }
        })
        .sum();

    let part_2: i32 = symbols
        .iter()
        .filter_map(|symbol| {
            let adjacent_numbers = adjacent_numbers(&num_index, symbol);
            if symbol.is_asterisk && adjacent_numbers.len() == 2 {
                Some(adjacent_numbers[0].value * adjacent_numbers[1].value)
            } else {
                None
            }
        })
        .sum();

    Ok(Solution {
        day: 3,
        part_1,
        part_2,
    })
}

fn build_index<T: HasRow>(items: &[T]) -> RowIndex<'_, T> {
    items.iter().fold(RowIndex::new(), |mut index, item| {
        index.insert(item.row(), item);
        index
    })
}

fn adjacent_symbols<'a>(
    symbol_index: &'a RowIndex<'a, Symbol>,
    number: &Number,
) -> Vec<&'a Symbol> {
    let above_row = number.row - 1;
    let below_row = number.row + 1;
    let left_col = number.col - 1;
    let right_col = number.col + num_digits(number.value);
    symbol_index
        .get_items_from_rows(above_row, below_row)
        .into_iter()
        .filter(|symbol| symbol.col >= left_col && symbol.col <= right_col)
        .collect()
}

fn adjacent_numbers<'a>(number_index: &'a RowIndex<Number>, symbol: &Symbol) -> Vec<&'a Number> {
    let above_row = symbol.row - 1;
    let below_row = symbol.row + 1;
    let left_col = symbol.col - 1;
    let right_col = symbol.col + 1;
    number_index
        .get_items_from_rows(above_row, below_row)
        .into_iter()
        .filter(|number| {
            let number_left_col = number.col;
            let number_right_col = number.col + num_digits(number.value) - 1;
            number_right_col >= left_col && number_left_col <= right_col
        })
        .collect()
}

#[derive(Clone, Debug)]
struct RowIndex<'a, T> {
    row_index: HashMap<i32, Vec<&'a T>>,
}

impl<'a, T> RowIndex<'a, T> {
    fn new() -> Self {
        Self {
            row_index: HashMap::new(),
        }
    }

    fn insert(&mut self, row: i32, item: &'a T) {
        self.row_index.entry(row).or_default().push(item);
    }

    fn get_items_from_rows(&self, row_start: i32, row_end: i32) -> Vec<&T> {
        let mut items = vec![];
        for row in row_start..=row_end {
            if let Some(items_in_row) = self.row_index.get(&row) {
                items.extend(items_in_row);
            }
        }
        items
    }
}

fn num_digits(number: i32) -> i32 {
    let mut num_digits = 0;
    let mut number = number;
    if number == 0 {
        return 1;
    }
    while number > 0 {
        number /= 10;
        num_digits += 1;
    }
    num_digits
}

fn symbol(input: &str) -> IResult<&str, &str> {
    alt((
        tag("@"),
        tag("#"),
        tag("$"),
        tag("%"),
        tag("&"),
        tag("*"),
        tag("-"),
        tag("+"),
        tag("="),
        tag("/"),
    ))(input)
}

fn number_or_symbol(input: &str) -> IResult<&str, &str> {
    alt((digit1, symbol))(input)
}

fn parse_schematic(input: &str) -> IResult<&str, Schematic> {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let items_and_cols = find_all_indexed(number_or_symbol)(line)?.1;
        for (item, col) in items_and_cols {
            if let Ok(value) = item.parse::<i32>() {
                numbers.push(Number {
                    value,
                    row: row as i32,
                    col: col as i32,
                });
            } else {
                symbols.push(Symbol {
                    is_asterisk: item == "*",
                    row: row as i32,
                    col: col as i32,
                });
            }
        }
    }
    Ok(("", (numbers, symbols)))
}

fn find_all_indexed<'a, T>(
    parser: impl Fn(&'a str) -> IResult<&str, T>,
) -> impl FnMut(&'a str) -> IResult<&str, Vec<(T, usize)>>
where
{
    move |mut input| {
        let mut matches = vec![];
        let mut index = 0;

        while let Ok((remainder, maybe_match)) = opt(&parser)(input) {
            if let Some(match_) = maybe_match {
                matches.push((match_, index));
                index += input.len() - remainder.len();
                input = remainder;
            } else {
                if remainder.is_empty() {
                    break;
                }
                index += 1;
                input = &remainder[1..];
            }
        }
        Ok(("", matches))
    }
}

#[cfg(test)]
mod tests {
    use itertools::enumerate;

    use super::*;

    #[test]
    fn test_find_all_indexed() -> Result<()> {
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let output = [
            vec![("467", 0), ("114", 5)],
            vec![("*", 3)],
            vec![("35", 2), ("633", 6)],
            vec![("#", 6)],
            vec![("617", 0), ("*", 3)],
            vec![("+", 5), ("58", 7)],
            vec![("592", 2)],
            vec![("755", 6)],
            vec![("$", 3), ("*", 5)],
            vec![("664", 1), ("598", 5)],
        ];

        let mut parser = find_all_indexed(number_or_symbol);
        for (index, line) in input.iter().enumerate() {
            let (remainder, matches) = parser(line)?;
            assert_eq!(remainder, "");
            assert_eq!(matches, output[index]);
        }
        Ok(())
    }

    #[test]
    fn test_parse_schematic() {
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .join("\n");
        let (numbers, symbols) = parse_schematic(&input).unwrap().1;

        assert_eq!(numbers.len(), 10);
        assert_eq!(symbols.len(), 6);
    }

    #[test]
    fn test_num_digits() {
        let inputs = vec![0, 1, 10, 100, 1000, 10000];
        let outputs = vec![1, 1, 2, 3, 4, 5];
        for (input, output) in inputs.into_iter().zip(outputs) {
            assert_eq!(num_digits(input), output);
        }
    }

    #[test]
    fn test_adjacent_symbols() {
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .join("\n");
        let (numbers, symbols) = parse_schematic(&input).unwrap().1;
        let sym_index = build_index(&symbols);

        let expected = [
            vec![&symbols[0]], // 467, *
            vec![],            // 114
            vec![&symbols[0]], // 35, *
            vec![&symbols[1]], // 633, #
            vec![&symbols[2]], // 617, *
            vec![],            // 58
            vec![&symbols[3]], // 592, +
            vec![&symbols[5]], // 755, *
            vec![&symbols[4]], // 664, $
            vec![&symbols[5]], // 598, *
        ];
        for (index, number) in enumerate(&numbers) {
            assert_eq!(adjacent_symbols(&sym_index, number), expected[index]);
        }
    }

    #[test]
    fn test_adjacent_numbers() {
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .join("\n");
        let (numbers, symbols) = parse_schematic(&input).unwrap().1;
        let num_index = build_index(&numbers);

        let expected = [
            vec![&numbers[0], &numbers[2]], // *
            vec![&numbers[3]],              // #
            vec![&numbers[4]],              // *
            vec![&numbers[6]],              // +
            vec![&numbers[8]],              // $
            vec![&numbers[7], &numbers[9]], // *
        ];
        for (index, symbol) in enumerate(&symbols) {
            assert_eq!(adjacent_numbers(&num_index, symbol), expected[index]);
        }
    }

    #[test]
    fn test_solution() -> Result<()> {
        assert_eq!(
            solution()?,
            Solution {
                day: 3,
                part_1: 520135,
                part_2: 72514855,
            }
        );
        Ok(())
    }
}
