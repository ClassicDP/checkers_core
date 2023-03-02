use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::wasm_bindgen;
use ts_rs::TS;
use crate::moves::BoardPos;
use crate::color::Color;
use crate::log;


fn to_js<T: Serialize>(val: T) -> JsValue {
    match serde_wasm_bindgen::to_value(&val) {
        Ok(js) => js,
        Err(_err) => JsValue::UNDEFINED,
    }
}

fn from_js<T: DeserializeOwned>(js: JsValue) -> Option<T> {
    let val = serde_wasm_bindgen::from_value(js);
    match val {
        Ok(val) => Some(val),
        Err(err) => {
            log(&format!("{}", err));
            None
        }
    }
}

#[wasm_bindgen]
impl Piece {
    #[wasm_bindgen]
    pub fn new(pos: BoardPos, color: Color, is_king: bool) -> Piece {
        Piece {
            pos,
            color,
            is_king,
            stricken: false,
        }
    }

    pub fn new_fom_js(js: JsValue) -> Option<Piece> {
        log(&format!("{:?}",js));
        match from_js(js) {
            Some(fi) => fi,
            None => None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn it(self) -> JsValue {
        to_js(self)
    }
    #[wasm_bindgen(setter)]
    pub fn set_it(&mut self, js: JsValue) {
        let model: Option<Piece> = from_js(js);
        match model {
            Some(val) => *self = val,
            None => {}
        }
    }
}



#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[wasm_bindgen]
#[derive(TS)]
#[ts(export)]
pub struct Piece {
    pub pos: BoardPos, // in pack_board
    pub color: Color,
    pub is_king: bool,
    pub stricken: bool,
}
