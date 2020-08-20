#![recursion_limit="512"]

mod home;
mod elements;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use home::Home;


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Home>::new().mount_to_body();
}