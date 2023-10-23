//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

extern crate wasm_game_of_life;
use wasm_game_of_life::Universe;

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new(6, 6);
    universe.clear();
    universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new(6, 6);
    universe.clear();
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);

    universe.add_changes(&[
        (1, 2, false),
        (2, 1, true),
        (3, 1, false),
        (4, 2, true),
    ]);

    universe
}

#[wasm_bindgen_test]
pub fn test_clear() {
    let mut input_universe = input_spaceship();
    input_universe.clear();

    assert_eq!(&input_universe.get_changes_len(), &108);
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_spaceship();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_spaceship();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

#[wasm_bindgen_test]
pub fn test_get_changes() {
    let mut input_universe = input_spaceship();
    let expected_universe = expected_spaceship();

    input_universe.tick();
    let changes = input_universe.get_changes();
    let expected_changes = expected_universe.get_changes();

    assert_eq!(changes.len(), expected_changes.len());
    for i in 0..changes.len() {
        // assertions
        assert_eq!(changes[i].row(), expected_changes[i].row());
        assert_eq!(changes[i].col(), expected_changes[i].col());
        //assert_eq!(changes[i].state(), expected_changes[i].state());
    }
}