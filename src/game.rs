use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use crate::{Color, Figure};
use crate::position::Position;
use crate::vector::Vector;

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    size: i8,
    position_history: Vec<Position>,
    vectors_map: Vec<Vec<Vector>>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(size: i8) -> Self {
        if size % 2 != 0 { panic!("Size must be even") }
        let size2: i16 = (size * size) as i16;
        let mut is_black_cell = |i: i16| -> bool {
            (i / size as i16 + i % 2) % 2 == 0
        };
        let mut is_on_board = |i: i16| -> bool {
            i >= 0 && i < size2 && is_black_cell(i)
        };
        let d4 = vec![size + 1, size - 1, -(size + 1), -(size - 1)];
        let mut vectors_map: Vec<Vec<Vector>> = Vec::with_capacity((size2 / 2) as usize);
        let mut board_to_pack: Vec<i16> = Vec::with_capacity(size2 as usize);
        let mut pack_to_board: Vec<i16> = Vec::with_capacity((size2 / 2) as usize);

        // packing board is array with only black cells
        let mut j: i16 = 0;
        for i in 0..size2 {
            if is_black_cell(i) {
                board_to_pack[i as usize] = j;
                pack_to_board[j as usize] = i;
                j += 1;
            }
        }

        for i in 0..size2 {
            if is_black_cell(i) {
                let mut p = i;
                let mut direction_index: i8 = 0;
                let mut d4_v_list: Vec<Vector> = Vec::new();
                for d in d4.iter() {
                    let mut v = Vector { points: vec![board_to_pack[p as usize]], direction_index };
                    loop {
                        p = p + *d as i16;
                        if !is_on_board(p) { break; }
                        v.points.push(board_to_pack[p as usize]);
                    }
                    if v.points.len() > 1 { d4_v_list.push(v); }
                }
                vectors_map.push(d4_v_list);
            }
        }
        Game {
            position_history: Vec::new(),
            vectors_map,
            size,
        }
    }

    pub fn start_position(&self) -> Position {
        let mut pos = Position::new(self.size);
        pos.set_fig(0, Figure::new(0, Color::Black, true));
        pos
    }

    #[wasm_bindgen(getter)]
    pub fn last_position(self) -> JsValue {
        match &self.position_history.last() {
            Some(pos) => pos.to_js(),
            None => JsValue::UNDEFINED
        }
    }
}
