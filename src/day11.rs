use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct CacheKey {
    stone: u64,
    n: usize,
}

fn blink(stone: u64, n: usize, cache: &mut HashMap<CacheKey, usize>) -> usize {
    if n == 0 {
        return 1;
    }
    
    let key = CacheKey { stone, n };
    if let Some(&result) = cache.get(&key) {
        return result;
    }
    
    let result = if stone == 0 {
        blink(1, n - 1, cache)
    } else {
        let str_stone = stone.to_string();
        if str_stone.len() % 2 == 0 {
            let mid = str_stone.len() / 2;
            let left = str_stone[..mid].parse::<u64>().unwrap();
            let right = str_stone[mid..].parse::<u64>().unwrap();
            blink(left, n - 1, cache) + blink(right, n - 1, cache)
        } else {
            blink(stone * 2024, n - 1, cache)
        }
    };
    
    cache.insert(key, result);
    result
}
fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

pub fn solve_part1() -> usize {
   let input = parse_input(include_str!("../inputs/day11.txt"));
    let mut cache = HashMap::new();
    let mut total = 0;
    
    for stone in input {
        total += blink(stone, 75, &mut cache);
    }
    
    total
}
