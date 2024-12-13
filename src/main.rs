
use day13::*;
use std::io::{self};

use std::time::Instant;


mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
fn main() -> io::Result<()> {

    let mut input = read_input("day13.txt")?;
    
    // Part 1
    let start = Instant::now();
    let result = solve_part1(&input);
    elapsed_time("Part 1", result, start);

    // Part 2
    let start = Instant::now();
    let result = solve_part2(&mut input);
    elapsed_time("Part 2", result, start);

    Ok(())
}