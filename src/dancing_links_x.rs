//!
//! Dancing Links data structure implementation:
//! this version is not 0 indexed;
//! this version stores nodes in a Vec so a Node use indexes to refer to its neighbors;
//! it also has a small addition to deal with problems where not all columns are required;
//! Node stores references to its neighbors and the column header (through indexes)
//! and the row index;
//! the final struct stores nodes, the sizes of the columns, the number of rows and the number of
//! required columns;
//! the row header is always at index 0, the n column headers are always at index 1 to n;
//! to match the same indexing of nodes, columns has num_columns + 1 elements.
//!

pub struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,

    col: usize,
    row: usize,
}

pub struct DancingLinksX {
    nodes: Vec<Node>,

    columns: Vec<usize>,
    rows: usize,

    required: usize,
}

impl DancingLinksX {
    pub fn new(num_columns: usize, required: usize) -> Self {
        let mut nodes: Vec<Node> = Vec::with_capacity(num_columns + 1);

        nodes.push(Node {
            left: 0,
            right: 0,
            up: 0,
            down: 0,

            col: 0,
            row: 0,
        });

        for idx in 1..=num_columns {
            nodes.push(Node {
                left: idx - 1,
                right: 0,
                up: idx,
                down: idx,

                col: idx,
                row: 0,
            });

            let left = nodes[idx].left;
            let right = nodes[idx].right;

            nodes[left].right = idx;
            nodes[right].left = idx;
        }

        Self {
            nodes,
            columns: vec![0; num_columns + 1],
            rows: 1,
            required,
        }
    }

    pub fn add_row(&mut self, row: impl IntoIterator<Item = usize>) {
        let mut cols = row.into_iter();

        let col = cols.next().unwrap();
        let first = self.nodes.len();

        self.nodes.push(Node {
            left: first,
            right: first,
            up: self.nodes[col].up,
            down: col,

            col,
            row: self.rows,
        });

        let up = self.nodes[first].up;
        let down = self.nodes[first].down;

        self.nodes[up].down = first;
        self.nodes[down].up = first;

        self.columns[col] += 1;

        for col in cols {
            let idx = self.nodes.len();

            self.nodes.push(Node {
                left: idx - 1,
                right: first,
                up: self.nodes[col].up,
                down: col,

                col,
                row: self.rows,
            });

            let left = self.nodes[idx].left;
            let right = self.nodes[idx].right;
            let up = self.nodes[idx].up;
            let down = self.nodes[idx].down;

            self.nodes[left].right = idx;
            self.nodes[right].left = idx;
            self.nodes[up].down = idx;
            self.nodes[down].up = idx;

            self.columns[col] += 1;
        }

        self.rows += 1;
    }

    fn cover(&mut self, col: usize) {
        let left = self.nodes[col].left;
        let right = self.nodes[col].right;

        self.nodes[left].right = right;
        self.nodes[right].left = left;

        let mut row = self.nodes[col].down;
        while row != col {
            let mut node = self.nodes[row].right;
            while node != row {
                self.columns[self.nodes[node].col] -= 1;

                let up = self.nodes[node].up;
                let down = self.nodes[node].down;

                self.nodes[up].down = down;
                self.nodes[down].up = up;

                node = self.nodes[node].right;
            }
            row = self.nodes[row].down;
        }
    }

    fn uncover(&mut self, col: usize) {
        let mut row = self.nodes[col].up;
        while row != col {
            let mut node = self.nodes[row].left;
            while node != row {
                let up = self.nodes[node].up;
                let down = self.nodes[node].down;

                self.nodes[up].down = node;
                self.nodes[down].up = node;

                self.columns[self.nodes[node].col] += 1;

                node = self.nodes[node].left;
            }
            row = self.nodes[row].up;
        }

        let left = self.nodes[col].left;
        let right = self.nodes[col].right;

        self.nodes[left].right = col;
        self.nodes[right].left = col;
    }

    fn search(&mut self, solution: &mut Vec<usize>, satisfied: &mut usize) -> Option<Vec<usize>> {
        if *satisfied >= self.required {
            return Some(solution.clone());
        }

        let col = self.choose_column();

        if col <= self.required {
            *satisfied += 1;
        }
        self.cover(col);

        let mut row = self.nodes[col].down;
        while row != col {
            let mut node = self.nodes[row].right;
            while node != row {
                if self.nodes[node].col <= self.required {
                    *satisfied += 1;
                }
                self.cover(self.nodes[node].col);
                node = self.nodes[node].right;
            }

            solution.push(self.nodes[row].row);
            if let Some(solution) = self.search(solution, satisfied) {
                return Some(solution);
            }
            solution.pop();

            let mut node = self.nodes[row].left;
            while node != row {
                self.uncover(self.nodes[node].col);
                if self.nodes[node].col <= self.required {
                    *satisfied -= 1;
                }
                node = self.nodes[node].left;
            }
            row = self.nodes[row].down;
        }

        self.uncover(col);
        if col <= self.required {
            *satisfied -= 1;
        }

        None
    }

    fn choose_column(&self) -> usize {
        let mut best = self.nodes[0].right;
        let mut min_size = self.columns[best];

        let mut col = self.nodes[best].right;
        while col != 0 && col <= self.required {
            let size = self.columns[col];
            if size < min_size {
                min_size = size;
                best = col;
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
