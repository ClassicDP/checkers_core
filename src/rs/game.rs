use std::rc::Rc;
use js_sys::Boolean;
use wasm_bindgen::prelude::*;
use crate::color::Color;
use crate::moves::BoardPos;
use crate::moves_list::{MoveList};
use crate::piece::Piece;
use crate::position::{Position, PositionHistoryItem, PosState};
use crate::position_environment::PositionEnvironment;
use ts_rs::*;
use serde::{Serialize};


#[wasm_bindgen]
#[derive(TS)]
#[ts(export)]
pub enum DrawType {
    draw1,
    draw2,
    draw3,
    draw4,
    draw5,
}

#[derive(Default)]
#[wasm_bindgen]
#[derive(TS)]
#[ts(export)]
#[derive(Serialize)]
pub struct GameState {
    kings_start_at: Option<usize>,
    kings_only_move_start_at: Option<usize>,
    triangle_start_at: Option<usize>,
    power_equal_start_at: Option<usize>,
    main_road_start_at: Option<usize>,
}

#[wasm_bindgen]
pub struct Game {
    position_history: std::vec::Vec<PositionHistoryItem>,
    state: GameState,
    position_environment: Rc<PositionEnvironment>,
    #[wasm_bindgen(skip)]
    pub current_position: Position,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(size: i8) -> Self {
        let environment = Rc::new(PositionEnvironment::new(size));
        Game {
            position_history: vec![],
            state: Default::default(),
            position_environment: environment.clone(),
            current_position: Position::new(environment.clone()),
        }
    }
    #[wasm_bindgen]
    pub fn insert_piece(&mut self, piece: Piece) {
        self.current_position.insert_piece(piece);
    }

