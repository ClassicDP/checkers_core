use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (js_namespace = Math)]
    fn random() -> f64;
    #[wasm_bindgen (js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Color {
    Black=0,
    White=8,
}

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
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Figure {
    pos: i32,
    color: Color,
    is_king: bool,
    stricken: bool,
}
#[wasm_bindgen]
impl Figure {
    #[wasm_bindgen(constructor)]
    pub fn new(pos: i32, color: Color, is_king: bool) -> Figure {
        Figure {
            pos,
            color,
            is_king,
            stricken: false,
        }
    }
    #[wasm_bindgen(getter)]
    pub fn it(self) -> JsValue {
        match serde_wasm_bindgen::to_value(&self) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED,
        }
    }
    #[wasm_bindgen(setter)]
    pub fn set_it(&mut self, js: JsValue) {
        let val = serde_wasm_bindgen::from_value(js);
        match val {
            Ok(val) => *self = val,
            Err(_err) => {}
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
enum Cell {
    None,
    Figure(Figure),
}


#[derive(Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Position {
    cells: Vec<Cell>,
}
#[wasm_bindgen]
impl Position {
    pub fn new(size: u32) -> Position {
        let mut pos = Position {cells:Vec::new()};
        pos.cells = Vec::new();
        pos.cells.resize(size as usize, Cell::None);
        pos
    }
    #[wasm_bindgen]
    pub fn set_fig(&mut self, _i: usize, ch: Figure) {
        self.cells[_i] = Cell::Figure(ch);
    }
    #[wasm_bindgen(getter)]
    pub fn it(self) -> JsValue {
        match serde_wasm_bindgen::to_value(&self) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED,
        }
    }
    #[wasm_bindgen]
    pub fn to_js(&self) -> JsValue {
        match serde_wasm_bindgen::to_value(self) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED,
        }
    }

}

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    size: u32,
    position_history: Vec<Position>,
}
#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(size: u32) -> Self {
        Game {
            position_history: (Vec::new()),
            size,
        }
    }

    pub fn start_position (&self) -> Position {
        let mut pos = Position::new(self.size);
        pos.set_fig(0, Figure::new(0, Color::Black, true));
        pos
    }

    #[wasm_bindgen(getter)]
    pub fn last_position(self) -> JsValue {
        match &self.position_history.last() {
            Some(pos)=>pos.to_js(),
            None => JsValue::UNDEFINED
        }
    }
}
