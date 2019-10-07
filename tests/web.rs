//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_tick() {
    let width = 6;
    let height = 6;
    // set spaceship
    let spaceship_cells: [u32; 5] = [9, 13, 15, 20, 21];
    let mut universe = Universe::new_with_state(width, height, &spaceship_cells);
    universe.tick();
    let expected_spaceship_conf: [u32; 5] = [8, 15, 16, 20, 21];
    assert_eq!(&universe.get_live_cells(), &expected_spaceship_conf);
}
