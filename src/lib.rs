use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use ts_rs::TS;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

mod position;
mod game;
mod vector;
mod mutable_iterator;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn to_js<T: Serialize>(val: T) -> JsValue {
    match serde_wasm_bindgen::to_value(&val) {
        Ok(js) => js,
        Err(_err) => JsValue::UNDEFINED,
    }
}

fn from_js<T: DeserializeOwned>(js: JsValue)->Option<T> {
    let val = serde_wasm_bindgen::from_value(js);
    match val {
        Ok(val) => {  Some(val) }
        Err(err) => { log(&format!("{}", err)); None }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialOrd, Serialize, Deserialize, Debug, Default)]
#[derive(TS)]
#[ts(export)]
#[derive (PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    White,
    #[default]
    None,
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
#[derive(TS)]
#[ts(export)]
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Figure {
    pos: i16, // in pack_board
    color: Color,
    is_king: bool,
    stricken: bool,
}

#[wasm_bindgen]
impl Figure {
    pub fn new(pos: i16, color: Color, is_king: bool) -> Figure {
        Figure {
            pos,
            color,
            is_king,
            stricken: false,
        }
    }

    pub fn new_fom_js(js: JsValue) -> Figure {
        match from_js(js) {
            Some(fi) => fi,
            None => {let fi: Figure = Default::default(); fi}
        }

    }



    #[wasm_bindgen(getter)]
    pub fn it(self) -> JsValue {
        to_js(self)
    }
    #[wasm_bindgen(setter)]
    pub fn set_it(&mut self, js: JsValue) {
        let model: Option<Figure> = from_js(js);
        match model {
            Some(val)=> *self = val,
            None => {}
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[derive(TS)]
enum Cell {
    None,
    CellFigure(RefCell<Figure>),
}

trait MutFigure {
    fn set_pos(&self, new_pos: i16);
}

impl MutFigure for Cell {
    fn set_pos(&self, new_pos: i16) {
        match self {
            Cell::CellFigure(fig) => { fig.borrow_mut().pos = new_pos; }
            _ => {},
        }
    }
}
