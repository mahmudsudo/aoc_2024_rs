use std::fs::read_to_string;

pub fn solve_part1() -> usize{
    read_to_string("inputs/day2.txt")
    .unwrap()
.lines()
.filter(|line| !line.trim().is_empty())
.map(|line| {
    // Parse numbers from the line
    let levels: Vec<i32> = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    
    is_safe(&levels)
})
.filter(|&safe| safe)
.count()
}


pub fn solve_part2() -> usize{
    read_to_string("inputs/day2.txt")
    .unwrap()
.lines()
.filter(|line| !line.trim().is_empty())
.map(|line| {
    // Parse numbers from the line
    let levels: Vec<i32> = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    
    is_safe_with_dampener(&levels)
})
.filter(|&safe| safe)
.count()
}
fn is_safe(levels: &Vec<i32>) -> bool {
    if levels.len() < 2 {
        return true;
    }

    // Check if sequence is increasing or decreasing
    let mut is_increasing = None;
    
    for i in 1..levels.len() {
        let diff = levels[i] - levels[i-1];
        
        // Difference must be between 1 and 3 (inclusive) for increasing
        // or between -3 and -1 (inclusive) for decreasing
        if diff == 0 || diff.abs() > 3 {
            return false;
        }
        
        match is_increasing {
            None => {
                is_increasing = Some(diff > 0);
            }
            Some(increasing) => {
                if (diff > 0) != increasing {
                    return false;
                }
            }
        }
    }
    
    true
}

fn is_safe_with_dampener(levels: &Vec<i32>) -> bool {
    // If it's already safe, no need to try removing elements
    if is_safe(levels) {
        return true;
    }
    
    // Try removing each element one at a time
    for i in 0..levels.len() {
        let mut modified_levels = levels.clone();
        modified_levels.remove(i);
        
        if is_safe(&modified_levels) {
            return true;
        }
    }
    
    false
}