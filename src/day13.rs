use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
pub struct Machine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

pub fn read_input(filename: &str) -> io::Result<Vec<Machine>> {
    let path = Path::new("inputs").join(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut machines = Vec::new();
    let mut current_machine = None;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            if let Some(machine) = current_machine.take() {
                machines.push(machine);
            }
            continue;
        }

        if line.starts_with("Button A") {
            let coords: Vec<i64> = line.split(':')
                .nth(1)
                .unwrap()
                .split(',')
                .map(|v| v.split('+').nth(1).unwrap().trim().parse().unwrap())
                .collect();
            
            current_machine = Some(Machine {
                button_a: Point { x: coords[0], y: coords[1] },
                button_b: Point { x: 0, y: 0 },
                prize: Point { x: 0, y: 0 },
            });
        } else if line.starts_with("Button B") {
            let coords: Vec<i64> = line.split(':')
                .nth(1)
                .unwrap()
                .split(',')
                .map(|v| v.split('+').nth(1).unwrap().trim().parse().unwrap())
                .collect();
            
            if let Some(ref mut machine) = current_machine {
                machine.button_b = Point { x: coords[0], y: coords[1] };
            }
        } else if line.starts_with("Prize") {
            let coords: Vec<i64> = line.split(':')
                .nth(1)
                .unwrap()
                .split(',')
                .map(|v| v.split('=').nth(1).unwrap().trim().parse().unwrap())
                .collect();
            
            if let Some(ref mut machine) = current_machine {
                machine.prize = Point { x: coords[0], y: coords[1] };
            }
        }
    }

    // Don't forget the last machine if file doesn't end with empty line
    if let Some(machine) = current_machine {
        machines.push(machine);
    }

    Ok(machines)
}

pub fn solve_part1(input: &[Machine]) -> i64 {
    let mut cost = 0;
    
    for machine in input {
        // Calculate determinant
        let deter = machine.button_a.x * machine.button_b.y - machine.button_a.y * machine.button_b.x;
        if deter == 0 {
            continue;
        }

        // Calculate operations using Cramer's rule
        let op1 = machine.button_b.y * machine.prize.x - machine.button_b.x * machine.prize.y;
        let op2 = -machine.button_a.y * machine.prize.x + machine.button_a.x * machine.prize.y;

        // Check if solution exists (both operations should be divisible by determinant)
        if op1 % deter != 0 || op2 % deter != 0 {
            continue;
        }

        let n = op1 / deter;
        let m = op2 / deter;

        // Only count if both values are non-negative
        if n >= 0 && m >= 0 {
            cost += n * 3 + m;
        }
    }

    cost
}

pub fn solve_part2(input: &mut Vec<Machine>) -> i64 {
    // Add 10_000_000_000_000 to all prize coordinates
    for machine in input.iter_mut() {
        machine.prize.x += 10_000_000_000_000;
        machine.prize.y += 10_000_000_000_000;
    }
    
    solve_part1(input)
}

pub fn elapsed_time(name: &str, result: i64, start_time: Instant) {
    let elapsed = start_time.elapsed();
    let minutes = elapsed.as_secs() / 60;
    let seconds = elapsed.as_secs() % 60;
    let milliseconds = elapsed.subsec_millis();
    
    println!("{} time: {}:{}:{}", name, minutes, seconds, milliseconds);
    println!("{} result: {}", name, result);
}

