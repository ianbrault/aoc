/*
** src/puzzles/aoc2023/day10.rs
*/

use super::Solution;
use crate::types::Grid;

use log::debug;

use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug)]
enum TileType {
    PipeVertical,
    PipeHorizontal,
    Pipe90NE,
    Pipe90NW,
    Pipe90SW,
    Pipe90SE,
    Ground,
    Start,
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::PipeVertical,
            '-' => Self::PipeHorizontal,
            'L' => Self::Pipe90NE,
            'J' => Self::Pipe90NW,
            '7' => Self::Pipe90SW,
            'F' => Self::Pipe90SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Tile::From<char>: invalid character: {}", value),
        }
    }
}

#[derive(Clone)]
struct Tile {
    ty: TileType,
    connects_north: bool,
    connects_south: bool,
    connects_east: bool,
    connects_west: bool,
}

impl Tile {
    fn new(ty: TileType) -> Self {
        let connects_north = matches!(
            ty,
            TileType::PipeVertical | TileType::Pipe90NE | TileType::Pipe90NW
        );
        let connects_south = matches!(
            ty,
            TileType::PipeVertical | TileType::Pipe90SE | TileType::Pipe90SW
        );
        let connects_east = matches!(
            ty,
            TileType::PipeHorizontal | TileType::Pipe90NE | TileType::Pipe90SE
        );
        let connects_west = matches!(
            ty,
            TileType::PipeHorizontal | TileType::Pipe90NW | TileType::Pipe90SW
        );
        Self {
            ty,
            connects_north,
            connects_south,
            connects_east,
            connects_west,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            ty: TileType::Ground,
            connects_north: false,
            connects_south: false,
            connects_east: false,
            connects_west: false,
        }
    }
}

struct PipeMap {
    grid: Grid<Tile>,
    start_point: (usize, usize),
}

impl PipeMap {
    fn loop_points(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::new();
        let mut point_set = HashSet::new();

        let mut current = self.start_point;
        loop {
            debug!("adding point {},{} to loop", current.0, current.1);
            points.push(current);
            point_set.insert(current);

            let (i, j) = current;
            let tile = self.grid.get(i, j);
            let mut end = true;
            // check connections in clockwise order
            if tile.connects_north && !point_set.contains(&(i - 1, j)) {
                current = (i - 1, j);
                end = false;
            } else if tile.connects_east && !point_set.contains(&(i, j + 1)) {
                current = (i, j + 1);
                end = false;
            } else if tile.connects_south && !point_set.contains(&(i + 1, j)) {
                current = (i + 1, j);
                end = false;
            } else if tile.connects_west && !point_set.contains(&(i, j - 1)) {
                current = (i, j - 1);
                end = false;
            }
            if end {
                break;
            }
        }

        points
    }
}

impl From<String> for PipeMap {
    fn from(value: String) -> Self {
        let chars = value
            .split('\n')
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = chars.first().unwrap().len();
        let height = chars.len();

        let mut grid = Grid::<Tile>::new(width, height);
        let mut start_point = (0, 0);
        for (i, row) in chars.into_iter().enumerate() {
            for (j, ch) in row.into_iter().enumerate() {
                let ty = TileType::from(ch);
                if matches!(ty, TileType::Start) {
                    start_point = (i, j);
                }
                grid.set(i, j, Tile::new(ty));
            }
        }

        // set reachability of the start
        let (i, j) = start_point;
        if i > 0 && grid.get(i - 1, j).connects_south {
            grid.get_mut(i, j).connects_north = true;
        }
        if i < height - 1 && grid.get(i + 1, j).connects_north {
            grid.get_mut(i, j).connects_south = true;
        }
        if j < width - 1 && grid.get(i, j + 1).connects_west {
            grid.get_mut(i, j).connects_east = true;
        }
        if j > 0 && grid.get(i, j - 1).connects_east {
            grid.get_mut(i, j).connects_west = true;
        }
        Self { grid, start_point }
    }
}

