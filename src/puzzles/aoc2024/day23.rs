/*
** src/puzzles/aoc2024/day23.rs
*/

use super::Solution;

use std::collections::{BTreeSet, HashMap, HashSet};

struct NetworkMap<'a> {
    id_to_name: HashMap<usize, &'a str>,
    adjacency_matrix: Vec<Vec<bool>>,
}

impl<'a> NetworkMap<'a> {
    fn computers(&self) -> impl Iterator<Item = usize> + '_ {
        0..self.id_to_name.len()
    }

    fn computer_name(&self, id: usize) -> &str {
        self.id_to_name[&id]
    }

    fn is_connected(&self, a: usize, b: usize) -> bool {
        self.adjacency_matrix[a][b]
    }

    fn connections(&self, to: usize) -> impl Iterator<Item = usize> + '_ {
        self.adjacency_matrix[to]
            .iter()
            .enumerate()
            .filter(|(_, adjacent)| **adjacent)
            .map(|(i, _)| i)
    }
}

impl<'a> From<&'a str> for NetworkMap<'a> {
    fn from(value: &'a str) -> Self {
        let connections = value
            .split('\n')
            .map(|line| (&line[..2], &line[3..]))
            .collect::<Vec<_>>();
        let mut computers = connections
            .iter()
            .fold(HashSet::new(), |mut acc, &(a, b)| {
                acc.insert(a);
                acc.insert(b);
                acc
            })
            .into_iter()
            .collect::<Vec<_>>();
        computers.sort();
        let ids = computers
            .into_iter()
            .enumerate()
            .map(|(i, c)| (c, i))
            .collect::<HashMap<_, _>>();
        let mut adjacency_matrix = vec![vec![false; ids.len()]; ids.len()];
        for (a, b) in connections {
            adjacency_matrix[ids[a]][ids[b]] = true;
            adjacency_matrix[ids[b]][ids[a]] = true;
        }
        Self {
            id_to_name: ids.into_iter().map(|(k, v)| (v, k)).collect(),
            adjacency_matrix,
        }
    }
}

fn find_interconnected_computers_with_t(network_map: &NetworkMap) -> usize {
    let mut connections = HashSet::new();
    for computer in network_map.computers() {
        for connection in network_map.connections(computer) {
            for subconnection in network_map.connections(connection) {
                if subconnection != computer && network_map.is_connected(computer, subconnection) {
                    let mut set = vec![computer, connection, subconnection];
                    set.sort();
                    connections.insert(set);
                }
            }
        }
    }
    connections
        .into_iter()
        .filter(|x| {
            x.iter()
                .any(|c| network_map.computer_name(*c).starts_with('t'))
        })
        .count()
}

fn bron_kerbosch(
    network_map: &NetworkMap,
    r: BTreeSet<usize>,
    mut p: BTreeSet<usize>,
    mut x: BTreeSet<usize>,
    cliques: &mut Vec<Vec<usize>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.into_iter().collect());
        return;
    }
    for v in p.iter().copied().collect::<Vec<_>>() {
        let mut next_r = r.clone();
        next_r.insert(v);
        let next_p = p
            .iter()
            .copied()
            .filter(|pp| network_map.is_connected(v, *pp))
            .collect();
        let next_x = x
            .iter()
            .copied()
            .filter(|xx| network_map.is_connected(v, *xx))
            .collect();
        bron_kerbosch(network_map, next_r, next_p, next_x, cliques);

        p.remove(&v);
        x.insert(v);
    }
}

fn largest_network(network_map: &NetworkMap) -> String {
    // implements the Bron-Kerbosch algorithm
    let r = BTreeSet::new();
    let p = network_map.computers().collect();
    let x = BTreeSet::new();
    let mut networks = Vec::new();
    bron_kerbosch(network_map, r, p, x, &mut networks);

    let largest = networks.into_iter().max_by_key(|n| n.len()).unwrap();
    let mut network = largest
        .into_iter()
        .map(|c| network_map.computer_name(c))
        .collect::<Vec<_>>();
    network.sort();
    network.join(",")
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The network map provides a list of every connection between two computers.
    let network_map = NetworkMap::from(input.as_str());

    // Part A: Find all the sets of three inter-connected computers. How many contain at least one
    // computer with a name that starts with t?
    let t_count = find_interconnected_computers_with_t(&network_map);
    solution.set_part_a(t_count);

    // Part B: The LAN party will be the largest set of computers that are all connected to each
    // other. The password to get into the LAN party is the name of every computer at the LAN
    // party, sorted alphabetically, then joined together with commas. What is the password to get
    // into the LAN party?
    let password = largest_network(&network_map);
    solution.set_part_b(password);

    solution
}
