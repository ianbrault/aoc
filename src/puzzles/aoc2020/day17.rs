/*
** src/puzzles/aoc2020/day17.rs
*/

use super::Solution;

use std::collections::HashSet;

struct Product3D<T> {
    a: Vec<T>,
    b: Vec<T>,
    c: Vec<T>,
    i: usize,
    j: usize,
    k: usize,
}

impl<T> Product3D<T> {
    fn new<I, J, K>(a: I, b: J, c: K) -> Self
    where
        I: Iterator<Item = T>,
        J: Iterator<Item = T>,
        K: Iterator<Item = T>,
    {
        let a = a.collect();
        let b = b.collect();
        let c = c.collect();
        Self {
            a,
            b,
            c,
            i: 0,
            j: 0,
            k: 0,
        }
    }
}

impl<T> Iterator for Product3D<T>
where
    T: Clone,
{
    type Item = (T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.a.len() {
            None
        } else {
            let a = self.a[self.i].clone();
            let b = self.b[self.j].clone();
            let c = self.c[self.k].clone();
            self.k += 1;
            if self.k == self.c.len() {
                self.k = 0;
                self.j += 1;
                if self.j == self.b.len() {
                    self.j = 0;
                    self.i += 1;
                }
            }
            Some((a, b, c))
        }
    }
}

struct Product4D<T> {
    a: Vec<T>,
    b: Vec<T>,
    c: Vec<T>,
    d: Vec<T>,
    i: usize,
    j: usize,
    k: usize,
    l: usize,
}

impl<T> Product4D<T> {
    fn new<I, J, K, L>(a: I, b: J, c: K, d: L) -> Self
    where
        I: Iterator<Item = T>,
        J: Iterator<Item = T>,
        K: Iterator<Item = T>,
        L: Iterator<Item = T>,
    {
        let a = a.collect();
        let b = b.collect();
        let c = c.collect();
        let d = d.collect();
        Self {
            a,
            b,
            c,
            d,
            i: 0,
            j: 0,
            k: 0,
            l: 0,
        }
    }
}

impl<T> Iterator for Product4D<T>
where
    T: Clone,
{
    type Item = (T, T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.a.len() {
            None
        } else {
            let a = self.a[self.i].clone();
            let b = self.b[self.j].clone();
            let c = self.c[self.k].clone();
            let d = self.d[self.l].clone();
            self.l += 1;
            if self.l == self.d.len() {
                self.l = 0;
                self.k += 1;
                if self.k == self.c.len() {
                    self.k = 0;
                    self.j += 1;
                    if self.j == self.b.len() {
                        self.j = 0;
                        self.i += 1;
                    }
                }
            }
            Some((a, b, c, d))
        }
    }
}

struct CubeAutomaton3D {
    initial_size: usize,
    // active cube sets are double-buffered so that we can do "simultaneous"
    // updates reading from one and writing to the other
    active_cubes_a: HashSet<(i64, i64, i64)>,
    active_cubes_b: HashSet<(i64, i64, i64)>,
    active_set: usize,
}

