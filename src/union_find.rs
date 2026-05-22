use std::cmp::Ordering::{Equal, Greater, Less};

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, node: usize) -> usize {
        if self.parent[node] != node {
            self.parent[node] = self.find(self.parent[node]);
        }
        self.parent[node]
    }

    pub fn union(&mut self, left: usize, right: usize) -> bool {
        let root_left = self.find(left);
        let root_right = self.find(right);
        if root_left == root_right {
            return false;
        }
        match self.rank[root_left].cmp(&self.rank[root_right]) {
            Less => self.parent[root_left] = root_right,
            Greater => self.parent[root_right] = root_left,
            Equal => {
                self.parent[root_left] = root_right;
                self.rank[root_right] += 1;
            }
        }
        true
    }
}
