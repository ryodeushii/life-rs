use std::fmt::{self, Display, Formatter};

use serde::Serialize;

use super::cell::{Cell, CellState, Cells};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
    pub prev_cells: Vec<Cell>,
    pub generation: u32,
    pub max_generations: u32,
    pub circular: bool,
}

impl Grid {
    pub fn new(width: u32, height: u32, randomize: Option<bool>, circular: bool) -> Grid {
        let randomize = randomize.unwrap_or(false);
        let cells = (0..width * height)
            .map(|_| {
                if randomize {
                    Cell::random_state()
                } else {
                    Cell::new_dead()
                }
            })
            .collect();

        Grid {
            generation: 0,
            circular,
            max_generations: width * height,
            width,
            height,
            cells,
            prev_cells: Vec::new(),
        }
    }

    pub fn set_custom_state(&mut self, cells: Vec<Cell>) {
        self.cells = cells;
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Option<Cell> {
        let mut x = x;
        let mut y = y;
        if self.circular {
            // lets assume, that we have a grid as circular rectangle
            // so if we go out of bounds, we go to the opposite side
            x = if x < 0 {
                self.width as i32 + x
            } else if x >= self.width as i32 {
                x - self.width as i32
            } else {
                x
            };
            y = if y < 0 {
                self.height as i32 + y
            } else if y >= self.height as i32 {
                y - self.height as i32
            } else {
                y
            };
        }
        let idx: i32 = y * self.width.try_into().unwrap_or(0) + x;
        if idx as u32 >= self.cells.len() as u32 || x < 0 || y < 0 {
            return None;
        }

        match self.cells.get(idx as usize) {
            Some(cell) => Some(cell.clone()),
            None => None,
        }
    }

    pub fn get_neighbors(&self, x: i32, y: i32) -> Cells {
        let cell = self.get_cell(x, y).unwrap();
        let mut neighbors = Cells::new(Vec::with_capacity(8), cell);

        let dead_cell = Cell::new_dead();
        // top-left
        neighbors.push(self.get_cell(x - 1, y - 1).unwrap_or(dead_cell.clone()));
        // top
        neighbors.push(self.get_cell(x, y - 1).unwrap_or(dead_cell.clone()));
        // top-right
        neighbors.push(self.get_cell(x + 1, y - 1).unwrap_or(dead_cell.clone()));
        // left
        neighbors.push(self.get_cell(x - 1, y).unwrap_or(dead_cell.clone()));
        // right
        neighbors.push(self.get_cell(x + 1, y).unwrap_or(dead_cell.clone()));
        // bottom-left
        neighbors.push(self.get_cell(x - 1, y + 1).unwrap_or(dead_cell.clone()));
        // bottom
        neighbors.push(self.get_cell(x, y + 1).unwrap_or(dead_cell.clone()));
        // bottom-right
        neighbors.push(self.get_cell(x + 1, y + 1).unwrap_or(dead_cell.clone()));

        neighbors
    }

    pub fn next_generation(&mut self) -> bool {
        // if we reached the max number of generations, exit
        if self.generation >= self.max_generations {
            println!(
                "Max generations reached ({}), exiting...",
                self.max_generations
            );
            return false;
        }
        self.prev_cells = self.cells.clone();
        let mut next_cells = Vec::with_capacity(self.cells.len());

        for (i, cell) in self.cells.iter().enumerate() {
            let x = i as i32 % self.width as i32;
            let y = i as i32 / self.width as i32;

            let neighbors = self.get_neighbors(x, y);
            let alive_neighbors = neighbors.count_alive();

            let next_cell = match cell.state {
                CellState::Alive => {
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        Cell::new_dead()
                    } else {
                        Cell::new_alive()
                    }
                }
                CellState::Dead => {
                    if alive_neighbors == 3 {
                        Cell::new_alive()
                    } else {
                        Cell::new_dead()
                    }
                }
            };

            next_cells.push(next_cell);
        }

        self.cells = next_cells;
        if self.prev_cells == self.cells {
            println!("Stabilized at generation {}", self.generation);

            return false;
        }
        self.generation += 1;
        true
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for (i, cell) in self.cells.iter().enumerate() {
            if i % self.width as usize == 0 {
                output.push_str("\n");
            }

            output.push_str(&format!("{} ", cell));
        }

        write!(f, "{}", output)
    }
}
