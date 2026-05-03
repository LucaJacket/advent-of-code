pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
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
        if root_left != root_right {
            self.parent[root_left] = root_right;
            true
        } else {
            false
        }
    }
}
