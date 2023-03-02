use std::rc::Rc;
use wasm_bindgen::prelude::*;
use crate::color::Color;
use crate::moves::BoardPos;
use crate::moves_list::{MoveItem, MoveList};
use crate::piece::Piece;
use crate::position::{Position, PositionHistoryItem, PosState};
use crate::position_environment::PositionEnvironment;
use ts_rs::*;
use serde::{Serialize};

#[wasm_bindgen]
#[derive(TS)]
#[ts(export)]
#[derive(Serialize)]
pub enum DrawType {
    Draw1,
    Draw2,
    Draw3,
    Draw4,
    Draw5,
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
    position_history: Vec<PositionHistoryItem>,
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
    pub fn get_move_list_for_front(&mut self) -> JsValue {
        let move_list = self.get_move_list(true);
        match serde_wasm_bindgen::to_value(&move_list) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED,
        }
    }

    fn get_move_list(&mut self, for_front: bool) -> MoveList {
        self.current_position.get_move_list(for_front)
    }

    #[wasm_bindgen(getter = moveColor)]
    pub fn get_color(&self) -> JsValue {
        match self.current_position.next_move {
            Some(color) => match serde_wasm_bindgen::to_value(&color) {
                Ok(js) => js,
                Err(_err) => JsValue::UNDEFINED,
            },
            None => JsValue::UNDEFINED
        }
    }

    #[wasm_bindgen(setter = moveColor)]
    pub fn set_color(&mut self, color: Color) {
        self.current_position.next_move = Some(color);
    }

    fn draw_check(&mut self, move_item: &MoveItem) -> Option<DrawType> {
        let i = self.position_history.len() - 1;
        // let ref mut pos_it = self.position_history[i];
        let cur_position = &mut self.current_position;
        let pos_history = &mut self.position_history;
        if cur_position.state.get_count(Color::White).king > 0 &&
            cur_position.state.get_count(Color::Black).king > 0 {
            // first position where both set kings
            if self.state.kings_start_at.is_none() {
                self.state.kings_start_at = Some(i);
            }
            // 1) если в течение 15 ходов игроки делали ходы только дамками, не передвигая
            // простых шашек и не производя взятия.
            if cur_position.get_piece_of_move_item(move_item).is_king {
                if self.state.kings_only_move_start_at.is_none() {
                    self.state.kings_only_move_start_at = Some(i);
                } else {
                    if i - self.state.kings_only_move_start_at.unwrap() > 15 {
                        return Some(DrawType::Draw1);
                    }
                }
            } else { self.state.kings_only_move_start_at = None; }


            // 2) если три раза повторяется одна и та же позиция
            let mut repeats = 0;
            let mut j = i;
            while pos_history[j].position.state == cur_position.state {
                if *cur_position == pos_history[j].position {
                    repeats += 1;
                    if repeats == 3 { return Some(DrawType::Draw2); }
                }
                j -= 1;
                if j < self.state.kings_start_at.unwrap_or(0) { break; }
            }

            // 3) если участник, имеющий три дамки (и более) против одной дамки противника,
            // за 15 ходов не возьмёт дамку противника
            let is_triangle = |state: &mut PosState| {
                (state.get_count(Color::White).king == 1 && state.get_count(Color::Black).king >= 3) ||
                    (state.get_count(Color::Black).king == 1 && state.get_count(Color::White).king >= 3)
            };
            if is_triangle(&mut pos_history[i].position.state) {
                if self.state.triangle_start_at.is_none() { self.state.triangle_start_at = Some(i); } else {
                    if is_triangle(&mut cur_position.state) &&
                        1 + i - self.state.triangle_start_at.unwrap() >= 15 { return Some(DrawType::Draw3); }
                }
            } else { self.state.triangle_start_at = None; }

            // 4) если в позиции, в которой оба соперника имеют дамки, не изменилось соотношение сил
            // (то есть не было взятия, и ни одна простая шашка не стала дамкой) на протяжении:
            // в 2- и 3-фигурных окончаниях — 5 ходов,
            // в 4- и 5-фигурных окончаниях — 30 ходов,
            // в 6- и 7-фигурных окончаниях — 60 ходов;
            if i > 0 {
                if cur_position.state == pos_history[i].position.state {
                    if self.state.power_equal_start_at.is_none() { self.state.power_equal_start_at = Some(i); }
                    let total = cur_position.state.get_total();
                    let n = 1 + i - self.state.power_equal_start_at.unwrap();
                    if total < 4 && n > 5 { return Some(DrawType::Draw4); }
                    if total < 6 && n > 30 { return Some(DrawType::Draw4); }
                    if total < 8 && n > 60 { return Some(DrawType::Draw4); }
                } else { self.state.power_equal_start_at = None; }
            }

            // если участник, имея в окончании партии три дамки, две дамки и простую, дамку и две простые,
            // ""три простые против одинокой дамки"", находящейся на большой дороге,
            // своим 5-м ходом не сможет добиться выигранной позиции;
            let is_single_on_main_road = |position: &mut Position| -> bool {
                let ref mut state = position.state;
                if (state.get_count(Color::Black).king == 1 ||
                    state.get_count(Color::White).king == 1) &&
                    state.get_total() == 4 {
                    let color = if state.get_count(Color::Black).king == 1 {
                        Color::Black
                    } else { Color::White };
                    for main_road_point in self.position_environment.get_vectors(0)[0].points.iter() {
                        if let Some(piece) = &position.cells[*main_road_point] {
                            return if piece.color == color { true } else {
                                false
                            };
                        }
                    }
                }
                false
            };

            if is_single_on_main_road(cur_position) {
                if is_single_on_main_road(&mut pos_history[i].position) {
                    if self.state.main_road_start_at.is_none() {
                        self.state.main_road_start_at = Some(i);
                    }
                    if is_single_on_main_road(cur_position) && 1 + i - self.state.main_road_start_at.unwrap() > 5 {
                        return Some(DrawType::Draw5);
                    }
                } else { self.state.main_road_start_at = None; }
            }
        } else { self.state.kings_start_at = None; }

        None
    }

    #[wasm_bindgen]
    pub fn make_move_for_front(&mut self, pos_chain: &JsValue) -> Result<JsValue, JsValue> {
        let mut pos_list: Vec<BoardPos> = Vec::new();
        let iterator = js_sys::try_iter(pos_chain)?.ok_or_else(|| {
            "need to pass iterable JS values!"
        })?;
        for x in iterator {
            // If the iterators `next` method throws an error, propagate it
            // up to the caller.
            let x = x?;

            // If `x` is a number, add it to our array of numbers!
            if x.as_f64().is_some() {
                pos_list.push(x.as_f64().unwrap() as BoardPos);
            }
        }
        if !pos_list.is_empty() {
            if self.current_position.cells[pos_list[0] as usize].is_some() {
                let move_list = self.get_move_list(true);
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
                        let draw = self.draw_check(&mut move_item);
                        self.position_history.push(PositionHistoryItem { move_item, position: self.current_position.clone() });
                        return if draw.is_none() { Ok(JsValue::TRUE) } else {
                            Ok(serde_wasm_bindgen::to_value(&draw.unwrap()).unwrap())
                        };
                    }
                }
            }
        }
        Ok(JsValue::FALSE)
    }
}