impl CubeAutomaton3D {
    fn active_cubes(&self) -> usize {
        match self.active_set {
            0 => &self.active_cubes_a,
            1 => &self.active_cubes_b,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .len()
    }

    fn is_active(&self, x: i64, y: i64, z: i64) -> bool {
        // check the current set
        match self.active_set {
            0 => &self.active_cubes_a,
            1 => &self.active_cubes_b,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .contains(&(x, y, z))
    }

    fn add_cube(&mut self, x: i64, y: i64, z: i64) {
        // add the cube to the upcoming set
        match self.active_set {
            0 => &mut self.active_cubes_b,
            1 => &mut self.active_cubes_a,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .insert((x, y, z));
    }

    fn remove_cube(&mut self, x: i64, y: i64, z: i64) {
        // remove the cube from the upcoming set
        match self.active_set {
            0 => &mut self.active_cubes_b,
            1 => &mut self.active_cubes_a,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .remove(&(x, y, z));
    }

    fn active_neighbors(&self, x: i64, y: i64, z: i64) -> usize {
        let active = Product3D::new((x - 1)..=(x + 1), (y - 1)..=(y + 1), (z - 1)..=(z + 1))
            .filter(|(dx, dy, dz)| self.is_active(*dx, *dy, *dz))
            .count();
        // exclude the given point
        if self.is_active(x, y, z) {
            active - 1
        } else {
            active
        }
    }

    fn run_cycle(&mut self, cycle: i64) {
        // clear the upcoming set
        match self.active_set {
            0 => &mut self.active_cubes_b,
            1 => &mut self.active_cubes_a,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .clear();

        let x_range = (-cycle - 1)..=(self.initial_size as i64 + cycle);
        let y_range = (-cycle - 1)..=(self.initial_size as i64 + cycle);
        let z_range = (-cycle - 1)..=(cycle + 1);

        for (x, y, z) in Product3D::new(x_range, y_range, z_range) {
            let active_neighbors = self.active_neighbors(x, y, z);
            if self.is_active(x, y, z) {
                if active_neighbors != 2 && active_neighbors != 3 {
                    self.remove_cube(x, y, z);
                } else {
                    self.add_cube(x, y, z);
                }
            } else if active_neighbors == 3 {
                self.add_cube(x, y, z);
            }
        }

        self.active_set = (self.active_set + 1) % 2;
    }

    fn run_to_completion(&mut self, cycles: usize) {
        for n in 0..cycles {
            self.run_cycle(n as i64);
        }
    }
}

impl From<&str> for CubeAutomaton3D {
    fn from(value: &str) -> Self {
        let mut active_cubes_a = HashSet::new();
        let mut active_cubes_b = HashSet::new();
        let mut initial_size = None;

        for (row, line) in value.split('\n').enumerate() {
            initial_size = Some(line.len());
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    active_cubes_a.insert((col as i64, row as i64, 0));
                    active_cubes_b.insert((col as i64, row as i64, 0));
                }
            }
        }

        Self {
            initial_size: initial_size.unwrap(),
            active_cubes_a,
            active_cubes_b,
            active_set: 0,
        }
    }
}

struct CubeAutomaton4D {
    initial_size: usize,
    // active cube sets are double-buffered so that we can do "simultaneous"
    // updates reading from one and writing to the other
    active_cubes_a: HashSet<(i64, i64, i64, i64)>,
    active_cubes_b: HashSet<(i64, i64, i64, i64)>,
    active_set: usize,
}

impl CubeAutomaton4D {
    fn active_cubes(&self) -> usize {
        match self.active_set {
            0 => &self.active_cubes_a,
            1 => &self.active_cubes_b,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .len()
    }

    fn is_active(&self, x: i64, y: i64, z: i64, w: i64) -> bool {
        // check the current set
        match self.active_set {
            0 => &self.active_cubes_a,
            1 => &self.active_cubes_b,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .contains(&(x, y, z, w))
    }

    fn add_cube(&mut self, x: i64, y: i64, z: i64, w: i64) {
        // add the cube to the upcoming set
        match self.active_set {
            0 => &mut self.active_cubes_b,
            1 => &mut self.active_cubes_a,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .insert((x, y, z, w));
    }

    fn remove_cube(&mut self, x: i64, y: i64, z: i64, w: i64) {
        // remove the cube from the upcoming set
        match self.active_set {
            0 => &mut self.active_cubes_b,
            1 => &mut self.active_cubes_a,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .remove(&(x, y, z, w));
    }

    fn active_neighbors(&self, x: i64, y: i64, z: i64, w: i64) -> usize {
        let active = Product4D::new(
            (x - 1)..=(x + 1),
            (y - 1)..=(y + 1),
            (z - 1)..=(z + 1),
            (w - 1)..=(w + 1),
        )
        .filter(|(dx, dy, dz, dw)| self.is_active(*dx, *dy, *dz, *dw))
        .count();
        // exclude the given point
        if self.is_active(x, y, z, w) {
            active - 1
        } else {
            active
        }
    }

    fn run_cycle(&mut self, cycle: i64) {
        // clear the upcoming set
        match self.active_set {
            0 => &mut self.active_cubes_b,
            1 => &mut self.active_cubes_a,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .clear();

        let x_range = (-cycle - 1)..=(self.initial_size as i64 + cycle);
        let y_range = (-cycle - 1)..=(self.initial_size as i64 + cycle);
        let z_range = (-cycle - 1)..=(cycle + 1);
        let w_range = (-cycle - 1)..=(cycle + 1);

        for (x, y, z, w) in Product4D::new(x_range, y_range, z_range, w_range) {
            let active_neighbors = self.active_neighbors(x, y, z, w);
            if self.is_active(x, y, z, w) {
                if active_neighbors != 2 && active_neighbors != 3 {
                    self.remove_cube(x, y, z, w);
                } else {
                    self.add_cube(x, y, z, w);
                }
            } else if active_neighbors == 3 {
                self.add_cube(x, y, z, w);
            }
        }

        self.active_set = (self.active_set + 1) % 2;
    }

    fn run_to_completion(&mut self, cycles: usize) {
        for n in 0..cycles {
            self.run_cycle(n as i64);
        }
    }
}

impl From<&str> for CubeAutomaton4D {
    fn from(value: &str) -> Self {
        let mut active_cubes_a = HashSet::new();
        let mut active_cubes_b = HashSet::new();
        let mut initial_size = None;

        for (row, line) in value.split('\n').enumerate() {
            initial_size = Some(line.len());
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    active_cubes_a.insert((col as i64, row as i64, 0, 0));
                    active_cubes_b.insert((col as i64, row as i64, 0, 0));
                }
            }
        }

        Self {
            initial_size: initial_size.unwrap(),
            active_cubes_a,
            active_cubes_b,
            active_set: 0,
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();

    // Part A: Starting with your given initial configuration, simulate six cycles in a
    // 3-dimensional space. How many cubes are left in the active state after the sixth cycle?
    let mut automaton = CubeAutomaton3D::from(input.as_str());
    automaton.run_to_completion(6);
    solution.set_part_a(automaton.active_cubes());

    // Part B: Starting with your given initial configuration, simulate six cycles in a
    // 4-dimensional space. How many cubes are left in the active state after the sixth cycle?
    let mut automaton = CubeAutomaton4D::from(input.as_str());
    automaton.run_to_completion(6);
    solution.set_part_b(automaton.active_cubes());

    solution
}
