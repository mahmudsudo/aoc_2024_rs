use std::collections::HashSet;

const  Input :&str = include_str!("../inputs/day4.txt");


fn count_xmas(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Check all possible directions
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down-right
        (-1, 1),  // diagonal up-right
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // diagonal up-left
        (1, -1),  // diagonal down-left
    ];

    for i in 0..rows {
        for j in 0..cols {
            for &(di, dj) in &directions {
                if check_xmas(&grid, i, j, di, dj) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_xmas(grid: &[Vec<char>], i: usize, j: usize, di: i32, dj: i32) -> bool {
    let target = ['X', 'M', 'A', 'S'];
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    for k in 0..4 {
        let ni = i as i32 + di * k;
        let nj = j as i32 + dj * k;
        
        if ni < 0 || ni >= rows || nj < 0 || nj >= cols {
            return false;
        }
        
        if grid[ni as usize][nj as usize] != target[k as usize] {
            return false;
        }
    }
    true
}

fn count_x_mas(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 1..rows-1 {
        for j in 1..cols-1 {
            if check_x_mas(&grid, i, j) {
                count += 1;
            }
        }
    }

    count
}

fn check_x_mas(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    // Check for MAS in both directions forming an X
    let arms = [
        [(i-1, j-1), (i, j), (i+1, j+1)],  // top-left to bottom-right
        [(i-1, j+1), (i, j), (i+1, j-1)]   // top-right to bottom-left
    ];

    let mut found_mas = 0;
    let target = ['M', 'A', 'S'];

    for arm in &arms {
        // Check forwards
        if arm.iter().enumerate().all(|(k, &(r, c))| grid[r][c] == target[k]) {
            found_mas += 1;
        }
        // Check backwards
        if arm.iter().enumerate().all(|(k, &(r, c))| grid[r][c] == target[2-k]) {
            found_mas += 1;
        }
    }

    found_mas == 2
}
pub fn solve_part1() -> usize {
    count_xmas(Input)
}
pub fn solve_part2() -> usize {
    count_x_mas(Input)
}
