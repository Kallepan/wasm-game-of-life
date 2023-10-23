mod utils;

use std::fmt;
use js_sys;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Change {
    row: u32,
    col: u32,
    state: bool,
}

#[wasm_bindgen]
impl Change {
    pub fn row(&self) -> u32 {
        self.row
    }

    pub fn col(&self) -> u32 {
        self.col
    }

    pub fn state(&self) -> bool {
        self.state
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
    old_cells: FixedBitSet,
    changes: Vec<Change>,
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height 
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        // row * width + col
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };
    
        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };
    
        let west = if col == 0 {
            self.width - 1
        } else {
            col - 1
        };
    
        let east = if col == self.width - 1 {
            0
        } else {
            col + 1
        };
    
        let nw = self.get_index(north, west);
        count += self.old_cells[nw] as u8;
    
        let n = self.get_index(north, col);
        count += self.old_cells[n] as u8;
    
        let ne = self.get_index(north, east);
        count += self.old_cells[ne] as u8;
    
        let w = self.get_index(row, west);
        count += self.old_cells[w] as u8;
    
        let e = self.get_index(row, east);
        count += self.old_cells[e] as u8;
    
        let sw = self.get_index(south, west);
        count += self.old_cells[sw] as u8;
    
        let s = self.get_index(south, col);
        count += self.old_cells[s] as u8;
    
        let se = self.get_index(south, east);
        count += self.old_cells[se] as u8;
    
        count
    }

    pub fn tick(&mut self) {
        // timer
        let _timer = utils::Timer::new("Universe::tick");

        // Overwrite old cells with current cells
        let _timer = utils::Timer::new("swap cells");
        self.old_cells = self.cells.clone();
        
        {
            let _timer = utils::Timer::new("new generation");
            // Loop through all cells
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);
    
                    self.cells.set(idx, match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (true, x) if x < 2 => false,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (true, 2) | (true, 3) => true,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (true, x) if x > 3 => false,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (false, 3) => true,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    });

                    // populate changes
                    if self.cells[idx] != self.old_cells[idx] {
                        self.changes.push(Change {
                            row,
                            col,
                            state: self.cells[idx],
                        });
                    }
                }
            }
        }
    }

    pub fn get_changes(&self) -> *const u32 {
        // Convert the vector to a JavaScript array
        self.changes.as_ptr() as *const u32
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 128;
        let height = 128;

        // Initialize with all dead cells
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        let mut changes = Vec::new();

        // Randomly initialize the cells
        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
            changes.push(Change {
                row: i as u32 / width,
                col: i as u32 % width,
                state: cells[i],
            });
        }
        let old_cells = cells.clone();
        
        log!("Created a universe with {} cells of {} width and {} height.", cells.len(), width, height);
        Universe {
            width,
            height,
            cells,
            old_cells,
            changes,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn get_changes_len(&self) -> usize {
        self.changes.len()
    }

    pub fn free_changes(&mut self) {
        self.changes.clear();
    }

    /// Responsive functions
    ///    
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.toggle(idx);
        self.changes.push(Change {
            row,
            col,
            state: self.cells[idx],
        });
    }

    pub fn randomize(&mut self) {
        for i in 0..self.cells.len() {
            self.cells.set(i, js_sys::Math::random() < 0.5);
            self.changes.push(Change {
                row: i as u32 / self.width,
                col: i as u32 % self.width,
                state: self.cells[i],
            });
        }
    }

    pub fn clear(&mut self) {
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
        for i in 0..self.cells.len() {
            self.changes.push(Change {
                row: i as u32 / self.width,
                col: i as u32 % self.width,
                state: self.cells[i],
            });
        }
    }
}

// Not exposed to JavaScript
impl Universe {
    /// Get the dead and alive cells as a pointer to the cells memory.
    pub fn get_cells(&self) -> &[u32] {
        // Convert to a slice
        &self.cells.as_slice()
    }
    
    /// Set cells to be alive in a universe by passing the row and column as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        // Set each cell to be alive
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true); 
        }
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state. 
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
    }

    /// Set the height of the universe.
    ///     
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
    }
}

impl fmt::Display for Universe {
    // Display the universe as a grid of cells

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                // Write the binary representation of the cell state
                // write!(f, "{}", cell)?;
                // Write the cell state as an emoji
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            // Add a new line at the end of each row
            write!(f, "\n")?;
        }
        // Return OK
        Ok(())
    }
}