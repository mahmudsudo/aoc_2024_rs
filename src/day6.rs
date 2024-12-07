use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_position(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }
}
fn solve_guard_patrol(map: &Vec<Vec<char>>) -> usize {
    // Find starting position and direction
    let (mut current_pos, mut current_dir) = find_starting_info(map);
    
    // Track visited positions
    let mut visited_positions = HashSet::new();
    visited_positions.insert(current_pos);

    // Safety counter to prevent infinite loop
    let mut iterations = 0;
    let max_iterations = map.len() * map[0].len() * 2; // Generous upper bound

    while iterations < max_iterations {
        // Check next position
        let next_pos = current_dir.move_position(current_pos);
        
        // Check if out of bounds
        if is_out_of_bounds(map, next_pos) {
            println!("Left map at: {:?}", current_pos);
            break;
        }

        // Check if blocked
        if is_blocked(map, next_pos) {
            // Turn right if blocked
            current_dir = current_dir.turn_right();
        } else {
            // Move forward
            current_pos = next_pos;
            visited_positions.insert(current_pos);
        }

        iterations += 1;
    }

    println!("Visited positions: {}", visited_positions.len());
    println!("Last position: {:?}", current_pos);
    println!("Last direction: {:?}", current_dir);

    visited_positions.len()
}

fn solve_guard_patrol_loop_positions(map: &Vec<Vec<char>>) -> usize {
    let (start_pos, start_dir) = find_starting_info(map);
    let mut loop_positions = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            // Skip start position and already blocked positions
            if (x as i32, y as i32) == start_pos || map[y][x] == '#' {
                continue;
            }

            // Try placing an obstruction
            let mut modified_map = map.clone();
            modified_map[y][x] = '#';

            // Check if this new obstruction creates a loop
            if creates_loop(&modified_map) {
                loop_positions += 1;
            }
        }
    }

    loop_positions
}

fn creates_loop(map: &Vec<Vec<char>>) -> bool {
    let (start_pos, start_dir) = find_starting_info(map);
    let mut visited_states = HashSet::new();
    let max_steps = map.len() * map[0].len() * 10; // Prevent truly infinite loops

    let mut current_pos = start_pos;
    let mut current_dir = start_dir;

    for _ in 0..max_steps {
        let state = (current_pos, current_dir);
        
        // If we've seen this exact state before, we're in a loop
        if visited_states.contains(&state) {
            return true;
        }
        visited_states.insert(state);

        // Check next position
        let next_pos = current_dir.move_position(current_pos);
        
        // Check if out of bounds
        if is_out_of_bounds(map, next_pos) {
            return false;
        }

        // Check if blocked
        if is_blocked(map, next_pos) {
            // Turn right if blocked
            current_dir = current_dir.turn_right();
        } else {
            // Move forward
            current_pos = next_pos;
        }
    }

    false
}

fn find_starting_info(map: &Vec<Vec<char>>) -> ((i32, i32), Direction) {
    for (y, row) in map.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            match ch {
                '^' => return ((x as i32, y as i32), Direction::Up),
                '>' => return ((x as i32, y as i32), Direction::Right),
                'v' => return ((x as i32, y as i32), Direction::Down),
                '<' => return ((x as i32, y as i32), Direction::Left),
                _ => continue,
            }
        }
    }
    panic!("No starting position found")
}

fn is_out_of_bounds(map: &Vec<Vec<char>>, (x, y): (i32, i32)) -> bool {
    x < 0 || y < 0 || x >= map[0].len() as i32 || y >= map.len() as i32
}

fn is_blocked(map: &Vec<Vec<char>>, (x, y): (i32, i32)) -> bool {
    map[y as usize][x as usize] == '#'
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn solve_part1() -> usize{
     solve_guard_patrol(&parse_input(include_str!("../inputs/day6.txt")))
}
pub fn solve_part2() -> usize{
    solve_guard_patrol_loop_positions(&parse_input(include_str!("../inputs/day6.txt")))
}