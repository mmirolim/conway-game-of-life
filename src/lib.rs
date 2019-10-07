mod utils;
use fixedbitset::FixedBitSet;
use js_sys::Math;
use std::collections::HashMap;
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys;

// A macro to provide `println!(..)` style syntax for `console.log` logging
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )*).into());
    }
}
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}
// Public methods, exported to JavaScript
#[wasm_bindgen]
impl Universe {
    pub fn new_with_state(width: u32, height: u32, alive_cells: &[u32]) -> Universe {
        utils::set_panic_hook();
        log!("new_with_state called width {} height {}", width, height);
        log!("alive_cells passed {:?}", alive_cells);
        let size = (width * height) as usize;
        let mut is_alive: HashMap<u32, bool> = HashMap::new();

        for v in alive_cells.iter() {
            is_alive.insert(*v, true);
        }
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(
                i,
                match is_alive.get(&(i as u32)) {
                    Some(_) => true,
                    None => false,
                },
            );
        }
        Universe {
            width,
            height,
            cells,
        }
    }
    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, if Math::random() > 0.5 { true } else { false });
        }

        Universe {
            width,
            height,
            cells,
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        // Rule 1: Any cell with fewer than 2 live neighbors dies,
                        // as if caused by underpopulation
                        (true, x) if x < 2 => false,
                        // Rule 2: Any live cell with 2 or 3 live neighbors
                        // lives on to the next generation
                        (true, 2) | (true, 3) => true,
                        // Rule 3: Any live cell with more than three live neighbors
                        // dies, as if by overpopulation
                        (true, x) if x > 3 => false,
                        // Rule 4: Any dead cell with exactly 3 live neighbors
                        // becomes a live cell, as if by reproduction
                        (false, 3) => true,
                        // All other cells remain in the same state
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }
        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl Universe {
    // returns live cells indexes
    pub fn get_live_cells(&self) -> Vec<u32> {
        let mut alive = vec![];
        for i in 0..self.cells.len() {
            if self.cells[i] {
                alive.push(i as u32);
            }
        }
        alive
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.cells.len() {
            let symbol = if !self.cells[i] { '◻' } else { '◼' };
            write!(f, "{}", symbol)?;
            if i as u32 % (self.width - 1) == 0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
