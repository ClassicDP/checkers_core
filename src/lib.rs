use js_sys::JsString;
use serde::{Deserialize, Serialize};
use serde::de::{DeserializeOwned, Error};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use ts_rs::TS;
use position::Position;

mod position;
mod game;
mod vector;

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
#[derive(Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug, Default)]
#[derive(TS)]
#[ts(export)]
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
    pos: i32,
    color: Color,
    is_king: bool,
    stricken: bool,
}

trait Getter {

}

#[wasm_bindgen]
impl Figure {
    pub fn new(pos: i32, color: Color, is_king: bool) -> Figure {
        Figure {
            pos,
            color,
            is_king,
            stricken: false,
        }
    }

    pub fn new_fom_js(js: JsValue) -> Figure {
        let mut fi: Figure = Default::default();
        match from_js(js) {
            Some(fi) => fi,
            None => {let mut fi: Figure = Default::default(); fi}
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

#[derive(Clone, Serialize, Deserialize)]
enum Cell {
    None,
    Figure(Figure),
}
