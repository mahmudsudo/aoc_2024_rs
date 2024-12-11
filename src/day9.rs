

fn compact_disk(disk: &mut Vec<Option<usize>>) {
    let len = disk.len();
    
    for target_pos in 0..len {
        if disk[target_pos].is_none() {
            // Find the rightmost file block
            if let Some(source_pos) = (target_pos..len).rev()
                .find(|&i| disk[i].is_some())
            {
                // Move the block
                disk[target_pos] = disk[source_pos];
                disk[source_pos] = None;
            } else {
                break;
            }
        }
    }
}


fn parse_disk_map(input: &str) -> Vec<u32> {
    input.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn expand_disk_map(lengths: &[u32]) -> Vec<Option<usize>> {
    let mut result = Vec::new();
    let mut file_id = 0;
    
    for (i, &length) in lengths.iter().enumerate() {
        for _ in 0..length {
            if i % 2 == 0 {
                result.push(Some(file_id));
            } else {
                result.push(None);
            }
        }
        if i % 2 == 0 {
            file_id += 1;
        }
    }
    
    result
}

#[derive(Debug)]
struct File {
    id: usize,
    start: usize,
    length: usize,
}

fn find_files(disk: &[Option<usize>]) -> Vec<File> {
    let mut files = Vec::new();
    let mut i = 0;
    
    while i < disk.len() {
        if let Some(id) = disk[i] {
            let start = i;
            let mut length = 1;
            while i + 1 < disk.len() && disk[i + 1] == Some(id) {
                length += 1;
                i += 1;
            }
            files.push(File { id, start, length });
        }
        i += 1;
    }
    
    files
}

fn find_free_space(disk: &[Option<usize>], start: usize, needed: usize) -> Option<usize> {
    let mut current_length = 0;
    let mut current_start = None;
    
    for (i, &block) in disk[..start].iter().enumerate() {
        if block.is_none() {
            if current_start.is_none() {
                current_start = Some(i);
            }
            current_length += 1;
            if current_length >= needed {
                return current_start;
            }
        } else {
            current_length = 0;
            current_start = None;
        }
    }
    None
}

fn compact_disk_whole_files(disk: &mut Vec<Option<usize>>) {
    let mut files = find_files(disk);
    files.sort_by_key(|f| std::cmp::Reverse(f.id));
    
    for file in files {
        if let Some(new_start) = find_free_space(disk, file.start, file.length) {
            // Move the whole file
            for i in 0..file.length {
                disk[new_start + i] = Some(file.id);
                disk[file.start + i] = None;
            }
        }
    }
}

fn calculate_checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(pos, &block)| {
            block.map(|file_id| pos * file_id)
        })
        .sum()
}

pub  fn solve_part2() -> usize {
    let lengths = parse_disk_map(include_str!("../inputs/day9.txt"));
    let mut disk = expand_disk_map(&lengths);
    compact_disk_whole_files(&mut disk);
    calculate_checksum(&disk)
}

pub fn solve_part1() -> usize {
    let lengths = parse_disk_map(include_str!("../inputs/day9.txt"));
    let mut disk = expand_disk_map(&lengths);
    compact_disk(&mut disk);
    calculate_checksum(&disk)
}