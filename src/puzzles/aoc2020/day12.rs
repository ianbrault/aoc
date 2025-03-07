/*
** src/puzzles/aoc2020/day12.rs
*/

use super::Solution;

#[derive(Clone, Copy)]
enum NavigationDirection {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl NavigationDirection {
    fn is_cardinal(&self) -> bool {
        matches!(
            self,
            NavigationDirection::North
                | NavigationDirection::South
                | NavigationDirection::East
                | NavigationDirection::West
        )
    }

    fn is_rotational(&self) -> bool {
        matches!(self, NavigationDirection::Left | NavigationDirection::Right)
    }

    fn rotate_left(&mut self) {
        match self {
            NavigationDirection::North => *self = NavigationDirection::West,
            NavigationDirection::South => *self = NavigationDirection::East,
            NavigationDirection::East => *self = NavigationDirection::North,
            NavigationDirection::West => *self = NavigationDirection::South,
            _ => unreachable!(),
        }
    }

    fn rotate_right(&mut self) {
        match self {
            NavigationDirection::North => *self = NavigationDirection::East,
            NavigationDirection::South => *self = NavigationDirection::West,
            NavigationDirection::East => *self = NavigationDirection::South,
            NavigationDirection::West => *self = NavigationDirection::North,
            _ => unreachable!(),
        }
    }
}

impl From<char> for NavigationDirection {
    fn from(c: char) -> Self {
        match c {
            'N' => NavigationDirection::North,
            'S' => NavigationDirection::South,
            'E' => NavigationDirection::East,
            'W' => NavigationDirection::West,
            'L' => NavigationDirection::Left,
            'R' => NavigationDirection::Right,
            'F' => NavigationDirection::Forward,
            _ => unreachable!(),
        }
    }
}

struct NavigationInstruction {
    direction: NavigationDirection,
    distance: i32,
}

impl From<&str> for NavigationInstruction {
    fn from(s: &str) -> Self {
        let direction = NavigationDirection::from(s.chars().next().unwrap());
        let distance = s[1..s.len()].parse().unwrap();

        Self {
            direction,
            distance,
        }
    }
}

struct Navigator<I> {
    x: i32,
    y: i32,
    direction: NavigationDirection,
    instructions: I,
    waypoint: Option<(i32, i32)>,
}

impl<I> Navigator<I> {
    fn with_waypoint(mut self, x: i32, y: i32) -> Self {
        self.waypoint = Some((x, y));
        self
    }

    fn direction_to_dx_dy(direction: NavigationDirection, distance: i32) -> (i32, i32) {
        match direction {
            NavigationDirection::North => (0, distance),
            NavigationDirection::South => (0, -distance),
            NavigationDirection::East => (distance, 0),
            NavigationDirection::West => (-distance, 0),
            _ => unreachable!(),
        }
    }

    fn move_ship(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn move_waypoint(&mut self, dx: i32, dy: i32) {
        if let Some((x, y)) = self.waypoint {
            self.waypoint = Some((x + dx, y + dy));
        } else {
            unreachable!()
        }
    }

    fn move_forward(&mut self, distance: i32) {
        if let Some((wx, wy)) = self.waypoint {
            for _ in 0..distance {
                self.move_ship(wx, wy);
            }
        } else {
            let (dx, dy) = Self::direction_to_dx_dy(self.direction, distance);
            self.move_ship(dx, dy);
        }
    }

    fn moves(&mut self, direction: NavigationDirection, distance: i32) {
        assert!(direction.is_cardinal());

        let (dx, dy) = Self::direction_to_dx_dy(direction, distance);

        // move the waypoint, if it is set
        // otherwise move the ship
        if self.waypoint.is_some() {
            self.move_waypoint(dx, dy);
        } else {
            self.move_ship(dx, dy);
        }
    }

    fn rotate_ship(&mut self, direction: NavigationDirection, degrees: i32) {
        let rotator = match direction {
            NavigationDirection::Left => NavigationDirection::rotate_left,
            NavigationDirection::Right => NavigationDirection::rotate_right,
            _ => unreachable!(),
        };

        for _ in 0..(degrees / 90) {
            rotator(&mut self.direction);
        }
    }

    fn rotate_waypoint(&mut self, direction: NavigationDirection, degrees: i32) {
        if let Some((mut x, mut y)) = self.waypoint {
            match direction {
                NavigationDirection::Left => {
                    for _ in 0..(degrees / 90) {
                        let t = x;
                        x = -y;
                        y = t;
                    }
                }
                NavigationDirection::Right => {
                    for _ in 0..(degrees / 90) {
                        let t = y;
                        y = -x;
                        x = t;
                    }
                }
                _ => unreachable!(),
            };
            self.waypoint = Some((x, y));
        } else {
            unreachable!()
        }
    }

    fn rotates(&mut self, direction: NavigationDirection, degrees: i32) {
        assert!(direction.is_rotational());
        assert!(degrees % 90 == 0);

        // rotate the waypoint, if it is set
        // otherwise rotate the ship
        if self.waypoint.is_some() {
            self.rotate_waypoint(direction, degrees);
        } else {
            self.rotate_ship(direction, degrees);
        }
    }
}

impl<'a, I> From<I> for Navigator<I>
where
    I: Iterator<Item = &'a NavigationInstruction>,
{
    fn from(instructions: I) -> Self {
        Self {
            x: 0,
            y: 0,
            // ship starts facing East
            direction: NavigationDirection::East,
            instructions,
            waypoint: None,
        }
    }
}

impl<'a, I> Iterator for Navigator<I>
where
    I: Iterator<Item = &'a NavigationInstruction>,
{
    // each iteration returns the new position
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        // consume instructions until they have been exhausted
        if let Some(instr) = self.instructions.next() {
            match instr.direction {
                dir if dir.is_cardinal() => self.moves(dir, instr.distance),
                dir if dir.is_rotational() => self.rotates(dir, instr.distance),
                NavigationDirection::Forward => self.move_forward(instr.distance),
                _ => unreachable!(),
            };
            Some((self.x, self.y))
        } else {
            None
        }
    }
}

fn distance(x: i32, y: i32) -> i32 {
    x.abs() + y.abs()
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let instructions = input
        .split('\n')
        .map(NavigationInstruction::from)
        .collect::<Vec<_>>();

    // Part A: Figure out where the navigation instructions lead. What is the Manhattan distance
    // between that location and the ship's starting position?
    let (x, y) = Navigator::from(instructions.iter()).last().unwrap();
    solution.set_part_a(distance(x, y));

    // Part B: Figure out where the navigation instructions actually lead (using the waypoint).
    // What is the Manhattan distance between that location and the ship's starting position?
    let (x, y) = Navigator::from(instructions.iter())
        .with_waypoint(10, 1)
        .last()
        .unwrap();
    solution.set_part_b(distance(x, y));

    solution
}
