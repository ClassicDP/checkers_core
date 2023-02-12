use std::rc::Rc;
use wasm_bindgen::prelude::*;
use crate::color::Color;
use crate::moves::BoardPos;
use crate::moves_list::MoveList;
use crate::piece::Piece;
use crate::position::{Position};
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

    pub fn get_move_list(&mut self, color: Color) -> JsValue {
        let ps = &self.current_position;
        let pieces: Vec<_> = ps.cells.iter()
            .filter(|piece| if let  Some(piece)  = piece{piece.color==color} else { false })
            .map(|piece| if let Some(piece) = piece {piece.pos} else { panic!("Position problem in get_move_list"); })
            .collect();
        let mut move_list = MoveList::new();
        for pos in &pieces {
            self.current_position.get_strike_list(*pos, &mut move_list, &vec![], true);
        }
        if move_list.list.is_empty() {
            for pos in &pieces {
                self.current_position.get_quiet_move_list(*pos, &mut move_list);
            }
        }
        match serde_wasm_bindgen::to_value(&move_list) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED,
        }
    }
}