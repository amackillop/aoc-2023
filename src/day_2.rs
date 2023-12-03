use crate::Result;
use crate::Solution;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::combinator::map;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cube {
    Red,
    Green,
    Blue,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

impl Round {
    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Game {
    id: i32,
    rounds: Vec<Round>,
}

type CubeCount = Round;

impl Game {
    fn is_possible(&self, cube_count: &CubeCount) -> bool {
        self.rounds.iter().all(|round| {
            round.red <= cube_count.red
                && round.green <= cube_count.green
                && round.blue <= cube_count.blue
        })
    }

    fn min_cube_count(&self) -> CubeCount {
        self.rounds.iter().fold(
            Round {
                red: i32::MIN,
                green: i32::MIN,
                blue: i32::MIN,
            },
            |min_count, round| Round {
                red: min_count.red.max(round.red),
                green: min_count.green.max(round.green),
                blue: min_count.blue.max(round.blue),
            },
        )
    }
}

pub fn solution() -> Result<Solution<i32>> {
    let day = 2;
    let cube_counts = CubeCount {
        red: 12,
        green: 13,
        blue: 14,
    };

    let part_1 = crate::input_lines(day)?.fold_ok(0, |sum, line| {
        let (_, game) = game(&line).unwrap();
        if game.is_possible(&cube_counts) {
            sum + game.id
        } else {
            sum
        }
    })?;

    let part_2 = crate::input_lines(day)?.fold_ok(0, |sum, line| {
        let (_, game) = game(&line).unwrap();
        let min_cube_count = game.min_cube_count();
        sum + min_cube_count.power()
    })?;

    Ok(Solution {
        day: 2,
        part_1,
        part_2,
    })
}

fn cube(input: &str) -> IResult<&str, Cube> {
    alt((
        value(Cube::Red, tag("red")),
        value(Cube::Green, tag("green")),
        value(Cube::Blue, tag("blue")),
    ))(input)
}

fn cube_count(input: &str) -> IResult<&str, (Cube, i32)> {
    map(tuple((i32, tag(" "), cube)), |pair| (pair.2, pair.0))(input)
}

fn round(input: &str) -> IResult<&str, Round> {
    map(separated_list1(tag(", "), cube_count), |colour_counts| {
        colour_counts.iter().fold(
            Round {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut round, (colour, count)| {
                match colour {
                    Cube::Red => round.red += count,
                    Cube::Green => round.green += count,
                    Cube::Blue => round.blue += count,
                }
                round
            },
        )
    })(input)
}

fn rounds(input: &str) -> IResult<&str, Vec<Round>> {
    separated_list1(tag("; "), round)(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    map(
        tuple((tag("Game "), i32, tag(": "), rounds)),
        |(_, id, _, rounds)| Game { id, rounds },
    )(input)
}

mod tests {
    use super::*;

    #[test]
    fn test_colour() -> Result<()> {
        assert_eq!(cube("red")?.1, Cube::Red);
        assert_eq!(cube("green")?.1, Cube::Green);
        assert_eq!(cube("blue")?.1, Cube::Blue);
        Ok(())
    }

    #[test]
    fn test_colour_count() -> Result<()> {
        assert_eq!(cube_count("1 red")?.1, (Cube::Red, 1));
        assert_eq!(cube_count("2 green")?.1, (Cube::Green, 2));
        assert_eq!(cube_count("3 blue")?.1, (Cube::Blue, 3));
        Ok(())
    }

    #[test]
    fn test_round() -> Result<()> {
        assert_eq!(
            round("3 blue, 4 red")?.1,
            Round {
                red: 4,
                green: 0,
                blue: 3,
            }
        );
        assert_eq!(
            round("1 red, 2 green, 6 blue")?.1,
            Round {
                red: 1,
                green: 2,
                blue: 6,
            }
        );
        assert_eq!(
            round("2 green")?.1,
            Round {
                red: 0,
                green: 2,
                blue: 0,
            }
        );
        Ok(())
    }

    #[test]
    fn test_rounds() -> Result<()> {
        assert_eq!(
            rounds("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")?.1,
            vec![
                Round {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Round {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Round {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ]
        );
        Ok(())
    }

    #[test]
    fn test_game() -> Result<()> {
        assert_eq!(
            game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")?.1,
            Game {
                id: 1,
                rounds: vec![
                    Round {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Round {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Round {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ]
            }
        );
        Ok(())
    }

    #[test]
    fn test_game_is_possible() {
        let inputs = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let games: Vec<Game> = inputs
            .iter()
            .map(|input| game(input).unwrap().1)
            .collect::<Vec<Game>>();

        let cube_count = CubeCount {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert!(games[0].is_possible(&cube_count));
        assert!(games[1].is_possible(&cube_count));
        assert!(!games[2].is_possible(&cube_count));
        assert!(!games[3].is_possible(&cube_count));
        assert!(games[4].is_possible(&cube_count));
    }

    #[test]
    fn test_min_cube_count() {
        let inputs = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let games: Vec<Game> = inputs
            .iter()
            .map(|input| game(input).unwrap().1)
            .collect::<Vec<Game>>();

        assert_eq!(
            games[0].min_cube_count(),
            CubeCount {
                red: 4,
                green: 2,
                blue: 6
            }
        );
        assert_eq!(
            games[1].min_cube_count(),
            CubeCount {
                red: 1,
                green: 3,
                blue: 4
            }
        );
        assert_eq!(
            games[2].min_cube_count(),
            CubeCount {
                red: 20,
                green: 13,
                blue: 6
            }
        );
        assert_eq!(
            games[3].min_cube_count(),
            CubeCount {
                red: 14,
                green: 3,
                blue: 15
            }
        );
        assert_eq!(
            games[4].min_cube_count(),
            CubeCount {
                red: 6,
                green: 3,
                blue: 2
            }
        );
    }

    #[test]
    fn test_solution() -> Result<()> {
        assert_eq!(
            solution()?,
            Solution {
                day: 2,
                part_1: 2545,
                part_2: 78111
            }
        );
        Ok(())
    }
}
