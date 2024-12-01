use std::{collections::HashMap, fs::read_to_string};

pub fn solve_part1() -> i64 {
   let m = read_to_string("inputs/day1.txt")
    .unwrap();
    calculate_total_distance(&m)
   
}
pub fn solve_part2()-> i64 {
    let m = read_to_string("inputs/day1.txt")
    .unwrap();
    calculate_similarity_score(&m)
}
fn calculate_total_distance(input: &str) -> i64 {
    // Parse input into two vectors
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    
    // Process each line
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
            
        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }
    
    // Sort both lists
    left_list.sort_unstable();
    right_list.sort_unstable();
    
    // Calculate total distance
    left_list.iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum()
}


fn calculate_similarity_score(input: &str) -> i64 {
    // Parse input into two vectors
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    
    // Process each line
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
            
        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }
    
    // Count occurrences in right list
    let right_counts: HashMap<i64, i64> = right_list
        .iter()
        .fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });
    
    // Calculate similarity score
    left_list.iter()
        .map(|&num| num * right_counts.get(&num).copied().unwrap_or(0))
        .sum()
}
