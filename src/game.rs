use crate::position::Position;
use crate::vector::Vector;
use crate::Moves::BoardPos;
use crate::{Color, Piece};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::rc::Rc;
use ts_rs::TS;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use crate::HashRcWrap::HashRcWrap;

#[wasm_bindgen]
#[derive(Clone, Deserialize, Serialize, Debug, TS)]
pub struct Game {
    pub size: i8,
    position_history: Vec<Position>,
    vectors_map: Vec<HashRcWrap<Vec<HashRcWrap<Vector<BoardPos>>>>>,
    board_to_pack: Vec<BoardPos>,
    pack_to_board: Vec<BoardPos>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(size: i8) -> Self {
        if size % 2 != 0 {
            panic!("Size must be even")
        }
        let size2 = (size * size) as BoardPos;
        let is_black_cell = |i: BoardPos| -> bool { (i / size as BoardPos + i % 2) % 2 == 0 };
        let is_on_board = |i: BoardPos| -> bool { i >= 0 && i < size2 && is_black_cell(i) };
        let d4 = vec![size + 1, size - 1, -(size + 1), -(size - 1)];
        let mut vectors_map = Vec::with_capacity((size2 / 2) as usize);
        let mut board_to_pack: Vec<BoardPos> = Vec::with_capacity(size2 as usize);
        board_to_pack.resize(size2 as usize, 0);
        let mut pack_to_board: Vec<BoardPos> = Vec::with_capacity((size2 / 2) as usize);
        pack_to_board.resize((size2 / 2) as usize, 0);
        // packing board is array with only black cells
        let mut j: BoardPos = 0;
        for i in 0..size2 as BoardPos {
            if is_black_cell(i) {
                board_to_pack[i] = j;
                pack_to_board[j] = i;
                j += 1;
            }
        }
        // vectors_map for packing board
        for i in 0..size2 {
            if is_black_cell(i) {
                let mut direction_index: i8 = 0;
                let mut d4_v_list = Vec::new();
                for d in d4.iter() {
                    let mut p = i;
                    let mut v: Vector<BoardPos> =
                        Vector::new(direction_index, vec![board_to_pack[p as usize]]);
                    loop {
                        p = ((p as i64) + (*d as i64)) as BoardPos;
                        if !is_on_board(p) {
                            break;
                        }
                        Rc::get_mut(&mut v.points)
                            .unwrap()
                            .push(board_to_pack[p as usize]);
                    }

                    if v.points.len() > 1 {
                        d4_v_list.push(HashRcWrap::new(v));
                    }
                    direction_index += 1;
                }
                vectors_map.push(HashRcWrap::new(d4_v_list));
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

    pub fn to_board(&self, pack_index: BoardPos) -> BoardPos {
        self.pack_to_board[pack_index]
    }

    pub fn to_pack(&self, board_index: BoardPos) -> BoardPos {
        self.board_to_pack[board_index]
    }

    pub fn js(&self) -> JsValue {
        let s = serde_json::to_value(self)
            .expect("Game serialize error")
            .to_string();
        JsValue::from_str(&s)
    }

    pub fn is_king_row(&self, piece: &Piece) -> bool {
        let size = (self.size / 2) as usize;
        if piece.color == Color::White {
            piece.pos > size * (size - 1)
        } else {
            piece.pos < size
        }
    }
}

impl Game {
    pub fn get_vectors(&self, pos: usize) -> Vec<HashRcWrap<Vector<BoardPos>>> {
        self.vectors_map[pos].get_unwrap().clone()
    }
}
#[cfg(test)]
mod tests {
    use crate::game::Game;
    use crate::position::Position;
    use crate::Moves::PieceMove;
    use crate::MovesList::MoveList;
    use crate::{Color, Piece};
    use std::cell::RefCell;
    use crate::HashRcWrap::HashRcWrap;

    #[test]
    fn game() {
        let game = Game::new(8);
        assert_eq!(game.board_to_pack.len(), game.pack_to_board.len() * 2);
        let mut pos = Position::new(HashRcWrap::new(game));
        pos.inset_piece(Piece::new(22, Color::White, false));
        pos.inset_piece(Piece::new(4, Color::Black, true));
        pos.inset_piece(Piece::new(21, Color::Black, true));
        pos.inset_piece(Piece::new(20, Color::Black, true));
        pos.inset_piece(Piece::new(12, Color::Black, true));
        pos.inset_piece(Piece::new(13, Color::Black, true));
        pos.inset_piece(Piece::new(26, Color::Black, true));
        if let Some(piece) = pos.cells[0].clone() {
            if let Some(set) = pos.pieces.get_mut(&piece.get_unwrap().color) {
                print!(" -piece {}  ", set.contains(&piece))
            }
        }
        let mut list = MoveList::new();
        pos.get_strike_list(22, &vec![3], &mut list);
        print!("\n\n{:?}", list);
    }
}
