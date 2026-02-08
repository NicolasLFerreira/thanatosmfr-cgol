use std::collections::HashSet;
use crate::types::cell_coord::CellCoord;

pub struct CellConfiguration {
    internal_cells: HashSet<CellCoord>,
}

// Instantiation
impl CellConfiguration {
    pub fn new() -> Self {
        Self {
            internal_cells: HashSet::new(),
        }
    }
}

// Crud stuff
impl CellConfiguration {
    pub fn is_alive(&self, coord: CellCoord) -> bool {
        self.internal_cells.contains(&coord)
    }

    pub fn spawn(&mut self, coord: CellCoord) {
        self.internal_cells.insert(coord);
    }

    pub fn despawn(&mut self, coord: CellCoord) {
        self.internal_cells.remove(&coord);
    }

    pub fn iter(&self) -> impl Iterator<Item = CellCoord> {
        self.internal_cells.iter().copied()
    }
}