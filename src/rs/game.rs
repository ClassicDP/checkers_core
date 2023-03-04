use std::cmp::Ordering;
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
use crate::color::Color::{Black, White};
use crate::game::FinishType::{BlackWin, Draw1, Draw2, Draw3, Draw4, Draw5, WhiteWin};

#[wasm_bindgen]
#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Debug, Clone)]
pub enum FinishType {
    Draw1,
    Draw2,
    Draw3,
    Draw4,
    Draw5,
    BlackWin,
    WhiteWin,
}


impl PartialOrd<Self> for FinishType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for FinishType {}

impl PartialEq<Self> for FinishType {
    fn eq(&self, other: &Self) -> bool {
        let is_draw = |x: &FinishType| {
            match x {
                Draw1 | Draw2 | Draw3 | Draw4 | Draw5 => { true }
                _ => { false }
            }
        };
        let is_win_same = |x: &FinishType, y: &FinishType| {
            match x {
                WhiteWin => match y {
                    WhiteWin => true,
                    _ => false
                }
                BlackWin => match y {
                    BlackWin => true,
                    _ => false
                }
                _ => false
            }
        };
        is_draw(self) && is_draw(other) || is_win_same(self, other)
    }
}


impl Ord for FinishType {
    fn cmp(&self, other: &Self) -> Ordering {
        if *self == BlackWin && *other != BlackWin { return Ordering::Less; }
        if *self == WhiteWin && *other != WhiteWin { return Ordering::Greater; }
        Ordering::Equal
    }
}
#[wasm_bindgen]
#[derive(Serialize, Debug)]
pub struct BestPos {
    pos: Option<PositionHistoryItem>,
    deep_eval: i32,
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

    fn best_move(&mut self, mut max_depth: i16, best_white: &mut i32,
                 best_black: &mut i32, depth: i16) -> BestPos {

        let ref mut move_list = self.current_position.get_move_list_cached();
        let mut pos_list: Vec<_> = {
            move_list.borrow_mut().list.iter_mut().map(|x| {
                let mut pos = self.current_position.make_move_and_get_position(x);
                pos.position.evaluate();
                self.current_position.unmake_move(x);
                pos
            }).collect()
        };
        if pos_list.len() == 0 { panic!("Best move: it`s standoff position") }
        let move_color = self.current_position.next_move.unwrap();
        if pos_list.len() < 3 { max_depth += 1; }
        pos_list.sort_by_key(|x|
            x.position.eval.unwrap() * if move_color == White { -1 } else { 1 });
        let mut best_pos = BestPos { pos: None, deep_eval: if move_color == White { i32::MIN } else { i32::MAX } };
        if depth < max_depth {
            for mut pos_it in pos_list {
                self.current_position.make_move(&mut pos_it.move_item);
                if self.finish_check(&pos_it.move_item).is_some() {
                    return BestPos{ deep_eval: pos_it.position.evaluate(), pos: Option::from(pos_it)};
                }
                self.position_history.push(pos_it);
                let deep_eval =
                    self.best_move(max_depth, best_white, best_black, depth + 1).deep_eval;
                let mut pos_it = self.position_history.pop().unwrap();
                self.current_position.unmake_move(&mut pos_it.move_item);
                if move_color == White {
                    if *best_black < deep_eval {
                        return BestPos{pos: Option::from(pos_it), deep_eval}
                    }
                    if best_pos.deep_eval < deep_eval {
                        best_pos = BestPos { pos: Option::from(pos_it), deep_eval };
                    }
                } else {
                    if *best_white > deep_eval {
                        return BestPos{pos: Option::from(pos_it), deep_eval};
                    }
                    if best_pos.deep_eval > deep_eval {
                        best_pos = BestPos { pos: Option::from(pos_it), deep_eval };
                    }
                }
            }
        } else {
            for mut pos in pos_list {
                best_pos = BestPos{ deep_eval: pos.position.evaluate(), pos: Some(pos)}
            }
        }
        if move_color==White {
            if *best_white < best_pos.deep_eval {*best_white = best_pos.deep_eval}
        } else {
            if *best_black > best_pos.deep_eval {*best_black = best_pos.deep_eval}
        }
        best_pos
    }

    #[wasm_bindgen]
    pub fn get_best_move(&mut self) -> JsValue {
        match serde_wasm_bindgen::to_value(
            &self.best_move(6, &mut i32::MIN, &mut i32::MAX, 0)) {
            Ok(js) => js,
            Err(_err) => JsValue::UNDEFINED
        }
    }

