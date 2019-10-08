#![feature(test)]

extern crate test;
use wasm_game_of_life;

#[bench]
fn universe_tick(b: &mut test::Bencher) {
    let spaceship_cells: [u32; 5] = [9, 13, 15, 20, 21];
    let mut universe = wasm_game_of_life::Universe::new_with_state(64, 64, &spaceship_cells);

    b.iter(|| {
        universe.tick();
    });
}