    #[wasm_bindgen]
    pub fn remove_piece(&mut self, pos: BoardPos) -> bool {
        self.current_position.remove_piece(pos)
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> JsValue {
        match serde_wasm_bindgen::to_value(&self.current_position) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn state(&self) -> JsValue {
        match serde_wasm_bindgen::to_value(&self.state) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED
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
        self.current_position.get_move_list(color, for_front)
    }


    pub fn draw_check(&mut self) -> Option<DrawType> {
        let i = (self.position_history.len() - 1);
        let ref mut pos_it = self.position_history[i];
        if pos_it.position.state.get_count(Color::White).king > 0 &&
            pos_it.position.state.get_count(Color::Black).king > 0 {
            // first position where both set kings
            if self.state.kings_start_at.is_none() {
                self.state.kings_start_at = Some(i);
            }
            // 1) если в течение 15 ходов игроки делали ходы только дамками, не передвигая
            // простых шашек и не производя взятия.
            if pos_it.position.get_piece_of_move_item(&pos_it.move_item).is_king {
                if self.state.kings_only_move_start_at.is_none() {
                    self.state.kings_only_move_start_at = Some(i);
                } else {
                    if i - self.state.kings_only_move_start_at.unwrap() > 15 {
                        return Some(DrawType::draw1);
                    }
                }
            } else { self.state.kings_only_move_start_at = None; }


            // 2) если три раза повторяется одна и та же позиция
            let mut repeats = 0;
            let mut j = i;
            let pos = &self.position_history[i].position;
            while self.position_history[j].position.state == pos.state {
                if *pos == self.position_history[j].position {
                    repeats += 1;
                    if repeats == 3 { return Some(DrawType::draw2); }
                }
                if j == 0 { break; }
                j -= 1;
            }

            // 3) если участник, имеющий три дамки (и более) против одной дамки противника,
            // за 15 ходов не возьмёт дамку противника
            let state = &mut self.position_history[i].position.state;
            if (state.get_count(Color::White).king == 1 && state.get_count(Color::Black).king >= 3) ||
                (state.get_count(Color::Black).king == 1 && state.get_count(Color::White).king >= 3) {
                if self.state.triangle_start_at.is_none() { self.state.triangle_start_at = Some(i); } else {
                    if i - self.state.triangle_start_at.unwrap() >= 15 { return Some(DrawType::draw3); }
                }
            } else { self.state.triangle_start_at = None; }

            // 4) если в позиции, в которой оба соперника имеют дамки, не изменилось соотношение сил
            // (то есть не было взятия, и ни одна простая шашка не стала дамкой) на протяжении:
            // в 2- и 3-фигурных окончаниях — 5 ходов,
            // в 4- и 5-фигурных окончаниях — 30 ходов,
            // в 6- и 7-фигурных окончаниях — 60 ходов;
            if i > 0 {
                let ref state = self.position_history[i].position.state;
                let ref prev_state = self.position_history[i - 1].position.state;
                if *state == *prev_state {
                    if self.state.power_equal_start_at.is_none() { self.state.power_equal_start_at = Some(i); }
                    let total = state.get_total();
                    let n = i - self.state.power_equal_start_at.unwrap();
                    if total < 4 && n > 5 { return Some(DrawType::draw4); }
                    if total < 6 && n > 30 { return Some(DrawType::draw4); }
                    if total < 8 && n > 60 { return Some(DrawType::draw4); }
                } else { self.state.power_equal_start_at = None; }
            }

            // если участник, имея в окончании партии три дамки, две дамки и простую, дамку и две простые,
            // ""три простые против одинокой дамки"", находящейся на большой дороге,
            // своим 5-м ходом не сможет добиться выигранной позиции;
            let ref mut state = self.position_history[i].position.state;
            if (state.get_count(Color::Black).king == 1 ||
                state.get_count(Color::White).king == 1) &&
                state.get_total() == 4 {
                let points = self.position_environment.get_vectors(0)[0].clone();
                let gen_road_pieces: Vec<_> =
                    points.into_iter().filter(|pos| self.current_position.cells[**pos].is_some()).collect();
                if gen_road_pieces.len() == 1 &&
                    self.current_position.cells[*gen_road_pieces[0]].is_some() {
                    if self.state.main_road_start_at.is_none() {
                        self.state.main_road_start_at = Some(i);
                    }
                    if i - self.state.main_road_start_at.unwrap() > 5 {
                        return Some(DrawType::draw5);
                    }
                } else { self.state.main_road_start_at = None; }
            } else { self.state.main_road_start_at = None; }
        } else { self.state.kings_start_at = None; }
        None
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

#[cfg(test)]
mod tests {
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::color::Color;
    use crate::game::Game;
    use crate::piece::Piece;
    use crate::position_environment::PositionEnvironment;

    #[test]
    fn game_test() {
        let game = Game::new(8);
        assert!(game.state.kings_start_at.is_none());
    }

    #[test]
    fn game_quite_move() {
        let mut game = Game::new(8);
        game.insert_piece(Piece::new(13, Color::White, true));
        vec![2, 27, 24].iter().for_each(|pos|game.insert_piece(Piece::new(*pos, Color::White, false)));
        let list = game.get_move_list(Color::White, true);
        print!("\ngame_quite_move {:?} \n", {
            let z: Vec<_> = list.list.iter().map(|x|x.mov.clone().unwrap()).collect();
            z
        });
        assert_eq!(list.list.len(), 15);
    }
    #[test]
    pub fn game_strike_list() {
        let mut game = Game::new(8);
        game.insert_piece(Piece::new(game.to_pack(47), Color::White, false));
        game.insert_piece(Piece::new(game.to_pack(63), Color::White, false));
        game.insert_piece(Piece::new(game.to_pack(15), Color::White, true));
        vec![54, 43, 20].iter()
            .for_each(|pos|
                game.insert_piece(Piece::new(game.to_pack(*pos), Color::Black, false)));
        for _t in 0..1000000 {
            let list = game.get_move_list(Color::White, true);
        }
        let list = game.get_move_list(Color::White, true);
        print!("\ngame_quite_move {:?} \n", {
            let z: Vec<_> = list.list.iter().map(|x|x.strike.clone().unwrap()).collect();
            z
        });
        assert_eq!(list.list.len(), 5);
    }

    #[test]
    fn performance () {
        PositionEnvironment::game();
    }
}