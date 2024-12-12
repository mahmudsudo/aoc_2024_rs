use day12::Field;

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
fn main() {
    let input = include_str!("../inputs/day12.txt").trim();
    let field_map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let field = Field::new(field_map);
    println!("The total price of the field is {}", field.price());
    println!("With the bulk discount, the price is {}", field.bulk_price());
}
