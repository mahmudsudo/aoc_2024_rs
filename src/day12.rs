use std::collections::{HashSet, VecDeque};

type Point = (i32, i32);

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const CELL_SIZE: usize = 3;

struct Region {
    idx: char,
    positions: HashSet<Point>,
    perimeter: i32,
    corners: i32,
}

impl Region {
    fn new(idx: char, start_position: Point) -> Self {
        let mut positions = HashSet::new();
        positions.insert(start_position);
        Self {
            idx,
            positions,
            perimeter: 0,
            corners: 0,
        }
    }

    fn price(&self) -> i32 {
        self.positions.len() as i32 * self.perimeter
    }

    fn side_price(&self) -> i32 {
        self.positions.len() as i32 * self.corners
    }
}

pub struct Field {
    field_map: Vec<Vec<char>>,
    regions: Vec<Region>,
    rows: i32,
    cols: i32,
}

impl Field {
   pub fn new(field_map: Vec<Vec<char>>) -> Self {
        let rows = field_map.len() as i32;
        let cols = field_map[0].len() as i32;
        let mut field = Self {
            field_map,
            regions: Vec::new(),
            rows,
            cols,
        };
        
        let mut remaining: HashSet<Point> = (0..rows)
            .flat_map(|r| (0..cols).map(move |c| (r, c)))
            .collect();

        while !remaining.is_empty() {
            let &position = remaining.iter().next().unwrap();
            remaining.remove(&position);
            field.fill_region(position, &mut remaining);
        }
        
        field
    }

    fn fill_region(&mut self, start: Point, remaining: &mut HashSet<Point>) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(start);
        let idx = self.field_map[start.0 as usize][start.1 as usize];
        let mut region = Region::new(idx, start);

        while let Some(position) = queue.pop_front() {
            if !visited.insert(position) {
                continue;
            }

            region.corners += self.count_corners(position);

            for &(dx, dy) in &DIRECTIONS {
                let next = (position.0 + dx, position.1 + dy);
                
                if self.out_of_bounds(next) {
                    region.perimeter += 1;
                    continue;
                }

                let next_idx = self.field_map[next.0 as usize][next.1 as usize];
                if next_idx == idx {
                    if !visited.contains(&next) {
                        queue.push_back(next);
                        region.positions.insert(next);
                    }
                } else {
                    region.perimeter += 1;
                }
            }
        }

        for &pos in &region.positions {
            remaining.remove(&pos);
        }
        self.regions.push(region);
    }

    fn neighbourhood(&self, position: Point) -> Vec<Vec<i32>> {
        let mut neighbourhood = vec![vec![0; CELL_SIZE]; CELL_SIZE];
        let idx = self.field_map[position.0 as usize][position.1 as usize];

        for row_index in 0..CELL_SIZE {
            for col_index in 0..CELL_SIZE {
                let row = position.0 - 1 + row_index as i32;
                let col = position.1 - 1 + col_index as i32;
                
                if !self.out_of_bounds((row, col)) {
                    neighbourhood[row_index][col_index] = 
                        (self.field_map[row as usize][col as usize] == idx) as i32;
                }
            }
        }

        neighbourhood
    }

    fn count_corners(&self, position: Point) -> i32 {
        let n = self.neighbourhood(position);
        let mut count = 0;

        // Top-left corner patterns
        if n[1][0] == 0 && n[0][1] == 0 { count += 1; }
        if n[0][0] == 0 && n[1][0] == 1 && n[0][1] == 1 { count += 1; }

        // Top-right corner patterns
        if n[1][2] == 0 && n[0][1] == 0 { count += 1; }
        if n[0][2] == 0 && n[1][2] == 1 && n[0][1] == 1 { count += 1; }

        // Bottom-left corner patterns
        if n[1][0] == 0 && n[2][1] == 0 { count += 1; }
        if n[2][0] == 0 && n[1][0] == 1 && n[2][1] == 1 { count += 1; }

        // Bottom-right corner patterns
        if n[1][2] == 0 && n[2][1] == 0 { count += 1; }
        if n[2][2] == 0 && n[1][2] == 1 && n[2][1] == 1 { count += 1; }

        count
    }

    fn out_of_bounds(&self, position: Point) -> bool {
        position.0 < 0 || position.0 >= self.rows || position.1 < 0 || position.1 >= self.cols
    }

   pub fn price(&self) -> i32 {
        self.regions.iter().map(|region| region.price()).sum()
    }

    pub fn bulk_price(&self) -> i32 {
        self.regions.iter().map(|region| region.side_price()).sum()
    }
}
