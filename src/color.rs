use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Deserialize, Serialize};
use ts_rs::*;


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
#[derive(PartialEq, Eq)]
#[derive(TS)]
#[ts(export)]
#[ts(rename = "ColorType")]
pub enum Color {
    Black = 0,
    White = 1
}

impl Color {
    pub fn inverse(&self) -> Color {
        if *self == Color::White {Color::Black} else { Color::White }
    } 
}
