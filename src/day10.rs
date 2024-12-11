use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn neighbors(&self) -> impl Iterator<Item = Point> + '_ {
        const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        DIRS.iter().map(|&(dx, dy)| Point::new(self.x + dx, self.y + dy))
    }
}

pub struct Grid {
    data: Vec<Vec<u8>>,
    height: i32,
    width: i32,
}

impl Grid {
   pub fn from_str(input: &str) -> Self {
        let data: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        let height = data.len() as i32;
        let width = data[0].len() as i32;
        Grid {
            data,
            height,
            width,
        }
    }

    fn get(&self, point: Point) -> Option<u8> {
        if point.x >= 0
            && point.y >= 0
            && point.x < self.width
            && point.y < self.height
        {
            Some(self.data[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn find_trailheads(&self) -> Vec<Point> {
        let mut trailheads = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point::new(x, y);
                if self.get(point) == Some(0) {
                    trailheads.push(point);
                }
            }
        }
        trailheads
    }

    fn get_valid_neighbors(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        let current_height = self.get(point).unwrap();
        point.neighbors().filter(move |&next| {
            self.get(next).map_or(false, |h| h == current_height + 1)
        }).collect::<Vec<_>>().into_iter()
    }

    fn find_reachable_nines(&self, start: Point) -> HashSet<Point> {
        let mut reachable_nines = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, HashSet::from([start])));

        while let Some((current, path)) = queue.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            if self.get(current) == Some(9) {
                reachable_nines.insert(current);
                continue;
            }

            for next in self.get_valid_neighbors(current) {
                if !path.contains(&next) {
                    let mut new_path = path.clone();
                    new_path.insert(next);
                    queue.push_back((next, new_path));
                }
            }
        }

        reachable_nines
    }
    fn count_distinct_paths(&self, start: Point) -> usize {
        // Track complete paths to avoid duplicates
        let mut paths = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(vec![start]);

        while let Some(current_path) = queue.pop_front() {
            let current = *current_path.last().unwrap();
            let current_height = self.get(current).unwrap();

            // If we've reached height 9, we've found a complete path
            if current_height == 9 {
                paths.insert(current_path);
                continue;
            }

            // For each valid next step
            for next in self.get_valid_neighbors(current) {
                if !current_path.contains(&next) {
                    let mut new_path = current_path.clone();
                    new_path.push(next);
                    queue.push_back(new_path);
                }
            }
        }

        paths.len()
    }

    pub fn solve_part1(&self) -> usize {
        self.find_trailheads()
            .into_iter()
            .map(|trailhead| self.find_reachable_nines(trailhead).len())
            .sum()
    }
    pub fn solve_part2(&self) -> usize {
        self.find_trailheads()
            .into_iter()
            .map(|trailhead| self.count_distinct_paths(trailhead))
            .sum()
    }
}