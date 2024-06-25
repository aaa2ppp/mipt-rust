#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: vec![T::default(); rows * cols],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        let mut grid: Vec<T> = Vec::from(grid);
        grid.resize(rows * cols, T::default());
        Self { rows, cols, grid }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        let idx = row * self.cols + col;
        &self.grid[idx] // XXX may be panic!
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        let idx = row * self.cols + col;
        if idx < self.grid.len() {
            self.grid[idx] = value;
        }
    }

    // pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
    //     let row_start = if row == 0 { 0 } else { row - 1 };
    //     let col_start = if col == 0 { 0 } else { col - 1 };
    //     let row_end = self.rows.min(row + 2);
    //     let col_end = self.cols.min(col + 2);

    //     let mut neigs: Vec<(usize, usize)> = Vec::with_capacity(8);
    //     for r in row_start..row_end {
    //         for c in col_start..col_end {
    //             if r != row || c != col {
    //                 neigs.push((r, c))
    //             }
    //         }
    //     }
    //     neigs
    // }

    pub fn neighbours(&self, row: usize, col: usize) -> NeighboursIterator {
        NeighboursIterator::new(self.rows, self.cols, row, col)
    }
}

#[derive(Debug)]
pub struct NeighboursIterator {
    row: usize,
    col: usize,
    // row_start: usize,
    col_start: usize,
    row_end: usize,
    col_end: usize,
    r: usize,
    c: usize,
}

impl NeighboursIterator {
    fn new(rows: usize, cols: usize, row: usize, col: usize) -> NeighboursIterator {
        let row_start = if row == 0 { 0 } else { row - 1 };
        let col_start = if col == 0 { 0 } else { col - 1 };

        NeighboursIterator {
            row,
            col,
            // row_start,
            col_start,
            row_end: rows.min(row + 2),
            col_end: cols.min(col + 2),
            r: row_start,
            c: col_start,
        }
    }

    fn next_pos(&mut self) {
        self.c += 1;
        if self.c == self.col_end {
            self.r += 1;
            self.c = self.col_start;
        }
    }
}

impl Iterator for NeighboursIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        while self.r < self.row_end {
            if self.r == self.row && self.c == self.col {
                self.next_pos();
                continue;
            }
            let res = Some((self.r, self.c));
            self.next_pos();
            return res;
        }
        None
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid0: Grid<Cell>,
    grid1: Grid<Cell>,
    parity: bool,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        let (rows, cols) = grid.size();
        GameOfLife {
            grid0: grid,
            grid1: Grid::new(rows, cols),
            parity: true,
        }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        if self.parity {
            &self.grid0
        } else {
            &self.grid1
        }
    }

    pub fn step(&mut self) {
        let (cur_grid, next_grid) = if self.parity {
            (&self.grid0, &mut self.grid1)
        } else {
            (&self.grid1, &mut self.grid0)
        };

        let (rows, cols) = cur_grid.size();
        for row in 0..rows {
            for col in 0..cols {
                let count: usize = cur_grid
                    .neighbours(row, col)
                    .map(|(row, col)| {
                        if *cur_grid.get(row, col) == Cell::Alive {
                            1
                        } else {
                            0
                        }
                    })
                    .sum();

                let value = if count == 3 || count == 2 && *cur_grid.get(row, col) == Cell::Alive {
                    Cell::Alive
                } else {
                    Cell::Dead
                };

                next_grid.set(value, row, col);
            }
        }

        self.parity = !self.parity;
    }
}
