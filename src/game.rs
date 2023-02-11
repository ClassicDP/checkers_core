use std::rc::Rc;
use wasm_bindgen::prelude::*;
use crate::piece::Piece;
use crate::position::{Cell, Position};
use crate::position_environment::PositionEnvironment;

#[wasm_bindgen]
pub struct Game {
    position_history: Vec<Position>,
    position_environment: Rc<PositionEnvironment>,
    current_position: Position,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(size: i8) -> Self {
        let environment = Rc::new(PositionEnvironment::new(size));
        Game {
            position_history: vec![],
            position_environment: environment.clone(),
            current_position: Position::new(environment.clone()),
        }
    }
    #[wasm_bindgen]
    pub fn insert_piece(&mut self, piece: Piece) {
        self.current_position.inset_piece(piece);
    }

    #[wasm_bindgen(getter)]
    pub fn position(&mut self) -> JsValue {
        match serde_wasm_bindgen::to_value(&self.current_position.cells) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED,
        }
    }
}