fn get_expanded_pipe_map(pipe_map: &PipeMap, points: &[(usize, usize)]) -> PipeMap {
    // expand 3x
    let mut grid = Grid::<Tile>::new(pipe_map.grid.width * 3, pipe_map.grid.height * 3);
    let start_point = (
        (pipe_map.start_point.0 * 3) + 1,
        (pipe_map.start_point.1 * 3) + 1,
    );

    for &(i, j) in points {
        let tile = pipe_map.grid.get(i, j);
        match tile.ty {
            TileType::PipeVertical | TileType::Start
                if tile.connects_north && tile.connects_south =>
            {
                grid.set(i * 3, (j * 3) + 1, Tile::new(TileType::PipeVertical));
                grid.set((i * 3) + 1, (j * 3) + 1, Tile::new(TileType::PipeVertical));
                grid.set((i * 3) + 2, (j * 3) + 1, Tile::new(TileType::PipeVertical));
            }
            TileType::PipeHorizontal | TileType::Start
                if tile.connects_east && tile.connects_west =>
            {
                grid.set((i * 3) + 1, j * 3, Tile::new(TileType::PipeHorizontal));
                grid.set(
                    (i * 3) + 1,
                    (j * 3) + 1,
                    Tile::new(TileType::PipeHorizontal),
                );
                grid.set(
                    (i * 3) + 1,
                    (j * 3) + 2,
                    Tile::new(TileType::PipeHorizontal),
                );
            }
            TileType::Pipe90NE | TileType::Start if tile.connects_north && tile.connects_east => {
                grid.set(i * 3, (j * 3) + 1, Tile::new(TileType::PipeVertical));
                grid.set((i * 3) + 1, (j * 3) + 1, Tile::new(TileType::Pipe90NE));
                grid.set(
                    (i * 3) + 1,
                    (j * 3) + 2,
                    Tile::new(TileType::PipeHorizontal),
                );
            }
            TileType::Pipe90NW | TileType::Start if tile.connects_north && tile.connects_west => {
                grid.set(i * 3, (j * 3) + 1, Tile::new(TileType::PipeVertical));
                grid.set((i * 3) + 1, (j * 3) + 1, Tile::new(TileType::Pipe90NW));
                grid.set((i * 3) + 1, j * 3, Tile::new(TileType::PipeHorizontal));
            }
            TileType::Pipe90SE | TileType::Start if tile.connects_south && tile.connects_east => {
                grid.set((i * 3) + 2, (j * 3) + 1, Tile::new(TileType::PipeVertical));
                grid.set((i * 3) + 1, (j * 3) + 1, Tile::new(TileType::Pipe90SE));
                grid.set(
                    (i * 3) + 1,
                    (j * 3) + 2,
                    Tile::new(TileType::PipeHorizontal),
                );
            }
            TileType::Pipe90SW | TileType::Start if tile.connects_south && tile.connects_west => {
                grid.set((i * 3) + 2, (j * 3) + 1, Tile::new(TileType::PipeVertical));
                grid.set((i * 3) + 1, (j * 3) + 1, Tile::new(TileType::Pipe90SW));
                grid.set((i * 3) + 1, j * 3, Tile::new(TileType::PipeHorizontal));
            }
            _ => {}
        }
    }

    PipeMap { grid, start_point }
}

fn enclosed_area(pipe_map: &PipeMap, points: &[(usize, usize)]) -> usize {
    // start by expanding the pipe map to a 3x resolution
    let pipe_map_large = get_expanded_pipe_map(pipe_map, points);

    // then flood fill to mark all points outside the loop
    let mut filled = Grid::<bool>::new(pipe_map_large.grid.width, pipe_map_large.grid.height);
    for (i, j, tile) in pipe_map_large.grid.iter_grid() {
        if !matches!(tile.ty, TileType::Ground) {
            filled.set(i, j, true);
        }
    }
    let mut queue = VecDeque::new();
    // start with all edge points
    for i in 0..pipe_map_large.grid.height {
        if matches!(pipe_map_large.grid.get(i, 0).ty, TileType::Ground) {
            queue.push_back((i, 0));
        }
        if matches!(
            pipe_map_large.grid.get(i, pipe_map_large.grid.width - 1).ty,
            TileType::Ground
        ) {
            queue.push_back((i, pipe_map_large.grid.width - 1));
        }
    }
    for j in 1..(pipe_map_large.grid.width - 1) {
        if matches!(pipe_map_large.grid.get(0, j).ty, TileType::Ground) {
            queue.push_back((0, j));
        }
        if matches!(
            pipe_map_large
                .grid
                .get(pipe_map_large.grid.height - 1, j)
                .ty,
            TileType::Ground
        ) {
            queue.push_back((pipe_map_large.grid.height - 1, j));
        }
    }
    while let Some((i, j)) = queue.pop_front() {
        if *filled.get(i, j) {
            continue;
        }
        filled.set(i, j, true);
        if i > 0 && !filled.get(i - 1, j) {
            queue.push_back((i - 1, j));
        }
        if j > 0 {
            queue.push_back((i, j - 1));
        }
        if i < pipe_map_large.grid.height - 1 {
            queue.push_back((i + 1, j));
        }
        if j < pipe_map_large.grid.width - 1 {
            queue.push_back((i, j + 1));
        }
    }

    // now downsize where the area is each 3x3 grid where all points are unfilled
    let mut area = 0;
    for i in 0..pipe_map.grid.height {
        for j in 0..pipe_map.grid.width {
            let mut all_unfilled = true;
            for ii in (i * 3)..((i + 1) * 3) {
                for jj in (j * 3)..((j + 1) * 3) {
                    if *filled.get(ii, jj) {
                        all_unfilled = false;
                        break;
                    }
                }
            }
            if all_unfilled {
                area += 1;
            }
        }
    }

    area
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // Scanning the area, you discover that the entire field you're standing on is densely packed
    // with pipes; it was hard to tell at first because they're the same metallic silver color as
    // the "ground". You make a quick sketch of all of the surface pipes you can see.
    let pipe_map = PipeMap::from(input);

    // Part A: Find the single giant loop starting at S. How many steps along the loop does it take
    // to get from the starting position to the point farthest from the starting position?
    let loop_points = pipe_map.loop_points();
    let farthest_point = (loop_points.len() / 2) + (loop_points.len() % 2);
    solution.set_part_a(farthest_point);

    // Part B: Figure out whether you have time to search for the nest by calculating the area
    // within the loop. How many tiles are enclosed by the loop?
    let area = enclosed_area(&pipe_map, &loop_points);
    solution.set_part_b(area);

    solution
}
