use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn parse_antenna_map(input: &str) -> HashMap<char, Vec<Point>> {
    let mut frequencies: HashMap<char, Vec<Point>> = HashMap::new();
    
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                frequencies
                    .entry(ch)
                    .or_default()
                    .push(Point::new(x as i32, y as i32));
            }
        }
    }
    
    frequencies
}

fn get_antinode(near: Point, far: Point) -> Point {
    // Calculate the vector from near to far
    let dx = far.x - near.x;
    let dy = far.y - near.y;
    
    // The antinode is at near - (far - near) = 2*near - far
    Point::new(
        2 * near.x - far.x,
        2 * near.y - far.y
    )
}

fn calculate_antinodes(antennas: &[Point]) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    
    // For each pair of antennas
    for i in 0..antennas.len() {
        for j in (i + 1)..antennas.len() {
            let a1 = antennas[i];
            let a2 = antennas[j];
            
            // For each antenna being the "near" one
            // First antinode: a1 is near, a2 is far
            let antinode1 = get_antinode(a1, a2);
            
            // Second antinode: a2 is near, a1 is far
            let antinode2 = get_antinode(a2, a1);
            
            antinodes.insert(antinode1);
            antinodes.insert(antinode2);
        }
    }
    
    antinodes
}

fn count_antinodes_in_bounds(input: &str) -> usize {
    let frequencies = parse_antenna_map(input);
    
    // Get map dimensions
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;
    
    // Calculate all antinodes
    let mut all_antinodes = HashSet::new();
    
    for antennas in frequencies.values() {
        let antinodes = calculate_antinodes(antennas);
        // Only keep antinodes that are within bounds
        let valid_antinodes: HashSet<_> = antinodes
            .into_iter()
            .filter(|point| {
                point.x >= 0 && point.x < width && 
                point.y >= 0 && point.y < height
            })
            .collect();
            
        all_antinodes.extend(valid_antinodes);
    }
    
    all_antinodes.len()
}

fn is_collinear(p1: Point, p2: Point, p3: Point) -> bool {
    // Calculate cross product of vectors p1p2 and p1p3
    // If cross product is 0, points are collinear
    let cross_product = (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x);
    cross_product == 0
}

fn find_antinodes(antennas: &[Point], width: i32, height: i32) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    
    // If we have less than 2 antennas, there are no antinodes
    if antennas.len() < 2 {
        return antinodes;
    }

    // Check every point in bounds
    for y in 0..height {
        for x in 0..width {
            let point = Point::new(x, y);
            
            // For each point, check if it's collinear with at least two antennas
            for i in 0..antennas.len() {
                for j in (i + 1)..antennas.len() {
                    if is_collinear(point, antennas[i], antennas[j]) {
                        antinodes.insert(point);
                        break; // Found one collinear pair, no need to check more
                    }
                }
            }
        }
    }
    
    antinodes
}
fn count_antinodes_in_bounds2(input: &str) -> usize {
    let frequencies = parse_antenna_map(input);
    
    // Get map dimensions
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;
    
    // Calculate all antinodes
    let mut all_antinodes = HashSet::new();
    
    for antennas in frequencies.values() {
        let antinodes = find_antinodes(antennas, width, height);
        all_antinodes.extend(antinodes);
    }
    
    all_antinodes.len()
}

pub fn solve_part1() -> usize{
    count_antinodes_in_bounds(include_str!("../inputs/day8.txt"))
}
pub fn solve_part2() -> usize{
    count_antinodes_in_bounds2(include_str!("../inputs/day8.txt"))
}