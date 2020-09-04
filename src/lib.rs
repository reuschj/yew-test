#![recursion_limit="512"]

// Top-level modules
mod home;
mod containers;
mod elements;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use home::Home;

// Main
#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Home>::new().mount_to_body();
}