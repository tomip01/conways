use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Debug)]
enum CellState {
    Alive,
    Dead,
}

pub struct ConwaysMap {
    grid: Vec<Vec<CellState>>,
    width: usize,
    height: usize,
}

impl ConwaysMap {
    pub fn new(width: usize, height: usize) -> Self {
        // initialize all cells as Dead
        Self {
            grid: vec![vec![CellState::Dead; width]; height],
            width,
            height,
        }
    }

    pub fn set_alive(&mut self, row: usize, column: usize) {
        if let Some(row_grid) = self.grid.get_mut(row) {
            if let Some(cell) = row_grid.get_mut(column) {
                *cell = CellState::Alive;
            }
        }
    }

    pub fn set_dead(&mut self, row: usize, column: usize) {
        if let Some(row_grid) = self.grid.get_mut(row) {
            if let Some(cell) = row_grid.get_mut(column) {
                *cell = CellState::Dead;
            }
        }
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        matches!(self.get(x, y), Some(CellState::Alive))
    }

    fn get(&self, row: usize, column: usize) -> Option<CellState> {
        if let Some(row_grid) = self.grid.get(row) {
            if let Some(cell) = row_grid.get(column) {
                return Some(*cell);
            }
        }
        None
    }

    pub fn tick(&mut self) {
        let mut new_world: Vec<Vec<CellState>> =
            vec![vec![CellState::Dead; self.width]; self.height];
        for (row_index, row) in new_world.iter_mut().enumerate().take(self.height) {
            for (column_index, cell) in row.iter_mut().enumerate().take(self.width) {
                let neighbours_count = self.neighbours_count(row_index, column_index);

                *cell = match (neighbours_count, self.grid[row_index][column_index]) {
                    // underpopulation
                    (0..=1, CellState::Alive) => CellState::Dead,
                    // survives
                    (2..=3, CellState::Alive) => CellState::Alive,
                    // overpopulation
                    (4.., CellState::Alive) => CellState::Dead,
                    // reproduction
                    (3, CellState::Dead) => CellState::Alive,
                    // no reproduction
                    _ => CellState::Dead,
                };
            }
        }

        self.grid = new_world;
    }

    fn neighbours_count(&self, row: usize, column: usize) -> u8 {
        let mut res = 0;

        for height in -1..=1 {
            for width in -1..=1 {
                let x = row as i32 + height;
                let y = column as i32 + width;

                // if both zeros, is the same cell, not a neighbour
                if height == 0 && width == 0 {
                    continue;
                }

                res += match self.get(x as usize, y as usize) {
                    Some(CellState::Alive) => 1,
                    _ => 0,
                };
            }
        }
        res
    }
}

impl Display for ConwaysMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let world = &self.grid;
        for row in world {
            for cell in row {
                write!(f, "{cell} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Alive => write!(f, "1"),
            CellState::Dead => write!(f, "0"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_game() {
        let conways = ConwaysMap::new(10, 10);
        assert_eq!(conways.height, 10);
        assert_eq!(conways.width, 10);
        assert_eq!(conways.grid.len(), 10);
        assert_eq!(conways.grid.first().unwrap().len(), 10);
    }

    #[test]
    fn correctly_setting_cells() {
        let dim = 10;
        let mut conway = ConwaysMap::new(dim, dim);
        conway.set_alive(1, 1);

        assert_eq!(conway.get(1, 1).unwrap(), CellState::Alive);

        // all Other cells are Dead
        for i in 0..dim {
            for j in 0..dim {
                if i == 1 && j == 1 {
                    continue;
                }
                assert_eq!(conway.get(i, j).unwrap(), CellState::Dead);
            }
        }
    }

    #[test]
    fn correctly_setting_dead_cells() {
        let dim = 10;
        let mut conway = ConwaysMap::new(dim, dim);
        conway.set_alive(1, 1);
        conway.set_dead(1, 1);

        // all cells are Dead
        for i in 0..dim {
            for j in 0..dim {
                assert_eq!(conway.get(i, j).unwrap(), CellState::Dead);
            }
        }
    }

    #[test]
    fn only_counts_alive_neighbours() {
        let dim = 10;
        let mut conway = ConwaysMap::new(dim, dim);
        conway.set_alive(1, 1);
        assert_eq!(conway.neighbours_count(1, 1), 0);
        assert_eq!(conway.neighbours_count(1, 2), 1);
        conway.set_alive(1, 2);
        assert_eq!(conway.neighbours_count(2, 1), 2);
    }

    #[test]
    fn underpopulation_rule() {
        let dim = 10;
        let mut conway = ConwaysMap::new(dim, dim);
        conway.set_alive(1, 1);
        conway.tick();
        assert_eq!(conway.get(1, 1).unwrap(), CellState::Dead);
        assert_eq!(conway.get(0, 1).unwrap(), CellState::Dead);
    }

    #[test]
    fn survives_rule() {
        let dim = 10;
        let mut conway = ConwaysMap::new(dim, dim);
        // for two
        conway.set_alive(1, 1);
        conway.set_alive(1, 2);
        conway.set_alive(1, 0);
        conway.tick();
        assert_eq!(conway.get(1, 1).unwrap(), CellState::Alive);

        // for three neighbours
        conway.set_alive(5, 5);
        conway.set_alive(5, 6);
        conway.set_alive(5, 4);
        conway.set_alive(4, 5);
        conway.tick();
        assert_eq!(conway.get(5, 5).unwrap(), CellState::Alive);
    }

    #[test]
    fn overpopulation_rule() {
        let dim = 10;
        let mut conway = ConwaysMap::new(dim, dim);
        conway.set_alive(1, 1);
        conway.set_alive(1, 2);
        conway.set_alive(1, 0);
        conway.set_alive(2, 1);
        conway.set_alive(0, 1);
        conway.tick();
        assert_eq!(conway.get(1, 1).unwrap(), CellState::Dead);
    }

    #[test]
    fn reproduction_rule() {
        let dim = 10;
        let mut conway = ConwaysMap::new(dim, dim);
        conway.set_alive(1, 2);
        conway.set_alive(1, 0);
        conway.set_alive(2, 1);
        conway.tick();
        assert_eq!(conway.get(1, 1).unwrap(), CellState::Alive);
    }
}
