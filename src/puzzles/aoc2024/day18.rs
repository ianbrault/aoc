/*
** src/puzzles/aoc2024/day18.rs
*/

use super::Solution;
use crate::types::{Maze, Point};

fn drop_bytes(space: &mut Maze, bytes: &[Point]) {
    for point in bytes {
        space.set(point.y as usize, point.x as usize, '#');
    }
}

fn find_blocking_byte(space: Maze, bytes: &[Point]) -> Option<String> {
    let mut a = 0;
    let mut b = bytes.len() - 1;
    let mut last_blocked = None;

    while a + 1 < b {
        let c = a + ((b - a) / 2);
        let mut c_space = space.clone();
        drop_bytes(&mut c_space, &bytes[..=c]);

        let distances = c_space.distance_matrix((0, 0));
        if distances.get(space.height - 1, space.width - 1) == &usize::MAX {
            b = c;
            last_blocked = Some(c);
        } else {
            a = c;
        }
    }

    last_blocked.map(|i| format!("{}", bytes[i]))
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let falling_bytes = input
        .split('\n')
        .map(|line| Point::try_from(line).unwrap())
        .collect::<Vec<_>>();
    let mut space = Maze::create(71, 71);

    // Part A: Simulate the first kilobyte (1024 bytes) falling onto your memory space. Afterward,
    // what is the minimum number of steps needed to reach the exit?
    drop_bytes(&mut space, &falling_bytes[..1024]);
    let distances = space.distance_matrix((0, 0));
    let steps_to_exit = distances.get(space.height - 1, space.width - 1);
    solution.set_part_a(steps_to_exit);

    // Part B: Simulate more of the bytes that are about to corrupt your memory space. What are the
    // coordinates of the first byte that will prevent the exit from being reachable from your
    // starting position?
    let blocker = find_blocking_byte(space, &falling_bytes[1024..]);
    solution.maybe_set_part_b(blocker);

    solution
}
