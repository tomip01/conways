use std::fmt::Display;

#[derive(Clone, Copy)]
enum CellState {
    Alive,
    Dead
}

pub struct Conways {
    world: Vec<Vec<CellState>>,
    width: usize,
    height: usize
}

impl Conways {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            world: vec![vec![CellState::Dead; width]; height],
            width,
            height
        }
    }

    pub fn set_alive(&mut self, row: usize, column: usize) {
        self.world[row][column] = CellState::Alive;
    }

    pub fn tick(&mut self) {
        let mut new_world: Vec<Vec<CellState>> = vec![vec![CellState::Dead; self.width]; self.height];
        for (row_index, row) in new_world.iter_mut().enumerate().take(self.height) {
            for (column_index, cell) in row.iter_mut().enumerate().take(self.width) {
                let neighbours_count = self.neighbours_count(row_index, column_index);

                *cell = match (neighbours_count, self.world[row_index][column_index]) {
                    // underpopulation
                    (0..=1, CellState::Alive) => CellState::Dead,
                    // survives
                    (2..=3, CellState::Alive) => CellState::Alive,
                    // overpopulation
                    (4.., CellState::Alive) => CellState::Dead,
                    // reproduction
                    (3, CellState::Dead) => CellState::Alive,
                    // no reproduction
                    _ => CellState::Dead 
                };
            }
        }

        self.world = new_world;
    }

    fn neighbours_count(&self, row: usize, column: usize) -> u8 {
        let mut res = 0;

        for height in -1..=1 {
            for width in -1..=1 {
                let x = row as i32 + height;
                let y = column as i32 + width;

                if x < 0 || y < 0 {
                    continue;
                }

                if height == 0 && width == 0 {
                    continue;
                }

                res += match self.world.get(x as usize) {
                    None => 0,
                    Some(v) => match v.get(y as usize) {
                        Some(CellState::Alive) => 1,
                        _ => 0
                    }
                };
            }
        }
        res
    }
}

impl Display for Conways {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let world = &self.world;
        for row in world {
            for cell in row {
                write!(f, "{cell} ")?;
            }
            writeln!(f)?;
        };
        Ok(())
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Alive => write!(f, "1"),
            CellState::Dead => write!(f, "0")
        }
    }
}
