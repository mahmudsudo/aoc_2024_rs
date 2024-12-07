use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

fn concatenate_numbers(a: i64, b: i64) -> i64 {
    let b_digits = b.to_string();
    let combined = format!("{}{}", a, b_digits);
    combined.parse().unwrap()
}

fn evaluate_expression(numbers: &[i64], operators: &[char]) -> i64 {
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            '|' => result = concatenate_numbers(result, numbers[i + 1]),
            _ => panic!("Invalid operator"),
        }
    }
    result
}

fn generate_operator_combinations(length: usize) -> Vec<Vec<char>> {
    let ops = vec!['+', '*', '|'];
    let mut combinations = Vec::new();
    
    fn backtrack(
        current: &mut Vec<char>, 
        length: usize, 
        max_length: usize,
        ops: &[char], 
        combinations: &mut Vec<Vec<char>>
    ) {
        if current.len() == max_length {
            combinations.push(current.clone());
            return;
        }
        
        if current.len() < max_length {
            for &op in ops {
                current.push(op);
                backtrack(current, length, max_length, ops, combinations);
                current.pop();
            }
        }
    }
    
    let mut current = Vec::new();
    backtrack(&mut current, length, length - 1, &ops, &mut combinations);
    combinations
}


fn solve_equation(equation: &Equation) -> bool {
    let operator_combinations = generate_operator_combinations(equation.numbers.len());
    
    for ops in operator_combinations {
        let result = evaluate_expression(&equation.numbers, &ops);
        if result == equation.test_value {
            return true;
        }
    }
    
    false
}

fn parse_input(filename: &str) -> Vec<Equation> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    
    reader.lines()
        .map(|line| {
            let line = line.expect("Could not read line");
            let parts: Vec<&str> = line.split(": ").collect();
            let test_value: i64 = parts[0].parse().expect("Invalid test value");
            let numbers: Vec<i64> = parts[1].split_whitespace()
                .map(|num| num.parse().expect("Invalid number"))
                .collect();
            
            Equation { test_value, numbers }
        })
        .collect()
}
pub fn solve_part1()->i64{
    let equations = parse_input("inputs/day7.txt");
    
     equations.iter()
        .filter(|eq| solve_equation(eq))
        .map(|eq| eq.test_value)
        .sum()

        //solve_equation has been updated to fit part2 to reduce redundant code 
}

pub fn solve_part2()->i64{
    let equations = parse_input("inputs/day7.txt");
    
     equations.iter()
        .filter(|eq| solve_equation(eq))
        .map(|eq| eq.test_value)
        .sum()
}