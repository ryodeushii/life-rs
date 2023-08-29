use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Cell {
    pub state: CellState,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Cells(Vec<Cell>, Cell);

impl Cells {
    pub fn new(cells: Vec<Cell>, cell: Cell) -> Cells {
        Cells(cells, cell)
    }
    pub fn count_alive(self) -> usize {
        let mut count = 0;
        for cell in self.0 {
            if cell.is_alive() {
                count += 1;
            }
        }
        count
    }
    pub fn push(&mut self, cell: Cell) {
        self.0.push(cell);
    }
}

impl Display for Cells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cells = format!(
            "\n{} {} {}\n{} {} {}\n{} {} {}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.1,
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7]
        );

        write!(f, "{}", cells)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new()
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cell = match self.state {
            CellState::Alive => "■",
            CellState::Dead => "□",
        };

        write!(f, "{}", cell)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum CellState {
    Alive,
    Dead,
}

impl Cell {
    pub fn new() -> Cell {
        Cell::default()
    }

    pub fn new_dead() -> Cell {
        Cell {
            state: CellState::Dead,
        }
    }

    pub fn new_alive() -> Cell {
        Cell {
            state: CellState::Alive,
        }
    }

    pub fn random_state() -> Cell {
        // generate random number 0..100, if > 50 - alive, else - dead
        let random_number = rand::random::<i8>();
        if random_number >= 0 {
            Cell::new_alive()
        } else {
            Cell::new_dead()
        }
    }

    pub fn is_alive(&self) -> bool {
        self.state == CellState::Alive
    }
}
