use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use ts_rs::TS;

impl Color {
    pub fn reverse(&mut self) {
        *self = if *self == Color::Black {
            Color::White
        } else {
            Color::Black
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialOrd, Serialize, Deserialize, Debug)]
#[derive(PartialEq, Eq, Hash, JsonSchema, TS)]
#[ts(export)]
pub enum Color {
    Black,
    White
}
