use std::{fmt::Display, usize};

pub struct Conways {
    world: Vec<Vec<u8>>,
    width: usize,
    height: usize
}

impl Conways {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            world: vec![vec![0; width]; height],
            width,
            height
        }
    }

    pub fn set_alive(&mut self, row: usize, column: usize) {
        self.world[row][column] = 1;
    }

    pub fn tick(&mut self) {
        let mut new_world: Vec<Vec<u8>> = vec![vec![0; self.width]; self.height];
        for row in 0..self.height {
            for column in 0..self.width {
                let neighbours_count = self.neighbours_count(row, column);

                let new_state = match (neighbours_count, self.world[row][column]) {
                    // underpopulation
                    (0..=1, 1) => 0,
                    // survives
                    (2..=3, 1) => 1,
                    // overpopulation
                    (4.., 1) => 0,
                    // reproduction
                    (3, 0) => 1,
                    // no reproduction
                    _ => 0 
                };

                println!("new state {new_state} {neighbours_count} {row} {column}");

                new_world[row][column] = new_state;
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
                    None => 0 as u8,
                    Some(v) => *v.get(y as usize).unwrap_or(&0)
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
            writeln!(f, "")?;
        };
        Ok(())
    }
}
