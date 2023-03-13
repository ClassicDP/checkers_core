extern crate core;
use wasm_bindgen::prelude::*;
pub mod moves;
mod moves_list;
mod position_environment;
mod mutable_iterator;
mod position;
mod vector;
pub mod piece;
pub mod color;
pub mod game;
mod mcts;
mod PositionHistory;



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    pub fn random() -> f64;
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}







