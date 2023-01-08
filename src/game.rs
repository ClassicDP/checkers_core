use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use ts_rs::TS;
use crate::position::Position;
use crate::vector::Vector;

#[wasm_bindgen]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[derive(TS)]
pub struct Game {
    pub size: i8,
    position_history: Vec<Position>,
    vectors_map: Vec<Vec<Vector>>,
    board_to_pack: Vec<i16>,
    pack_to_board: Vec<i16>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(size: i8) -> Self {
        if size % 2 != 0 { panic!("Size must be even") }
        let size2: i16 = (size * size) as i16;
        let is_black_cell = |i: i16| -> bool {
            (i / size as i16 + i % 2) % 2 == 0
        };
        let is_on_board = |i: i16| -> bool {
            i >= 0 && i < size2 && is_black_cell(i)
        };
        let d4 = vec![size + 1, size - 1, -(size + 1), -(size - 1)];
        let mut vectors_map: Vec<Vec<Vector>> = Vec::with_capacity((size2 / 2) as usize);
        let mut board_to_pack: Vec<i16> = Vec::with_capacity(size2 as usize);
        board_to_pack.resize(size2 as usize, 0);
        let mut pack_to_board: Vec<i16> = Vec::with_capacity((size2 / 2) as usize);
        pack_to_board.resize((size2 / 2) as usize, 0);
        // packing board is array with only black cells
        let mut j: i16 = 0;
        for i in 0..size2 {
            if is_black_cell(i) {
                board_to_pack[i as usize] = j;
                pack_to_board[j as usize] = i;
                j += 1;
            }
        }
        // vectors_map for packing board
        for i in 0..size2 {
            if is_black_cell(i) {
                let mut direction_index: i8 = 0;
                let mut d4_v_list: Vec<Vector> = Vec::new();
                for d in d4.iter() {
                    let mut p = i;
                    let mut v = Vector { points: vec![board_to_pack[p as usize]], direction_index };
                    loop {
                        p = p + *d as i16;
                        if !is_on_board(p) { break; }
                        v.points.push(board_to_pack[p as usize]);
                    }

                    if v.points.len() > 1 { d4_v_list.push(v); }
                    direction_index += 1;
                }
                vectors_map.push(d4_v_list);
            }
        }
        Game {
            position_history: Vec::new(),
            pack_to_board,
            board_to_pack,
            vectors_map,
            size,
        }
    }

    pub fn to_board(&self, pack_index: i16) -> i16 {
        self.pack_to_board[pack_index as usize]
    }

    pub fn to_pack(&self, board_index: i16) -> i16 {
        self.board_to_pack[board_index as usize]
    }

    pub fn js(&self) -> JsValue {
        let s = serde_json::to_value(self).expect("Game serialize error")
            .to_string();
        JsValue::from_str(&s)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Game;
    #[test]
    fn game() {
        let game = Game::new(8);
        assert_eq!(game.board_to_pack.len(), game.pack_to_board.len() * 2);
    }
}
