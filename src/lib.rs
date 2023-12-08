pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(PartialEq, Eq, Debug)]
pub struct Solution<T: Display> {
    day: u8,
    part_1: T,
    part_2: T,
}

impl <T: Display> Display for Solution<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "~~~~~~~~~~~~~~~ Day {} ~~~~~~~~~~~~~~~",
            self.day as isize
        )?;
        writeln!(f, "Part 1: {}", self.part_1)?;
        writeln!(f, "Part 2: {}", self.part_2)?;
        Ok(())
    }
}

// Load lines from a file
pub fn input_lines(day: u8) -> Result<Lines<BufReader<File>>> {
    let file = File::open(format!("./input/{day}.txt"))?;
    Ok(BufReader::new(file).lines())
}
