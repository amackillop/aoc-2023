use aoc_2023::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short)]
    day: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.day {
        1 => println!("{}", aoc_2023::day_1::solution()?),
        2 => println!("{}", aoc_2023::day_2::solution()?),
        3 => println!("{}", aoc_2023::day_3::solution()?),
        4 => println!("{}", aoc_2023::day_4::solution()?),
        5 => println!("{}", aoc_2023::day_5::solution()?),
        6 => println!("{}", aoc_2023::day_6::solution()?),
        _ => println!("Day {} not implemented", args.day),
    }
    Ok(())
}
