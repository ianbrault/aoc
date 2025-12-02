/*
** src/puzzles/aoc2021/day18.rs
*/

use super::Solution;

use std::cmp;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum NumberType {
    Number(u8),
    Nested,
}

impl NumberType {
    fn number(&self) -> u8 {
        match self {
            Self::Number(n) => *n,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for NumberType {
    fn from(value: u8) -> Self {
        Self::Number(value)
    }
}

impl From<u32> for NumberType {
    fn from(value: u32) -> Self {
        Self::Number(value as u8)
    }
}

pub struct TreeNode<T> {
    _id: u64,
    pub data: T,
    // ID of parent node
    parent: Option<u64>,
    // IDs of children nodes
    pub children: Vec<u64>,
}

impl<T> TreeNode<T> {
    fn new(_id: u64, data: T, parent: Option<u64>) -> Self {
        let children = vec![];
        Self {
            _id,
            data,
            parent,
            children,
        }
    }

    fn find_child(&self, node_id: u64) -> Option<usize> {
        self.children
            .iter()
            .enumerate()
            .find(|&(_, &child_id)| child_id == node_id)
            .map(|(i, _)| i)
    }
}

pub struct Tree<T> {
    pub root: Option<u64>,
    nodes: Vec<Option<TreeNode<T>>>,
    // maps node IDs to their position in the nodes array
    node_positions: HashMap<u64, usize>,
    id_tracker: u64,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        let nodes = (0..64).map(|_| None).collect();
        let node_positions = HashMap::new();
        Self {
            root: None,
            nodes,
            node_positions,
            id_tracker: 0,
        }
    }

    pub fn node(&self, id: u64) -> Option<&TreeNode<T>> {
        let pos = self.node_positions[&id];
        self.nodes[pos].as_ref()
    }

    pub fn node_data(&self, id: u64) -> Option<&T> {
        self.node(id).map(|node| &node.data)
    }

    pub fn node_mut(&mut self, id: u64) -> Option<&mut TreeNode<T>> {
        let pos = self.node_positions[&id];
        self.nodes[pos].as_mut()
    }

    fn find_first_open_slot(&mut self) -> usize {
        for (i, node) in self.nodes.iter().enumerate() {
            if node.is_none() {
                return i;
            }
        }

        // no slot found, resize
        let size = self.nodes.len();
        self.nodes.resize_with(size * 2, Default::default);
        size
    }

    pub fn insert(&mut self, data: T, parent: Option<u64>) -> u64 {
        let id = self.id_tracker;
        let node = TreeNode::new(id, data, parent);

        // add and track the new node
        let pos = self.find_first_open_slot();
        self.nodes[pos] = Some(node);
        self.node_positions.insert(id, pos);
        self.id_tracker += 1;

        // if provided, hook the node up to its parent
        if let Some(parent_id) = parent {
            let parent_node = self.node_mut(parent_id).unwrap();
            parent_node.children.push(id);
        } else {
            self.root = Some(id);
        }

        id
    }

    pub fn remove(&mut self, node_id: u64) {
        if let Some(node) = self.node(node_id) {
            // unhook from the parent
            if let Some(parent_id) = node.parent {
                let parent = self.node_mut(parent_id).unwrap();
                let i = parent.find_child(node_id).unwrap();
                parent.children.remove(i);
            }

            // remove from the nodes and node position structures
            let pos = self.node_positions.remove(&node_id).unwrap();
            self.nodes[pos] = None;
        }
    }

    pub fn left_neighbor_node(&self, node_id: u64) -> Option<u64> {
        if let Some(node) = self.node(node_id) {
            // grab the parent
            if let Some(parent_id) = node.parent {
                let parent = self.node(parent_id).unwrap();
                // check the parent's children for a left neighbor
                let i = parent.find_child(node_id).unwrap();
                if i > 0 {
                    Some(parent.children[i - 1])
                } else {
                    // otherwise need to check the parent's parent
                    self.left_neighbor_node(parent_id)
                }
            } else {
                // otherwise there is no neighbor
                None
            }
        } else {
            None
        }
    }

    pub fn left_neighbor_leaf(&self, node_id: u64) -> Option<u64> {
        if let Some(neighbor_id) = self.left_neighbor_node(node_id) {
            let mut id = neighbor_id;
            let mut node = self.node(neighbor_id).unwrap();
            while !node.children.is_empty() {
                // check the rightmost child
                id = node.children[node.children.len() - 1];
                node = self.node(id).unwrap();
            }
            Some(id)
        } else {
            None
        }
    }

    pub fn right_neighbor_node(&self, node_id: u64) -> Option<u64> {
        if let Some(node) = self.node(node_id) {
            // grab the parent
            if let Some(parent_id) = node.parent {
                let parent = self.node(parent_id).unwrap();
                // check the parent's children for a right neighbor
                let i = parent.find_child(node_id).unwrap();
                if i < parent.children.len() - 1 {
                    Some(parent.children[i + 1])
                } else {
                    // otherwise need to check the parent's parent
                    self.right_neighbor_node(parent_id)
                }
            } else {
                // otherwise there is no neighbor
                None
            }
        } else {
            None
        }
    }

    pub fn right_neighbor_leaf(&self, node_id: u64) -> Option<u64> {
        if let Some(neighbor_id) = self.right_neighbor_node(node_id) {
            let mut id = neighbor_id;
            let mut node = self.node(neighbor_id).unwrap();
            while !node.children.is_empty() {
                // check the leftmost child
                id = node.children[0];
                node = self.node(id).unwrap();
            }
            Some(id)
        } else {
            None
        }
    }

    fn consume_tree(&mut self, tree: &Self, from_node: u64, into_node: u64)
    where
        T: Clone,
    {
        if let Some(node) = tree.node(from_node) {
            let new_id = self.insert(node.data.clone(), Some(into_node));
            for child_id in node.children.iter() {
                self.consume_tree(tree, *child_id, new_id);
            }
        }
    }

    // combines two trees under a single root
    pub fn combine_trees(tree_a: &Self, tree_b: &Self, root: T) -> Self
    where
        T: Clone,
    {
        let mut tree = Self::new();
        let root_id = tree.insert(root, None);
        // add the left and right trees
        if let Some(left_root) = tree_a.root {
            tree.consume_tree(tree_a, left_root, root_id);
        }
        if let Some(right_root) = tree_b.root {
            tree.consume_tree(tree_b, right_root, root_id);
        }

        tree
    }
}

struct SnailfishNumber {
    tree: Tree<NumberType>,
}

impl SnailfishNumber {
    fn parse_number(tree: &mut Tree<NumberType>, s: &str, node_id: u64, pos: &mut usize) {
        // skip the leading bracket
        *pos += 1;

        while *pos < s.len() {
            let c = s.chars().nth(*pos).unwrap();
            if c == ',' {
                // continue, not relevant for parsing
                *pos += 1;
            } else if c == ']' {
                // terminate
                *pos += 1;
                break;
            } else if c.is_ascii_digit() {
                // leaf node of the tree, insert and continue
                tree.insert(c.to_digit(10).unwrap().into(), Some(node_id));
                *pos += 1;
            } else if c == '[' {
                // add a branch point and recurse down another level
                let new_node = tree.insert(NumberType::Nested, Some(node_id));
                Self::parse_number(tree, s, new_node, pos);
            }
        }
    }

    fn find_nested_pair_rec(&self, depth: usize, node_id: u64) -> Option<u64> {
        let node = self.tree.node(node_id).unwrap();
        for child_id in node.children.iter() {
            let child_node = self.tree.node(*child_id).unwrap();
            if child_node.data == NumberType::Nested {
                if depth == 4 {
                    return Some(*child_id);
                } else if let Some(id) = self.find_nested_pair_rec(depth + 1, *child_id) {
                    return Some(id);
                }
            }
        }

        None
    }

    fn magnitude_rec(&self, node_id: u64) -> u64 {
        let node = self.tree.node(node_id).unwrap();
        match node.data {
            NumberType::Number(n) => n as u64,
            NumberType::Nested => {
                let left_id = node.children[0];
                let right_id = node.children[1];
                (3 * self.magnitude_rec(left_id)) + (2 * self.magnitude_rec(right_id))
            }
        }
    }

    fn magnitude(&self) -> u64 {
        match self.tree.root {
            Some(root_id) => {
                let node = self.tree.node(root_id).unwrap();
                let left_id = node.children[0];
                let right_id = node.children[1];
                (3 * self.magnitude_rec(left_id)) + (2 * self.magnitude_rec(right_id))
            }
            _ => unreachable!(),
        }
    }

    // finds the leftmost pair nested inside 4 pairs
    fn find_nested_pair(&self) -> Option<u64> {
        if let Some(root) = self.tree.root {
            self.find_nested_pair_rec(1, root)
        } else {
            None
        }
    }

    fn explode(mut self, node_id: u64) -> Self {
        let node = self.tree.node(node_id).unwrap();

        // grab the left and right elements of the nested pair
        let left_id = node.children[0];
        let right_id = node.children[1];
        let left = self.tree.node_data(left_id).unwrap().number();
        let right = self.tree.node_data(right_id).unwrap().number();

        // check for a left neighbor and add the left element to it, if found
        if let Some(left_neighbor_id) = self.tree.left_neighbor_leaf(left_id) {
            let node = self.tree.node_mut(left_neighbor_id).unwrap();
            // note: assumes that this is a number and not a nested pair
            node.data = (node.data.number() + left).into();
        }
        // check for a right neighbor and add the right element to it, if found
        if let Some(right_neighbor_id) = self.tree.right_neighbor_leaf(right_id) {
            let node = self.tree.node_mut(right_neighbor_id).unwrap();
            // note: assumes that this is a number and not a nested pair
            node.data = (node.data.number() + right).into();
        }

        // first remove the children
        self.tree.remove(left_id);
        self.tree.remove(right_id);
        // then replace the nested pair with 0
        // note: need to borrow mutably here separate from immutable borrows above
        self.tree.node_mut(node_id).unwrap().data = 0u8.into();

        self
    }

    fn find_big_pair_rec(&self, node_id: u64) -> Option<u64> {
        let node = self.tree.node(node_id).unwrap();
        match node.data {
            NumberType::Number(n) => {
                if n > 9 {
                    Some(node_id)
                } else {
                    None
                }
            }
            NumberType::Nested => {
                for child_id in node.children.iter() {
                    if let Some(id) = self.find_big_pair_rec(*child_id) {
                        return Some(id);
                    }
                }
                None
            }
        }
    }

    // finds a number greater than or equal to 10
    fn find_big_pair(&self) -> Option<u64> {
        if let Some(root) = self.tree.root {
            self.find_big_pair_rec(root)
        } else {
            None
        }
    }

    fn split(mut self, node_id: u64) -> Self {
        let node = self.tree.node_mut(node_id).unwrap();
        let n = match node.data {
            NumberType::Number(n) => n,
            _ => unreachable!(),
        };

        node.data = NumberType::Nested;
        self.tree.insert((n / 2).into(), Some(node_id));
        self.tree.insert(((n + 1) / 2).into(), Some(node_id));

        self
    }

    fn reduce_number(mut self) -> Self {
        let mut continue_reduction = true;
        while continue_reduction {
            continue_reduction = false;

            // first check for explode then check for split
            // either being found returns to the top of the loop
            if let Some(node_id) = self.find_nested_pair() {
                self = self.explode(node_id);
                continue_reduction = true;
            } else if let Some(node_id) = self.find_big_pair() {
                self = self.split(node_id);
                continue_reduction = true;
            }
        }

        self
    }

    fn to_string(&self, node_id: u64) -> String {
        if let Some(node) = self.tree.node(node_id) {
            match node.data {
                NumberType::Number(n) => n.to_string(),
                NumberType::Nested => {
                    let children = node
                        .children
                        .iter()
                        .map(|&child_id| self.to_string(child_id))
                        .collect::<Vec<_>>();
                    format!("[{}]", children.join(","))
                }
            }
        } else {
            String::new()
        }
    }
}

impl From<&str> for SnailfishNumber {
    fn from(value: &str) -> Self {
        // build up a tree representation
        let mut tree = Tree::new();
        let node_id = tree.insert(NumberType::Nested, None);

        Self::parse_number(&mut tree, value, node_id, &mut 0);
        Self { tree }
    }
}

impl std::ops::Add<Self> for &SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let tree = Tree::combine_trees(&self.tree, &rhs.tree, NumberType::Nested);
        let output = SnailfishNumber { tree };
        output.reduce_number()
    }
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(root_id) = self.tree.root {
            write!(f, "{}", self.to_string(root_id))
        } else {
            write!(f, "")
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let numbers = input
        .split('\n')
        .map(SnailfishNumber::from)
        .collect::<Vec<_>>();

    // Part A: Add up all of the snailfish numbers from the homework assignment in the order they
    // appear. What is the magnitude of the final sum?
    let mut sum = &numbers[0] + &numbers[1];
    for number in numbers.iter().skip(2) {
        sum = &sum + number;
    }
    solution.set_part_a(sum.magnitude());

    // Part B: What is the largest magnitude of any sum of two different snailfish numbers from the
    // homework assignment?
    let mut max_magnitude = 0;
    for i in 0..(numbers.len() - 1) {
        for j in (i + 1)..numbers.len() {
            let a = &numbers[i];
            let b = &numbers[j];
            let c = a + b;
            let d = b + a;
            max_magnitude = cmp::max(max_magnitude, cmp::max(c.magnitude(), d.magnitude()));
        }
    }
    solution.set_part_b(max_magnitude);

    solution
}
