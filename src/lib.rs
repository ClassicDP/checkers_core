use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::hash::Hash;
use std::panic::RefUnwindSafe;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use ts_rs::TS;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use crate::game::HashRcWrap;
use crate::Moves::BoardPos;


mod position;
mod game;
mod vector;
mod mutable_iterator;
mod Moves;


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
#[derive(Copy, Clone, PartialOrd, Serialize, Deserialize, Debug)]
#[derive(TS)]
#[ts(export)]
#[derive (PartialEq, Eq, Hash)]
pub enum Color {
    Black=0,
    White=1,
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
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Piece {
    pos: BoardPos, // in pack_board
    color: Color,
    is_king: bool,
    stricken: bool,
}


#[wasm_bindgen]
impl Piece {
    pub fn new(pos: BoardPos, color: Color, is_king: bool) -> Piece {
        Piece {
            pos,
            color,
            is_king,
            stricken: false,
        }
    }

    pub fn new_fom_js(js: JsValue) -> Option<Piece> {
        match from_js(js) {
            Some(fi) => fi,
            None => {None}
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
            Some(val)=> *self = val,
            None => {}
        }
    }
}


type Cell = Option<HashRcWrap<Piece>>;



