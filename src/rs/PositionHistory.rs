use std::cell::{RefCell, RefMut};
use crate::moves_list::MoveItem;
use crate::position::{Position, PosState};
use wasm_bindgen::prelude::*;
use ts_rs::*;
use serde::Serialize;
use std::cmp::Ordering;
use std::rc::Rc;
use crate::color::Color::{Black, White};
use crate::PositionHistory::FinishType::{BlackWin, Draw1, Draw2, Draw3, Draw4, Draw5, WhiteWin};

#[wasm_bindgen]
#[derive(Serialize, Debug)]
#[derive(TS)]
#[ts(export)]
pub struct PositionAndMove {
    #[wasm_bindgen(skip)]
    pub pos: Position,
    #[wasm_bindgen(skip)]
    pub mov: Option<MoveItem>,
}

impl PositionAndMove {
    pub fn from(pos: Position, mov: MoveItem) -> PositionAndMove {
        PositionAndMove {
            pos,
            mov: Option::from(mov),
        }
    }
    pub fn from_pos(pos: Position) -> PositionAndMove {
        PositionAndMove {
            pos,
            mov: None,
        }
    }
}
#[derive(Debug)]
pub struct PositionHistory {
    list: Vec<Rc<RefCell<PositionAndMove>>>,
}

impl PositionHistory {
    pub fn len(&self) -> usize {
        self.list.len()
    }
}


impl PositionHistory {
    pub fn new() -> PositionHistory {
        PositionHistory {
            list: vec![]
        }
    }
    pub fn last(&mut self) -> Rc<RefCell<PositionAndMove>> {
        self.list.last().unwrap().clone()
    }

    pub fn cut_to (&mut self, to: usize) {
        self.list = self.list[0..to].to_owned();
    }
    pub fn push(&mut self, pos_mov: PositionAndMove) -> Option<FinishType> {
        self.list.push(Rc::new(RefCell::from(pos_mov)));
        self.finish_check()
    }

    pub fn push_rc(&mut self, pos_mov: Rc<RefCell<PositionAndMove>>) -> Option<FinishType> {
        self.list.push(pos_mov);
        self.finish_check()
    }

    pub fn pop(&mut self) -> Option<Rc<RefCell<PositionAndMove>>> {
        self.list.pop()
    }

    pub fn finish_check(&mut self) -> Option<FinishType> {
        let mut i = self.list.len();
        if i == 0 { return None; }
        let pos_history = &self.list;
        let ref mut current = &pos_history[i - 1];
        let list = current.borrow_mut().pos.get_move_list_cached();

        if list.as_ref().as_ref().unwrap().list.len() == 0 {
            return if current.borrow().pos.next_move.is_some() &&
                current.borrow().pos.next_move.unwrap() == White { Some(BlackWin) } else { Some(WhiteWin) };
        }


        let environment = current.borrow().pos.environment.clone();
        if current.borrow_mut().pos.state.get_count(White).king > 0 &&
            current.borrow_mut().pos.state.get_count(Black).king > 0 {
            i -= 1;
            // first position where both set kings
            if current.borrow().pos.state.kings_start_at.is_none() ||
                current.borrow().pos.state.kings_start_at.unwrap() > i {
                current.borrow_mut().pos.state.kings_start_at = Some(i);
            }

            // 3) если участник, имеющий три дамки (и более) против одной дамки противника,
            // за 15 ходов не возьмёт дамку противника
            let is_triangle = |state: &mut PosState| {
                (state.get_count(White).king == 1 && state.get_count(Black).king >= 3) ||
                    (state.get_count(Black).king == 1 && state.get_count(White).king >= 3)
            };
            if is_triangle(&mut current.borrow_mut().pos.state) {
                if current.borrow().pos.state.triangle_start_at.is_none()
                    || current.borrow().pos.state.triangle_start_at.unwrap() > i {
                    current.borrow_mut().pos.state.triangle_start_at = Some(i);
                } else {
                    if i - current.borrow().pos.state.triangle_start_at.unwrap() >= 15 { return Some(Draw3); }
                }
            } else { current.borrow_mut().pos.state.triangle_start_at = None; }

            if i < 1 { return None; }


            // 1) если в течение 15 ходов игроки делали ходы только дамками, не передвигая
            // простых шашек и не производя взятия.
            if pos_history[i].borrow().pos
                .cells[pos_history[i].borrow().mov.as_ref().unwrap().to()].as_ref().unwrap().is_king {
                if current.borrow().pos.state.kings_only_move_start_at.is_none() ||
                    current.borrow().pos.state.kings_only_move_start_at.unwrap() > i {
                    current.borrow_mut().pos.state.kings_only_move_start_at = Some(i);
                }
                if i - current.borrow().pos.state.kings_only_move_start_at.unwrap() > 15 {
                    return Some(Draw1);
                }
            } else {
                current.borrow_mut().pos.state.kings_only_move_start_at = None;
            }

            // 2) если три раза повторяется одна и та же позиция
            current.borrow_mut().pos.state.repeats = 0;
            let mut j = i - 1;
            while pos_history[j].borrow().pos.state == current.borrow().pos.state {
                if current.borrow().pos == pos_history[j].borrow().pos {
                    current.borrow_mut().pos.state.repeats += 1;
                    if current.borrow().pos.state.repeats > 2 {
                        return Some(Draw2);
                    }
                }
                if j < current.borrow().pos.state.kings_start_at.unwrap_or(0)
                    || j == 0 { break; }
                j -= 1;
            }


            // 4) если в позиции, в которой оба соперника имеют дамки, не изменилось соотношение сил
            // (то есть не было взятия, и ни одна простая шашка не стала дамкой) на протяжении:
            // в 2- и 3-фигурных окончаниях — 5 ходов,
            // в 4- и 5-фигурных окончаниях — 30 ходов,
            // в 6- и 7-фигурных окончаниях — 60 ходов;
            if pos_history[i - 1].borrow().pos.state == pos_history[i].borrow_mut().pos.state {
                if current.borrow().pos.state.power_equal_start_at.is_none()
                    || current.borrow().pos.state.power_equal_start_at.unwrap() > i - 1 {
                    current.borrow_mut().pos.state.power_equal_start_at = Some(i - 1);
                }
                let total = current.borrow().pos.state.get_total();
                // if cur_position.state.power_equal_start_at.is_none() {panic!("!");}
                let n = i - current.borrow().pos.state.power_equal_start_at.unwrap();
                if total < 4 && n > 5 { return Some(Draw4); }
                if total < 6 && n > 30 { return Some(Draw4); }
                if total < 8 && n > 60 { return Some(Draw4); }
            } else { current.borrow_mut().pos.state.power_equal_start_at = None; }

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
                    for main_road_point in environment.get_vectors(0)[0].points.iter() {
                        if let Some(piece) = &position.cells[*main_road_point] {
                            return if piece.color == color { true } else {
                                false
                            };
                        }
                    }
                }
                false
            };
            if is_single_on_main_road(&mut current.borrow_mut().pos) {
                if current.borrow().pos.state.main_road_start_at.is_none() ||
                    current.borrow().pos.state.main_road_start_at.unwrap() > i {
                    current.borrow_mut().pos.state.main_road_start_at = Some(i);
                }
                if i - current.borrow().pos.state.main_road_start_at.unwrap() >= 10 {
                    return Some(Draw5);
                }
            } else { current.borrow_mut().pos.state.main_road_start_at = None; }
        } else { current.borrow_mut().pos.state.kings_start_at = None; }
        None
    }
}


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
