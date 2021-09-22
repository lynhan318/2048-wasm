#![recursion_limit = "256"]
mod components;

use components::page;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::utils::document;

#[wasm_bindgen(start)]
pub fn run_app() {
    let root = document()
        .query_selector("#root")
        .expect("can't get #root node for rendering")
        .expect("can't unwrap #root node");

    App::<page::RootPage>::new().mount(root);
}
