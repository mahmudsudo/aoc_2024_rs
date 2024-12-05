use std::collections::{HashMap, HashSet, VecDeque};

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let parts: Vec<_> = input.trim().split("\n\n").collect();
    
    // Parse rules
    let rules: Vec<(u32, u32)> = parts[0]
        .lines()
        .map(|line| {
            let nums: Vec<u32> = line.split('|')
                .map(|n| n.parse().unwrap())
                .collect();
            (nums[0], nums[1])
        })
        .collect();
    
    // Parse updates
    let updates: Vec<Vec<u32>> = parts[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    
    (rules, updates)
}

fn is_valid_order(update: &[u32], rules: &[(u32, u32)]) -> bool {
    let positions: HashMap<u32, usize> = update
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i))
        .collect();
    
    // Check each applicable rule
    for &(before, after) in rules {
        // Only check rules where both pages are in this update
        if let (Some(&pos_before), Some(&pos_after)) = (positions.get(&before), positions.get(&after)) {
            if pos_before > pos_after {
                return false;
            }
        }
    }
    
    true
}
fn topological_sort(pages: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();
    let pages_set: HashSet<_> = pages.iter().copied().collect();
    
    // Initialize in-degree counts to 0
    for &page in pages {
        in_degree.insert(page, 0);
    }
    
    // Build graph and count incoming edges
    for &(before, after) in rules {
        if pages_set.contains(&before) && pages_set.contains(&after) {
            graph.entry(before).or_default().push(after);
            *in_degree.entry(after).or_default() += 1;
        }
    }
    
    // Start with nodes that have no dependencies
    let mut queue: VecDeque<u32> = in_degree
        .iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(&page, _)| page)
        .collect();
    
    let mut result = Vec::new();
    
    // Process queue
    while let Some(page) = queue.pop_front() {
        result.push(page);
        
        if let Some(neighbors) = graph.get(&page) {
            for &next in neighbors {
                *in_degree.get_mut(&next).unwrap() -= 1;
                if in_degree[&next] == 0 {
                    queue.push_back(next);
                }
            }
        }
    }
    
    // If we couldn't order all pages (cycle in graph), return original order
    if result.len() != pages.len() {
        pages.to_vec()
    } else {
        result
    }
}
pub fn solve_part1() -> u32 {
    let (rules, updates) = parse_input(include_str!("../inputs/day5.txt"));
    
    updates.iter()
        .filter(|update| is_valid_order(update, &rules))
        .map(|update| {
            // Get middle page number
            update[update.len() / 2]
        })
        .sum()
}

pub fn solve_part2() -> u32 {
    let (rules, updates) = parse_input(include_str!("../inputs/day5.txt"));
    
    updates.iter()
        .filter(|update| !is_valid_order(update, &rules))
        .map(|update| {
            let sorted = topological_sort(update, &rules);
            sorted[sorted.len() / 2]
        })
        .sum()
}