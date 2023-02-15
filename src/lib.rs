#![feature(extern_types)]
extern crate core;
use crate::moves::BoardPos;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use schemars::JsonSchema;
use ts_rs::TS;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use tsify::{declare, Tsify};
use wasm_bindgen::prelude::*;

mod moves;
mod moves_list;
mod position_environment;
mod mutable_iterator;
mod position;
mod vector;
mod piece;
mod color;
mod game;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}







