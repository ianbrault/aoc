/*
** src/puzzles/aoc2024/day9.rs
*/

use super::Solution;

use std::collections::VecDeque;
use std::iter;

#[derive(Clone)]
enum DiskItem {
    File(usize, usize),
    FreeSpace(usize),
}

#[derive(Clone, Debug)]
struct Block(usize);

fn parse_disk_map(input: String) -> Vec<DiskItem> {
    let mut items = Vec::new();
    for (i, c) in input.chars().enumerate() {
        let n = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            items.push(DiskItem::File(i / 2, n));
        } else {
            items.push(DiskItem::FreeSpace(n));
        }
    }
    items
}

fn compact_hard_drive(disk_map: &[DiskItem]) -> Vec<Block> {
    let mut blocks = Vec::new();

    let mut queue = VecDeque::new();
    queue.extend(disk_map.iter().cloned());

    let mut state = queue.pop_front();
    let mut tail = queue.pop_back();
    while let Some(item) = &mut state {
        match item {
            DiskItem::File(id, file_size) => {
                blocks.extend(iter::repeat(Block(*id)).take(*file_size));
                state = queue.pop_front();
            }
            DiskItem::FreeSpace(ref mut size) => {
                if let Some(tail_item) = &mut tail {
                    match tail_item {
                        DiskItem::File(id, ref mut file_size) => {
                            if *file_size == 0 {
                                tail = queue.pop_back();
                            } else if file_size >= size {
                                blocks.extend(iter::repeat(Block(*id)).take(*size));
                                *file_size -= *size;
                                state = queue.pop_front();
                            } else {
                                blocks.extend(iter::repeat(Block(*id)).take(*file_size));
                                *size -= *file_size;
                                tail = queue.pop_back();
                            }
                        }
                        DiskItem::FreeSpace(_) => {
                            tail = queue.pop_back();
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }
    // check for leftover blocks on the tail
    if let Some(DiskItem::File(id, file_size)) = tail {
        blocks.extend(iter::repeat(Block(id)).take(file_size));
    }

    blocks
}

fn find_space_for_file(
    disk_map: &[DiskItem],
    file_size: usize,
    max_index: usize,
) -> Option<(usize, usize)> {
    for (i, item) in disk_map.iter().enumerate() {
        if i >= max_index {
            return None;
        }
        match item {
            &DiskItem::FreeSpace(size) if size >= file_size => {
                return Some((i, size));
            }
            _ => {}
        };
    }
    None
}

fn compact_hard_drive_no_fragmentation(disk_map: &[DiskItem]) -> Vec<Block> {
    let mut items = disk_map.to_vec();

    let max_file_id = items
        .iter()
        .filter_map(|item| match item {
            &DiskItem::File(id, _) => Some(id),
            _ => None,
        })
        .max()
        .unwrap();
    for file_id in (0..=max_file_id).rev() {
        let (file_index, file) = items
            .iter()
            .enumerate()
            .find(|(_, item)| matches!(item, &&DiskItem::File(id, _) if id == file_id))
            .unwrap();
        // find the first available free space that fits the file
        let file_size = match file {
            &DiskItem::File(_, size) => size,
            _ => unreachable!(),
        };
        if let Some((free_index, free_size)) = find_space_for_file(&items, file_size, file_index) {
            if free_index < file_index {
                items[free_index] = DiskItem::File(file_id, file_size);
                items[file_index] = DiskItem::FreeSpace(file_size);
                if free_size > file_size {
                    items.insert(free_index + 1, DiskItem::FreeSpace(free_size - file_size));
                }
            }
        }
    }

    items
        .into_iter()
        .flat_map(|item| match item {
            DiskItem::File(id, size) => iter::repeat(Block(id)).take(size),
            DiskItem::FreeSpace(size) => iter::repeat(Block(0)).take(size),
        })
        .collect()
}

fn filesystem_checksum(blocks: &[Block]) -> usize {
    blocks.iter().enumerate().map(|(i, Block(n))| i * n).sum()
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The pilot shows you the disk map. It uses a dense format to represent the layout of files
    // and free space on the disk. The digits alternate between indicating the length of a file and
    // the length of free space.
    let disk_map = parse_disk_map(input);

    // Part A: Compact the amphipod's hard drive using the process the pilot requested. What is the
    // resulting filesystem checksum?
    let blocks = compact_hard_drive(&disk_map);
    let checksum = filesystem_checksum(&blocks);
    solution.set_part_a(checksum);

    // Part B: Rather than move individual blocks, the pilot would like to try compacting the files
    // on his disk by moving whole files instead. Start over, now compacting the hard drive using
    // this new method instead. What is the resulting filesystem checksum?
    let blocks = compact_hard_drive_no_fragmentation(&disk_map);
    let checksum = filesystem_checksum(&blocks);
    solution.set_part_b(checksum);

    solution
}
