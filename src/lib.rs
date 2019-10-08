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

macro_rules! logtime {
    ( $t:tt ) => {
        web_sys::console::time_with_label($t.into());
    };
}

macro_rules! logtime_end {
    ( $t:tt ) => {
        web_sys::console::time_end_with_label($t.into());
    };
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
    alive: Vec<u32>,
}

// Public methods, exported to JavaScript
#[wasm_bindgen]
impl Universe {
    pub fn new_with_state(width: u32, height: u32, alive_cells: &[u32]) -> Universe {
        logtime!("universe::new_with_state");
        utils::set_panic_hook();
        log!(
            "universe::new_with_state called width {} height {}",
            width,
            height
        );
        log!("alive_cells passed {:?}", alive_cells);
        let size = (width * height) as usize;
        let mut alive = Vec::with_capacity(size);
        let mut is_alive: HashMap<u32, bool> = HashMap::new();

        for v in alive_cells.iter() {
            is_alive.insert(*v, true);
            alive.push(*v);
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

        logtime_end!("universe::new_with_state");
        Universe {
            width,
            height,
            cells,
            alive,
        }
    }
    pub fn new(width: u32, height: u32) -> Universe {
        logtime!("universe::new()");
        utils::set_panic_hook();
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        let mut alive = Vec::with_capacity(size);
        for i in 0..size {
            let alive = if Math::random() > 0.5 {
                alive.push(i as u32);
                true
            } else {
                false
            };
            cells.set(i, alive);
        }

        logtime_end!("universe::new()");
        Universe {
            width,
            height,
            cells,
            alive,
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

    pub fn alive_cells(&self) -> *const u32 {
        self.alive.as_slice().as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn tick(&mut self) {
        self.alive.clear();
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                let alive = match (cell, live_neighbors) {
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
                };
                next.set(idx, alive);
                if alive {
                    self.alive.push(idx as u32);
                }
            }
        }
        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        // neighbors rows/cols
        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if col == 0 { self.width - 1 } else { col - 1 };
        let east = if col == self.width - 1 { 0 } else { col + 1 };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, col);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, col);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    pub fn get_live_cells_count(&self) -> u32 {
        self.alive.len() as u32
    }
}

impl Universe {
    // returns live cells indexes
    pub fn get_live_cells(&self) -> Vec<u32> {
        self.alive.to_vec()
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

#[cfg(test)]
mod tests {
    use super::Universe;
    #[test]
    pub fn test_tick() {
        let width = 6;
        let height = 6;
        // set spaceship
        let spaceship_cells: [u32; 5] = [9, 13, 15, 20, 21];
        println!("input {:?}", spaceship_cells);
        let mut universe = Universe::new_with_state(width, height, &spaceship_cells);
        universe.tick();
        let expected_spaceship_conf: [u32; 5] = [8, 15, 16, 20, 21];
        assert_eq!(&universe.get_live_cells(), &expected_spaceship_conf);
    }
}
