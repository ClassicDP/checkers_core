use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use ts_rs::TS;
use crate::position::Position;
use crate::Moves::BoardPos;
use crate::vector::Vector;


#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(TS)]
pub struct HashRcWrap<T> {
    value: Rc<RefCell<T>>,
}

impl<T> Deref for HashRcWrap<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        self.value.deref()
    }
}

impl<T: Debug> HashRcWrap<T> {
    pub fn new(value: T) -> HashRcWrap<T> {
        HashRcWrap {
            value: Rc::new(RefCell::new(value))
        }
    }
    pub fn get_unwrap_mut(&self) -> RefMut<'_, T> {
        self.value.deref().try_borrow_mut().expect("already borrowed")
    }
    pub fn get_unwrap(&self) -> Ref<'_, T> {
        self.value.deref().borrow()
    }
}

impl<T> PartialEq<Self> for HashRcWrap<T> {
    fn eq(&self, other: &Self) -> bool {
        (*self.value).as_ptr() == (*other.value).as_ptr()
    }
}

impl<T> Eq for HashRcWrap<T> {}

impl<T> Hash for HashRcWrap<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = (*self.value).as_ptr();
        ptr.hash(state)
    }
}


#[wasm_bindgen]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[derive(TS)]
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
        if size % 2 != 0 { panic!("Size must be even") }
        let size2 = (size * size) as BoardPos;
        let is_black_cell = |i: BoardPos| -> bool {
            (i / size as BoardPos + i % 2) % 2 == 0
        };
        let is_on_board = |i: BoardPos| -> bool {
            i >= 0 && i < size2  && is_black_cell(i)
        };
        let d4 = vec![size + 1, size - 1, -(size + 1), -(size - 1)];
        let mut vectors_map = Vec::with_capacity((size2 / 2) as usize);
        let mut board_to_pack: Vec<BoardPos> = Vec::with_capacity(size2 as usize);
        board_to_pack.resize(size2 as usize, 0);
        let mut pack_to_board: Vec<BoardPos> = Vec::with_capacity((size2 / 2) as usize);
        pack_to_board.resize((size2 / 2) as usize, 0);
        // packing board is array with only black cells
        let mut j: BoardPos = 0;
        for i  in 0..size2 as BoardPos {
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
                let mut d4_v_list= Vec::new();
                for d in d4.iter() {
                    let mut p = i;
                    let mut v: Vector<BoardPos> =
                        Vector::new(direction_index, vec![board_to_pack[p as usize]]);
                    loop {
                        p = ((p as i64) + (*d as i64)) as BoardPos;
                        if !is_on_board(p) { break; }
                        Rc::get_mut(&mut v.points).unwrap().push(board_to_pack[p as usize]);
                    }

                    if v.points.len() > 1 { d4_v_list.push(HashRcWrap::new(v)); }
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
        let s = serde_json::to_value(self).expect("Game serialize error")
            .to_string();
        JsValue::from_str(&s)
    }
}

impl Game {
    pub fn get_vectors(&self, pos: usize) -> Vec<HashRcWrap<Vector<BoardPos>>> {
        self.vectors_map[pos].get_unwrap().clone()
    }
}
#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use crate::{Color, Piece};
    use crate::game::{Game, HashRcWrap};
    use crate::position::Position;

    #[test]
    fn game() {
        let game = Game::new(8);
        assert_eq!(game.board_to_pack.len(), game.pack_to_board.len() * 2);
        let mut pos = Position::new(HashRcWrap::new(game));
        pos.inset_piece(Piece::new(0, Color::Black, true));
        let p1 = pos.game.get_unwrap().board_to_pack[9];
        pos.inset_piece(Piece::new(p1, Color::White, true));
        if let Some(piece) = pos.cells[0].clone() {
            if let Some(set) = pos.pieces.get_mut(&piece.get_unwrap().color) {
                print!(" -piece {}  ", set.contains(&piece))
            }
        }
        pos.get_strike_list(0, 3);
        print!("{:?}", pos.cells);
    }
}