    #[wasm_bindgen]
    pub fn get_best_move_rust(&mut self) -> BestPos {
        self.best_move(6, &mut i32::MIN, &mut i32::MAX, 0)
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


    pub fn finish_check(&mut self, move_item: &MoveItem) -> Option<FinishType> {
        let cur_position = &mut self.current_position;
        if cur_position.get_move_list_cached().borrow().list.len() == 0 {
            return if cur_position.next_move.is_some() &&
                cur_position.next_move.unwrap() == White { Some(BlackWin) } else { Some(WhiteWin) };
        }
        if self.position_history.len() < 1 { return None; }
        let i = self.position_history.len() - 1;
        // let ref mut pos_it = self.position_history[i];
        let pos_history = &mut self.position_history;
        if pos_history[i].position.state.get_count(White).king > 0 &&
            pos_history[i].position.state.get_count(Black).king > 0 {
            // first position where both set kings
            if self.state.kings_start_at.is_none() {
                self.state.kings_start_at = Some(i);
            }
            // 1) если в течение 15 ходов игроки делали ходы только дамками, не передвигая
            // простых шашек и не производя взятия.
            if pos_history[i].position.get_piece_of_move_item(move_item).is_king {
                if self.state.kings_only_move_start_at.is_none() {
                    self.state.kings_only_move_start_at = Some(i);
                }
                if cur_position.get_piece_of_move_item(move_item).is_king &&
                    1 + i - self.state.kings_only_move_start_at.unwrap() > 15 {
                    return Some(Draw1);
                }
            } else { self.state.kings_only_move_start_at = None; }


            // 2) если три раза повторяется одна и та же позиция
            let mut repeats = 0;
            let mut j = i;
            while pos_history[j].position.state == cur_position.state {
                if *cur_position == pos_history[j].position {
                    repeats += 1;
                    if repeats == 3 { return Some(Draw2); }
                }
                j -= 1;
                if j < self.state.kings_start_at.unwrap_or(0) { break; }
            }

            // 3) если участник, имеющий три дамки (и более) против одной дамки противника,
            // за 15 ходов не возьмёт дамку противника
            let is_triangle = |state: &mut PosState| {
                (state.get_count(White).king == 1 && state.get_count(Black).king >= 3) ||
                    (state.get_count(Black).king == 1 && state.get_count(White).king >= 3)
            };
            if is_triangle(&mut pos_history[i].position.state) {
                if self.state.triangle_start_at.is_none() { self.state.triangle_start_at = Some(i); } else {
                    if is_triangle(&mut cur_position.state) &&
                        1 + i - self.state.triangle_start_at.unwrap() >= 15 { return Some(Draw3); }
                }
            } else { self.state.triangle_start_at = None; }

            // 4) если в позиции, в которой оба соперника имеют дамки, не изменилось соотношение сил
            // (то есть не было взятия, и ни одна простая шашка не стала дамкой) на протяжении:
            // в 2- и 3-фигурных окончаниях — 5 ходов,
            // в 4- и 5-фигурных окончаниях — 30 ходов,
            // в 6- и 7-фигурных окончаниях — 60 ходов;
            if i > 0 {
                if pos_history[i - 1].position.state == pos_history[i].position.state {
                    if self.state.power_equal_start_at.is_none() { self.state.power_equal_start_at = Some(i); }
                    if cur_position.state == pos_history[i].position.state {
                        let total = cur_position.state.get_total();
                        let n = 1 + i - self.state.power_equal_start_at.unwrap();
                        if total < 4 && n > 5 { return Some(Draw4); }
                        if total < 6 && n > 30 { return Some(Draw4); }
                        if total < 8 && n > 60 { return Some(Draw4); }
                    }
                } else { self.state.power_equal_start_at = None; }
            }

            // если участник, имея в окончании партии три дамки, две дамки и простую, дамку и две простые,
            // ""три простые против одинокой дамки"", находящейся на большой дороге,
            // своим 5-м ходом не сможет добиться выигранной позиции;
            let is_single_on_main_road = |position: &mut Position| -> bool {
                let ref mut state = position.state;
                if (state.get_count(Black).king == 1 ||
                    state.get_count(White).king == 1) &&
                    state.get_total() == 4 {
                    let color = if state.get_count(Black).king == 1 {
                        Black
                    } else { White };
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
                        return Some(Draw5);
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
                        let draw = self.finish_check(&mut move_item);
                        self.position_history.push(
                            PositionHistoryItem { move_item, position: self.current_position.clone() });
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
    use crate::game::{Game};
    use crate::game::FinishType::{BlackWin, Draw1, Draw2, Draw3, WhiteWin};
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
    pub fn best_move() {
        let mut game = Game::new(8);
        game.current_position.next_move = Some(Color::White);
        game.insert_piece(Piece::new(game.to_pack(0), Color::White, true));
        vec![9, 11, 13, 25, 27, 29, 41, 43, 45].iter()
            .for_each(|pos|
                game.insert_piece(Piece::new(game.to_pack(*pos), Color::Black, false)));

        let best = game.get_best_move_rust();
        print!("\n best: {:?} \n", {
            best
        });
    }

    #[test]
    fn finish_cmp() {
        assert_eq!(Draw2, Draw1);
        assert_eq!(WhiteWin, WhiteWin);
        assert_eq!(BlackWin, BlackWin);
        assert_ne!(BlackWin, WhiteWin);
        assert_ne!(Draw2, WhiteWin);
        assert_eq!(WhiteWin > BlackWin, true);
        assert_eq!(BlackWin < WhiteWin, true);
        assert_eq!(WhiteWin < BlackWin, false);
        assert_eq!(WhiteWin > Draw3, true);
        assert_eq!(BlackWin < Draw1, true);
    }

    #[test]
    fn performance() {
        PositionEnvironment::game();
    }
}