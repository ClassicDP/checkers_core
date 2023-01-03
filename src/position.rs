use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use crate::{Cell, Figure};

#[derive(Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Position {
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Position {
    pub fn new(size: i8) -> Position {
        if size % 2 != 0 { panic!("Size must be even"); }
        let mut pos = Position { cells: Vec::new() };
        pos.cells = Vec::new();
        pos.cells.resize((size * size / 2) as usize, Cell::None);
        pos
    }
    #[wasm_bindgen]
    pub fn set_fig(&mut self, _i: usize, ch: Figure) {
        self.cells[_i] = Cell::Figure(ch);
    }

    #[wasm_bindgen]
    pub fn to_js(&self) -> JsValue {
        match serde_wasm_bindgen::to_value(self) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED,
        }
    }
}
