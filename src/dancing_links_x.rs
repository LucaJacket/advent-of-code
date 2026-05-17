pub struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,

    col: usize,
    row: usize,
}

pub struct Column {
    index: usize,
    size: usize,
    is_required: bool,
}

pub struct DancingLinksX {
    nodes: Vec<Node>,
    columns: Vec<Column>,
    rows: usize,
    header: usize,
    required: usize,
}

impl DancingLinksX {
    pub fn new(num_columns: usize, required: usize) -> Self {
        let mut nodes = Vec::with_capacity(num_columns + 1);
        let mut columns = Vec::with_capacity(num_columns);

        nodes.push(Node {
            left: 0,
            right: 0,
            up: 0,
            down: 0,
            col: 0,
            row: 0,
        });

        for i in 0..num_columns {
            let idx = nodes.len();

            nodes.push(Node {
                left: idx - 1,
                right: 0,
                up: idx,
                down: idx,
                col: i,
                row: usize::MAX,
            });

            nodes[idx - 1].right = idx;
            nodes[0].left = idx;

            columns.push(Column {
                index: idx,
                size: 0,
                is_required: i < required,
            });
        }

        Self {
            nodes,
            columns,
            rows: 0,
            header: 0,
            required,
        }
    }

    pub fn add_row(&mut self, columns: &[usize]) {
        let first = self.nodes.len();

        for &c in columns {
            self.columns[c].size += 1;

            let idx = self.nodes.len();
            let col = self.columns[c].index;

            self.nodes.push(Node {
                left: if idx == first { idx } else { idx - 1 },
                right: first,
                up: self.nodes[col].up,
                down: col,
                col: c,
                row: self.rows,
            });

            let up = self.nodes[col].up;

            self.nodes[col].up = idx;
            self.nodes[up].down = idx;

            if idx != first {
                self.nodes[idx - 1].right = idx;
                self.nodes[first].left = idx;
            }
        }

        self.rows += 1;
    }

    pub fn cover(&mut self, col: usize) {
        let left = self.nodes[col].left;
        let right = self.nodes[col].right;

        self.nodes[left].right = right;
        self.nodes[right].left = left;

        let mut row = self.nodes[col].down;

        while row != col {
            let mut node = self.nodes[row].right;

            while node != row {
                let up = self.nodes[node].up;
                let down = self.nodes[node].down;

                self.nodes[up].down = down;
                self.nodes[down].up = up;

                self.columns[self.nodes[node].col].size -= 1;

                node = self.nodes[node].right;
            }

            row = self.nodes[row].down;
        }
    }

    pub fn uncover(&mut self, col: usize) {
        let mut row = self.nodes[col].up;

        while row != col {
            let mut node = self.nodes[row].left;

            while node != row {
                let up = self.nodes[node].up;
                let down = self.nodes[node].down;

                self.nodes[up].down = node;
                self.nodes[down].up = node;

                self.columns[self.nodes[node].col].size += 1;

                node = self.nodes[node].left;
            }

            row = self.nodes[row].up;
        }

        let left = self.nodes[col].left;
        let right = self.nodes[col].right;

        self.nodes[left].right = col;
        self.nodes[right].left = col;
    }

    pub fn search(
        &mut self,
        solution: &mut Vec<usize>,
        satisfied: &mut usize,
    ) -> Option<Vec<usize>> {
        if *satisfied >= self.required {
            return Some(solution.clone());
        }

        let col = self.choose_column();

        *satisfied += 1;
        self.cover(col);

        let mut row = self.nodes[col].down;

        while row != col {
            solution.push(self.nodes[row].row);

            let mut node = self.nodes[row].right;
            while node != row {
                let col = self.nodes[node].col;
                if self.columns[col].is_required {
                    *satisfied += 1;
                }
                let col = self.columns[col].index;
                self.cover(col);

                node = self.nodes[node].right;
            }

            if let Some(solution) = self.search(solution, satisfied) {
                return Some(solution);
            }

            solution.pop();

            let mut node = self.nodes[row].left;
            while node != row {
                let col = self.nodes[node].col;
                if self.columns[col].is_required {
                    *satisfied -= 1;
                }
                let col = self.columns[col].index;
                self.uncover(col);

                node = self.nodes[node].left;
            }

            row = self.nodes[row].down;
        }

        self.uncover(col);
        *satisfied -= 1;

        None
    }

    pub fn choose_column(&self) -> usize {
        let mut best = self.nodes[self.header].right;
        let mut min_size = usize::MAX;

        let mut col = best;

        while col != self.header {
            let c = self.nodes[col].col;

            if self.columns[c].is_required {
                let size = self.columns[c].size;

                if size < min_size {
                    min_size = size;
                    best = col;
                }
            }

            col = self.nodes[col].right;
        }

        best
    }

    pub fn solve(&mut self) -> Option<Vec<usize>> {
        let mut solution = Vec::new();
        let mut satisfied = 0;

        self.search(&mut solution, &mut satisfied)
    }
}
