use crate::cell::Cell;

pub struct Universe {
    pub cells: Vec<Cell>,
    width: u32,
    height: u32,
}

impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        let mut cells: Vec<Cell> = (0..width * height).map(|_| Cell::Dead).collect();
        let starting_pattern: Vec<(i32, i32)> = vec![(5, 4), (7, 4), (5, 5), (6, 5), (5, 6)];
        for p in starting_pattern {
            let x = p.0 as usize;
            let y = p.1 as usize;
            cells[x * width as usize + y] = Cell::Alive;
        }

        Self {
            width,
            height,
            cells,
        }
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn count_live_neighbors(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let n_row = (row + delta_row) % self.height;
                let n_col = (col + delta_col) % self.width;
                let idx = self.get_index(n_row, n_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_cells = self.count_live_neighbors(row, col);
                let next_cell = match (cell, live_cells) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
}
