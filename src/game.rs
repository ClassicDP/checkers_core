use std::rc::Rc;
use js_sys::Boolean;
use wasm_bindgen::prelude::*;
use crate::color::Color;
use crate::log;
use crate::moves::BoardPos;
use crate::moves_list::{MoveList};
use crate::piece::Piece;
use crate::position::{Position, PositionHistoryItem};
use crate::position_environment::PositionEnvironment;


#[wasm_bindgen]
pub struct Game {
    position_history: std::vec::Vec<PositionHistoryItem>,
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
        match serde_wasm_bindgen::to_value(&self.current_position) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED,
        }
    }

    #[wasm_bindgen]
    pub fn to_board(&self, pack_index: BoardPos) -> BoardPos {
        self.position_environment.pack_to_board[pack_index]
    }

    #[wasm_bindgen]
    pub fn to_pack(&self, board_index: BoardPos) -> BoardPos {
        self.position_environment.board_to_pack[board_index]
    }

    #[wasm_bindgen]
    pub fn get_move_list_for_front(&mut self, color: Color) -> JsValue {
        let move_list = self.get_move_list(color, true);
        match serde_wasm_bindgen::to_value(&move_list) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED,
        }
    }

    fn get_move_list(&mut self, color: Color, for_front: bool) -> MoveList {
        let ps = &self.current_position;
        let pieces_pos: std::vec::Vec<_> = ps.cells.iter()
            .filter(|piece| if let Some(piece) = piece { piece.color == color } else { false })
            .map(|piece| if let Some(piece) = piece { piece.pos } else { panic!("Position problem in get_move_list"); })
            .collect();
        let mut move_list = MoveList::new();
        for pos in &pieces_pos {
            self.current_position.get_strike_list(*pos, &mut move_list, &vec![], for_front);
        }
        if move_list.list.is_empty() {
            for pos in &pieces_pos {
                self.current_position.get_quiet_move_list(*pos, &mut move_list);
            }
        }
        move_list
    }


    #[wasm_bindgen]
    pub fn make_move_for_front(&mut self, pos_chain: &JsValue) -> Result<js_sys::Boolean, JsValue> {
        let mut pos_list: Vec<BoardPos> = Vec::new();
        let iterator = js_sys::try_iter(pos_chain)?.ok_or_else(|| {
            "need to pass iterable JS values!"
        })?;
        for x in iterator {
            // If the iterator's `next` method throws an error, propagate it
            // up to the caller.
            let x = x?;

            // If `x` is a number, add it to our array of numbers!
            if x.as_f64().is_some() {
                pos_list.push(x.as_f64().unwrap() as BoardPos);
            }
        }
        if !pos_list.is_empty() {
            if let Some(piece) = &self.current_position.cells[pos_list[0] as usize] {
                let move_list = self.get_move_list(piece.color, true);
                for mut move_item in move_list.list {
                    let mut i = 1;
                    let mut ok = true;
                    for mov in &move_item {
                        if pos_list.len() <= i {
                            ok = false;
                            break;
                        }
                        if pos_list[i] != mov.to() || pos_list[i - 1] != mov.from() {
                            ok = false;
                            break;
                        }
                        i += 1;
                    }
                    if ok && pos_list.len() == i {
                        self.current_position.make_move(&mut move_item);
                        self.position_history.push(PositionHistoryItem { move_item, position: self.current_position.clone() });
                        return Ok(Boolean::from(JsValue::TRUE));
                    }
                }
            }
        }
        Ok(Boolean::from(JsValue::FALSE))
    }
}