/*
** src/puzzles/aoc2024/day15.rs
*/

use super::Solution;
use crate::types::{Direction, Grid};
use crate::utils;

use log::debug;

use std::collections::HashSet;

#[derive(Clone)]
struct Map {
    grid: Grid<char>,
    robot: (usize, usize),
}

impl Map {
    fn new(mut grid: Grid<char>) -> Self {
        let robot = grid.find(&'@').unwrap();
        grid.set(robot.0, robot.1, '.');
        Self { grid, robot }
    }

    fn gps(i: usize, j: usize) -> usize {
        (100 * i) + j
    }

    fn is_empty(&self, at: (usize, usize)) -> bool {
        let (i, j) = at;
        self.grid.get(i, j) == &'.'
    }

    fn is_box(&self, at: (usize, usize)) -> bool {
        let (i, j) = at;
        let item = *self.grid.get(i, j);
        item == 'O' || item == '[' || item == ']'
    }

    fn neighbor(&self, at: (usize, usize), direction: Direction) -> (usize, usize) {
        let (i, j) = at;
        self.grid.neighbor(i, j, direction).unwrap()
    }

    fn boxes(&self) -> Vec<(usize, usize)> {
        self.grid.find_all_with(|&c| c == 'O' || c == '[')
    }

    fn move_robot_into_box(&mut self, at: (usize, usize), direction: Direction) {
        let mut point = at;
        while self.is_box(point) {
            point = self.neighbor(point, direction);
        }
        if self.is_empty(point) {
            self.grid.set(point.0, point.1, 'O');
            self.grid.set(at.0, at.1, '.');
            self.robot = at;
        }
    }

    fn box_adjacent_coordinates(
        &self,
        at: (usize, usize),
        direction: Direction,
    ) -> Vec<(usize, usize)> {
        let (i, j) = at;
        match direction {
            Direction::East => {
                vec![(i, j + 2)]
            }
            Direction::West => {
                vec![(i, j - 1)]
            }
            Direction::North => {
                vec![(i - 1, j), (i - 1, j + 1)]
            }
            Direction::South => {
                vec![(i + 1, j), (i + 1, j + 1)]
            }
            _ => unreachable!(),
        }
    }

    fn can_move_robot_into_box_doubled(&self, at: (usize, usize), direction: Direction) -> bool {
        let (i, j) = at;
        // always index the box from the left side
        if self.grid.get(i, j) == &']' {
            return self.can_move_robot_into_box_doubled((i, j - 1), direction);
        }
        debug!(
            "can_move_robot_into_box_doubled: at: {:?}: direction: {:?}",
            at, direction
        );

        let mut can_move = true;
        for point in self.box_adjacent_coordinates(at, direction) {
            if self.is_box(point) {
                can_move = can_move && self.can_move_robot_into_box_doubled(point, direction);
            } else {
                can_move = can_move && self.is_empty(point);
            }
        }
        can_move
    }

    fn move_robot_into_box_doubled(
        &mut self,
        at: (usize, usize),
        direction: Direction,
        visited: &mut HashSet<(usize, usize)>,
    ) {
        let (i, j) = at;
        // always index the box from the left side
        if self.grid.get(i, j) == &']' {
            self.move_robot_into_box_doubled((i, j - 1), direction, visited);
            return;
        }
        if visited.contains(&at) {
            return;
        }
        debug!(
            "move_robot_into_box_doubled: at: {:?}: direction: {:?}",
            at, direction
        );

        let mut adjacent_boxes = self
            .box_adjacent_coordinates(at, direction)
            .into_iter()
            .filter(|&point| self.is_box(point))
            .collect::<Vec<_>>();
        if adjacent_boxes.len() == 2
            && self.grid.get(adjacent_boxes[0].0, adjacent_boxes[0].1) == &'['
        {
            let _ = adjacent_boxes.pop();
        }
        for adjacent_box in adjacent_boxes {
            self.move_robot_into_box_doubled(adjacent_box, direction, visited);
        }
        let (next_i, next_j) = self.neighbor(at, direction);
        // box is always indexed from the left side
        self.grid.set(i, j, '.');
        self.grid.set(i, j + 1, '.');
        self.grid.set(next_i, next_j, '[');
        self.grid.set(next_i, next_j + 1, ']');
        visited.insert(at);
    }

    fn move_robot(&mut self, direction: Direction) {
        let next = self.neighbor(self.robot, direction);
        if self.is_empty(next) {
            self.robot = next;
        } else if self.is_box(next) {
            self.move_robot_into_box(next, direction);
        }
    }

    fn move_robot_doubled(&mut self, direction: Direction) {
        let next = self.neighbor(self.robot, direction);
        if self.is_empty(next) {
            self.robot = next;
        } else if self.is_box(next) && self.can_move_robot_into_box_doubled(next, direction) {
            let mut visited = HashSet::new();
            self.move_robot_into_box_doubled(next, direction, &mut visited);
            self.robot = next;
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, j, c) in self.grid.iter_grid() {
            if j == 0 {
                writeln!(f)?;
            }
            if (i, j) == self.robot {
                write!(f, "@")?;
            } else {
                write!(f, "{}", c)?;
            }
        }
        writeln!(f)
    }
}

fn parse_input(input: String) -> (Map, Vec<Direction>) {
    let (grid, moves) = utils::split(&input, "\n\n").unwrap();
    let map = Map::new(Grid::from(grid.to_owned()));
    let moves = moves
        .split('\n')
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => Direction::North,
                'v' => Direction::South,
                '>' => Direction::East,
                '<' => Direction::West,
                _ => unreachable!(),
            })
        })
        .collect::<Vec<_>>();
    (map, moves)
}

fn double_map(mut map: Map, robot: (usize, usize)) -> Map {
    map.grid.set(robot.0, robot.1, '@');

    let mut new_grid = Grid::new(map.grid.width * 2, map.grid.height);
    for (i, j, c) in map.grid.iter_grid() {
        let (a, b) = match c {
            '#' => ('#', '#'),
            'O' => ('[', ']'),
            '.' => ('.', '.'),
            '@' => ('@', '.'),
            _ => unreachable!(),
        };
        new_grid.set(i, j * 2, a);
        new_grid.set(i, (j * 2) + 1, b);
    }
    Map::new(new_grid)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The lanternfish have a map of the warehouse and a list of movements the robot will attempt
    // to make. The problem is that the movements will sometimes fail as boxes are shifted around,
    // making the actual movements of the robot difficult to predict.
    let (map, moves) = parse_input(input);
    let start = map.robot;

    // Part A: Predict the motion of the robot and boxes in the warehouse. After the robot is
    // finished moving, what is the sum of all boxes' GPS coordinates?
    let mut map_a = map.clone();
    for &direction in moves.iter() {
        map_a.move_robot(direction);
    }
    let coordinates = map_a
        .boxes()
        .into_iter()
        .map(|(i, j)| Map::gps(i, j))
        .sum::<usize>();
    solution.set_part_a(coordinates);

    // Part B: A second warehouse's robot is also malfunctioning. Its layout is surprisingly
    // similar to the one you just helped, but everything except the robot is twice as wide!
    // Predict the motion of the robot and boxes in this new, scaled-up warehouse. What is the sum
    // of all boxes' final GPS coordinates?
    let mut map_b = double_map(map, start);
    for &direction in moves.iter() {
        map_b.move_robot_doubled(direction);
    }
    let coordinates = map_b
        .boxes()
        .into_iter()
        .map(|(i, j)| Map::gps(i, j))
        .sum::<usize>();
    solution.set_part_b(coordinates);

    solution
}