#[cfg(test)]
mod tests {
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
        game.current_position.next_move = Option::from(Color::White);
        game.insert_piece(Piece::new(13, Color::White, true));
        vec![2, 27, 24].iter().for_each(|pos| game.insert_piece(Piece::new(*pos, Color::White, false)));
        let list = game.get_move_list(true);
        print!("\ngame_quite_move {:?} \n", {
            let z: Vec<_> = list.list.iter().map(|x| x.mov.clone().unwrap()).collect();
            z
        });
        assert_eq!(list.list.len(), 15);
    }

    #[test]
    pub fn game_strike_list() {
        let mut game = Game::new(8);
        game.current_position.next_move = Some(Color::White);
        game.insert_piece(Piece::new(game.to_pack(47), Color::White, false));
        game.insert_piece(Piece::new(game.to_pack(63), Color::White, false));
        game.insert_piece(Piece::new(game.to_pack(15), Color::White, true));
        vec![54, 43, 20].iter()
            .for_each(|pos|
                game.insert_piece(Piece::new(game.to_pack(*pos), Color::Black, false)));
        for _t in 0..1000000 {
            let _list = game.get_move_list(true);
        }
        let list = game.get_move_list(true);
        print!("\ngame_quite_move {:?} \n", {
            let z: Vec<_> = list.list.iter().map(|x| x.strike.clone().unwrap()).collect();
            z
        });
        assert_eq!(list.list.len(), 5);
    }

    #[test]
    fn performance() {
        PositionEnvironment::game();
    }
}