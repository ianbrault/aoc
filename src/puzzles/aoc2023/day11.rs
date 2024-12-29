/*
** src/puzzles/aoc2023/day11.rs
*/

use super::Solution;
use crate::types::Grid;
use crate::utils;

#[derive(Clone, Default)]
enum Space {
    #[default]
    Empty,
    Galaxy,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("Space::From<char>: invalid character: {}", value),
        }
    }
}

struct Image {
    galaxies: Vec<(usize, usize)>,
    expanded_rows: Vec<usize>,
    expanded_cols: Vec<usize>,
}

impl Image {
    fn shortest_path_sum(&self, expansion_factor: usize) -> usize {
        let mut distances = 0;
        let mut expanded_crossed = 0;

        for x in 0..(self.galaxies.len() - 1) {
            for y in (x + 1)..self.galaxies.len() {
                let (ia, ja) = self.galaxies[x];
                let (ib, jb) = self.galaxies[y];
                let (i0, i1) = utils::min_max(ia, ib);
                let (j0, j1) = utils::min_max(ja, jb);
                // check how many expanded rows/columns are crossed
                let rows = self
                    .expanded_rows
                    .iter()
                    .filter(|&&i| i > i0 && i < i1)
                    .count();
                let cols = self
                    .expanded_cols
                    .iter()
                    .filter(|&&j| j > j0 && j < j1)
                    .count();
                distances += (i1 - i0 - rows) + (j1 - j0 - cols);
                expanded_crossed += rows + cols;
            }
        }

        distances + (expanded_crossed * expansion_factor)
    }
}

impl From<String> for Image {
    fn from(value: String) -> Self {
        let data = value
            .split('\n')
            .map(|line| line.chars().map(Space::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let grid = Grid::from(data);

        let expanded_rows = (0..grid.height)
            .filter(|&i| grid.iter_row(i).all(|space| matches!(space, Space::Empty)))
            .collect();
        let expanded_cols = (0..grid.width)
            .filter(|&j| grid.iter_col(j).all(|space| matches!(space, Space::Empty)))
            .collect();

        let mut galaxies = Vec::new();
        for (i, j, space) in grid.iter_grid() {
            if matches!(space, Space::Galaxy) {
                galaxies.push((i, j));
            }
        }

        Self {
            galaxies,
            expanded_rows,
            expanded_cols,
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The researcher has collected a bunch of data and compiled it into a single giant image. The
    // image includes empty space and galaxies. The researcher is trying to figure out the sum of
    // the lengths of the shortest path between every pair of galaxies. However, the universe
    // expanded in the time it took the light from those galaxies to reach the observatory. Due to
    // gravitational effects, only some space expands. In fact, the result is that any rows or
    // columns that contain no galaxies should all actually be twice as big.
    let image = Image::from(input);

    // Part A: Expand the universe, then find the length of the shortest path between every pair of
    // galaxies. What is the sum of these lengths?
    let expansion_factor = 2;
    let shortest_path_sum_double = image.shortest_path_sum(expansion_factor);
    solution.set_part_a(shortest_path_sum_double);

    // Part B: The galaxies are much older (and thus much farther apart) than the researcher
    // initially estimated. Now, instead of the expansion you did before, make each empty row or
    // column one million times larger. Starting with the same initial image, expand the universe
    // according to these new rules, then find the length of the shortest path between every pair
    // of galaxies. What is the sum of these lengths?
    let expansion_factor = 1000000;
    let shortest_path_sum_million = image.shortest_path_sum(expansion_factor);
    solution.set_part_b(shortest_path_sum_million);

    solution
}
