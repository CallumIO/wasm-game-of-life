use std::fmt;
use wasm_bindgen::prelude::*;

/// use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifeCell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<LifeCell>,
}
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    LifeCell::Alive
                } else {
                    LifeCell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn next_iteration(&mut self) {
        let mut next = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let id = self.get_index(y, x);
                let cell = self.cells[id];
                let neighbours_alive = self.neighbour_alive_count(y, x);
                let next_cell = match (cell, neighbours_alive) {
                    (LifeCell::Alive, x) if x < 2 => LifeCell::Dead,
                    (LifeCell::Alive, 2) | (LifeCell::Alive, 3) => LifeCell::Alive,
                    (LifeCell::Alive, x) if x > 3 => LifeCell::Dead,
                    (LifeCell::Dead, 3) => LifeCell::Alive,
                    (otherwise, _) => otherwise,
                };
                next[id] = next_cell;
            }
        }
        self.cells = next;
    }
}

impl Universe {
    fn get_index(&self, y: u32, x: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn neighbour_alive_count(&self, y: u32, x: u32) -> u8 {
        let mut c = 0;
        for row in [self.height - 1, 0, 1].iter().cloned() {
            for col in [self.width - 1, 0, 1].iter().cloned() {
                if row == 0 && col == 0 {
                    continue;
                }
                let nr = (row + y) % self.height;
                let nc = (col + x) % self.width;
                let idx = self.get_index(nr, nc);
                c += self.cells[idx] as u8;
            }
        }
        c
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in row {
                let s = if cell == LifeCell::Dead { '◻' } else { '◼' };
                write!(f, "{}", s)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
