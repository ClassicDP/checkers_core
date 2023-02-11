use std::borrow::BorrowMut;
use std::cell::RefCell;
use crate::position::Position;
use crate::moves::BoardPos;
use crate::color::Color;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use crate::moves_list::MoveList;
use crate::piece::Piece;
use crate::vector::Vector;

#[wasm_bindgen]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PositionEnvironment {
    pub size: i8,
    vectors_map: Vec<Vec<Rc<Vector<BoardPos>>>>,
    board_to_pack: Vec<BoardPos>,
    pack_to_board: Vec<BoardPos>,
}

#[wasm_bindgen]
impl PositionEnvironment {
    #[wasm_bindgen(constructor)]
    pub fn new(size: i8) -> Self {
        if size % 2 != 0 {
            panic!("Size must be even")
        }
        let size2 = (size * size) as BoardPos;
        let is_black_cell = |i: BoardPos| -> bool { (i / size as BoardPos + i % 2) % 2 == 0 };
        let is_on_board = |i: BoardPos| -> bool { i < size2 && is_black_cell(i) };
        let d4 = vec![size + 1, size - 1, -(size + 1), -(size - 1)];
        let mut vectors_map = Vec::new();
        let mut board_to_pack: Vec<BoardPos> = Vec::new();
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

                    let mut points = vec![board_to_pack[p]];
                    loop {
                        p = ((p as i64) + (*d as i64)) as BoardPos;
                        if !is_on_board(p) {
                            break;
                        }
                        points.push(board_to_pack[p as usize]);
                    }
                    let v: Vector<BoardPos> =
                        Vector::new(direction_index, points);

                    if v.points.len() > 1 {
                        d4_v_list.push(Rc::new(v));
                    }
                    direction_index += 1;
                }
                vectors_map.push(d4_v_list);
            }
        }
        PositionEnvironment {
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
    #[wasm_bindgen]
    pub fn test() -> JsValue {
        let game = PositionEnvironment::new(8);
        let mut pos = Position::new(Rc::new(game));
        pos.inset_piece(Piece::new(22, Color::White, false));
        pos.inset_piece(Piece::new(4, Color::Black, true));
        pos.inset_piece(Piece::new(21, Color::Black, true));
        pos.inset_piece(Piece::new(20, Color::Black, true));
        pos.inset_piece(Piece::new(12, Color::Black, true));
        pos.inset_piece(Piece::new(13, Color::Black, true));
        pos.inset_piece(Piece::new(26, Color::Black, true));


        let mut list = MoveList::new();
        // pos.get_strike_list(22, &mut list, &vec![]);
        // print!("\n\n list: {:?}", list);

        // for _i in 0..100000 {
        //     let mut list = MoveList::new();
        //     pos.get_strike_list(22, &mut list, &vec![]);
        //     pos.make_move(&mut list.list[0]);
        //     pos.unmake_move(&mut list.list[0]);
        //     let mut p0 = pos.make_move_and_get_position(&mut list.list[0]);
        //     pos.unmake_move(p0.move_item.borrow_mut());
        //     let mut p1 = pos.make_move_and_get_position(&mut list.list[0]);
        //     if p0 != p1 { break; }
        //     pos.unmake_move(p1.move_item.borrow_mut());
        // }
        // let mut list = MoveList::new();
        // // pos.get_strike_list(22, &mut list, &vec![]);
        // // let mut p0 = pos.make_move_and_get_position(&mut list.list[0]);
        // // pos.unmake_move(p0.move_item.borrow_mut());
        //

        for _i in 0..100000 {
            let mut list = MoveList::new();
            pos.get_strike_list(22, &mut list, &vec![]);
            let mut p0 = pos.make_move_and_get_position(&mut list.list[0]);
            pos.unmake_move(p0.move_item.borrow_mut());
            let p1 = p0.clone();
            if p0!= p1 {break;}
        };

        let mut list = MoveList::new();
        pos.get_strike_list(22, &mut list, &vec![]);
        match serde_wasm_bindgen::to_value(&list) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED,
        }



        // for _i  in 0..100000 {
        //     let mut p1 = p0.clone();
        //     if p0!= p1 {break;}
        // }
    }
}

impl PositionEnvironment {
    pub fn get_vectors(&self, pos: usize) -> &Vec<Rc<Vector<BoardPos>>> {
        &self.vectors_map[pos]
    }
}


#[cfg(test)]
mod tests {
    use crate::position_environment::PositionEnvironment;
    use crate::position::Position;
    use crate::moves_list::MoveList;
    use crate::color::Color;
    use std::rc::Rc;
    use std::time::Instant;
    use crate::piece::Piece;


    #[test]
    fn game() {
        // Game::test();
        let game = PositionEnvironment::new(8);
        assert_eq!(game.board_to_pack.len(), game.pack_to_board.len() * 2);
        let mut pos = Position::new(Rc::new(game));
        pos.inset_piece(Piece::new(22, Color::White, false));
        pos.inset_piece(Piece::new(4, Color::Black, true));
        pos.inset_piece(Piece::new(21, Color::Black, true));
        pos.inset_piece(Piece::new(20, Color::Black, true));
        pos.inset_piece(Piece::new(12, Color::Black, true));
        pos.inset_piece(Piece::new(13, Color::Black, true));
        pos.inset_piece(Piece::new(26, Color::Black, true));

        for _i in 0..100000 {
            let mut list = MoveList::new();
            pos.get_strike_list(22, &mut list, &vec![]);
            let po = pos.make_move_and_get_position(&mut list.list[0]);
            if po != po { break; }
            pos.unmake_move(&mut list.list[0]);
        }
        return;
        let mut list = MoveList::new();



        pos.get_strike_list(22, &mut list, &vec![]);
        print!("\n\n{:?}", list);

        let start = Instant::now();
        for i in 0..100000 {
            pos.get_strike_list(22, &mut list, &vec![]);
            // pos.make_move(&mut list.list[0]);
            // pos.unmake_move(&mut list.list[0]);
        }
        let duration = start.elapsed();
        println!("\n\nTime elapsed is: {:?}\n", duration);

        pos.make_move(&mut list.list[0]);
        print!("\n\n{:?}", pos);
        pos.unmake_move(&mut list.list[0]);
        print!("\n\n{:?}", pos);
        let mut list = MoveList::new();
        pos.get_quiet_move_list(21, &mut list);
        print!("\n\n{:?}", list);
    }
}
