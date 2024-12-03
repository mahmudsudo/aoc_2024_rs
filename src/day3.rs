use regex::Regex;

pub fn solve_part1() -> i32 {
    let input = include_str!("../inputs/day3.txt");
    // Create a regex pattern for valid mul instructions
    // Matches mul(X,Y) where X and Y are 1-3 digits
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    // Find all valid matches and sum their products
    re.captures_iter(input)
        .map(|cap| {
            // Parse the two numbers from each capture
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

#[derive(Debug)]
enum Instruction {
    Multiply(i32, i32),
    Do,
    Dont,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
   
    
    // Create regex patterns for all instruction types
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    
    // Find all matches and their positions
    let mut matches = vec![];
    
    // Find multiplication instructions
    for cap in mul_re.captures_iter(input) {
        let pos = cap.get(0).unwrap().start();
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        matches.push((pos, Instruction::Multiply(x, y)));
    }
    
    // Find do() instructions
    for mat in do_re.find_iter(input) {
        matches.push((mat.start(), Instruction::Do));
    }
    
    // Find don't() instructions
    for mat in dont_re.find_iter(input) {
        matches.push((mat.start(), Instruction::Dont));
    }
    
    // Sort by position to maintain order
    matches.sort_by_key(|&(pos, _)| pos);
    
    // Extract just the instructions in order
    matches.into_iter().map(|(_, instr)| instr).collect()
}

pub fn solve_part2() -> i64 {
    let instructions = parse_instructions(include_str!("../inputs/day3.txt"));
    let mut enabled = true;
    let mut sum = 0;
    
    for instruction in instructions {
        match instruction {
            Instruction::Multiply(x, y) if enabled => {
                sum += i64::from(x) * i64::from(y);
            }
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            _ => {} // Disabled multiplications
        }
    }
    
    sum
}