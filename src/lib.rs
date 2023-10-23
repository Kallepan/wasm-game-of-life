mod utils;

use std::fmt;
use js_sys;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

extern crate web_sys;
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}



#[wasm_bindgen] // Expose to JavaScript
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen] // Expose to JavaScript
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state. 
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
    }

    pub fn height(&self) -> u32 {
        self.height 
    }

    /// Set the height of the universe.
    ///     
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        // row * width + col
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        // Check all neighbors
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                // Skip the cell itself
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                // Get the neighbor's position
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;

                // Get the index of the neighbor
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        // Clone the cells
        let mut next = self.cells.clone(); 

        // Loop through all cells
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(idx, match (cell, live_neighbors) {
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
            }
        }

        // Replace the old cells with the new ones
        self.cells = next; }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 128;
        let height = 128;

        // Initialize with all dead cells
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        // Randomly initialize the cells
        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
        }

        log!("Created a universe with {} cells of {} width and {} height.", cells.len(), width, height);
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    /// Responsive functions    
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.toggle(idx);
    }

    pub fn randomize(&mut self) {
        for i in 0..self.cells.len() {
            self.cells.set(i, js_sys::Math::random() < 0.5);
        }
    }

    pub fn clear(&mut self) {
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
    }
}

// Do not expose to JavaScript
impl Universe {
    /// Get the dead and alive cells as a pointer to the cells memory.
    pub fn get_cells(&self) -> &[u32] {
        // Convert to a slice
        &self.cells.as_slice()
    }

    
    /// Set cells to be alive in a universe by passing the row and column as an array.
    /// Do not expose to JavaScript.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        // Set each cell to be alive
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true); 
        }
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