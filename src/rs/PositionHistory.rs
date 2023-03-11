use std::cell::RefCell;
use std::rc::Rc;
use crate::moves_list::MoveItem;
use crate::position::{Position, PosState};
use wasm_bindgen::prelude::*;
use ts_rs::*;
use serde::Serialize;
use std::cmp::Ordering;
use std::ops::Index;
use crate::color::Color::{Black, White};
use crate::PositionHistory::FinishType::{BlackWin, Draw1, Draw2, Draw3, Draw4, Draw5, WhiteWin};

#[wasm_bindgen]
#[derive(Serialize, Debug, Clone)]
#[derive(TS)]
#[ts(export)]
pub struct PositionAndMove {
    #[wasm_bindgen(skip)]
    pub pos: RefCell<Position>,
    #[wasm_bindgen(skip)]
    pub mov: RefCell<Option<MoveItem>>,
}

impl PositionAndMove {
    pub fn from(pos: Position, mov: MoveItem) -> PositionAndMove {
        PositionAndMove {
            pos: RefCell::new(pos),
            mov: RefCell::new(Option::from(mov)),
        }
    }
    pub fn from_ref_cell(pos: RefCell<Position>, mov: RefCell<Option<MoveItem>>) -> PositionAndMove {
        PositionAndMove {
            pos,
            mov
        }
    }
}

pub struct PositionHistory {
    list: Vec<Rc<PositionAndMove>>,
}

impl PositionHistory {
    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl Index<usize> for PositionHistory {
    type Output = Rc<PositionAndMove>;

    fn index(&self, index: usize) -> &Self::Output {
        self.list.index(index)
    }
}

impl PositionHistory {
    pub fn new() -> PositionHistory {
        PositionHistory {
            list: vec![]
        }
    }
    pub fn push(&mut self, pos_mov: PositionAndMove) -> Option<FinishType> {
        self.list.push(Rc::new(pos_mov));
        self.finish_check()
    }

    pub fn pop(&mut self) -> Option<Rc<PositionAndMove>> {
        self.list.pop()
    }

    pub fn finish_check(&mut self) -> Option<FinishType> {
        let i = self.list.len();
        if i==0 {return None}
        let current = self.list[i-1].clone();
        if current.pos.borrow_mut().get_move_list_cached().borrow().list.len() == 0 {
            return if current.pos.borrow().next_move.is_some() &&
                current.pos.borrow().next_move.unwrap() == White { Some(FinishType::BlackWin) } else { Some(FinishType::WhiteWin) };
        }
        if i < 2 { return None; }
        // let ref mut pos_it = self.position_history[i];
        let pos_history = &mut self.list;
        let environment = current.pos.borrow().environment.clone();
        if current.pos.borrow_mut().state.get_count(White).king > 0 &&
            current.pos.borrow_mut().state.get_count(Black).king > 0 {
            // first position where both set kings
            if current.pos.borrow().state.kings_start_at.is_none() || current.pos.borrow().state.kings_start_at.unwrap() > i {
                current.pos.borrow_mut().state.kings_start_at = Some(i);
            }
            // 1) если в течение 15 ходов игроки делали ходы только дамками, не передвигая
            // простых шашек и не производя взятия.
            if i > 1 &&
                pos_history[i - 1].pos.borrow().cells[pos_history[i - 1].mov.borrow_mut().as_ref().unwrap().to()].as_ref().unwrap().is_king {
                if current.pos.borrow().state.kings_only_move_start_at.is_none() ||
                    current.pos.borrow().state.kings_only_move_start_at.unwrap() > i {
                    current.pos.borrow_mut().state.kings_only_move_start_at = Some(i - 1);
                }
                if i - current.pos.borrow().state.kings_only_move_start_at.unwrap() >= 15 {
                    return Some(FinishType::Draw1);
                }
            } else {
                current.pos.borrow_mut().state.kings_only_move_start_at = None;
            }


            // 2) если три раза повторяется одна и та же позиция
            let mut repeats = 0;
            let mut j: i32 = i as i32 - 1;
            while j >= 0 && pos_history[j as usize].pos.borrow().state == current.pos.borrow().state {
                if current.pos == pos_history[j as usize].pos {
                    repeats += 1;
                    if repeats > 1 {
                        return Some(FinishType::Draw2);
                    }
                }
                if j < current.pos.borrow().state.kings_start_at.unwrap_or(0) as i32 { break; }
                j -= 1;
            }
            current.pos.borrow_mut().state.repeats = Some(repeats);

            // 3) если участник, имеющий три дамки (и более) против одной дамки противника,
            // за 15 ходов не возьмёт дамку противника
            let is_triangle = |state: &mut PosState| {
                (state.get_count(White).king == 1 && state.get_count(Black).king >= 3) ||
                    (state.get_count(Black).king == 1 && state.get_count(White).king >= 3)
            };
            if is_triangle(&mut current.pos.borrow_mut().state) {
                if current.pos.borrow().state.triangle_start_at.is_none()
                    || current.pos.borrow().state.triangle_start_at.unwrap() > i { current.pos.borrow_mut().state.triangle_start_at = Some(i); } else {
                    if i - current.pos.borrow().state.triangle_start_at.unwrap() >= 15 { return Some(FinishType::Draw3); }
                }
            } else { current.pos.borrow_mut().state.triangle_start_at = None; }

            // 4) если в позиции, в которой оба соперника имеют дамки, не изменилось соотношение сил
            // (то есть не было взятия, и ни одна простая шашка не стала дамкой) на протяжении:
            // в 2- и 3-фигурных окончаниях — 5 ходов,
            // в 4- и 5-фигурных окончаниях — 30 ходов,
            // в 6- и 7-фигурных окончаниях — 60 ходов;
            if i > 1 && pos_history[i - 1].pos.borrow().state == pos_history[i - 2].pos.borrow().state {
                if current.pos.borrow().state.power_equal_start_at.is_none()
                    || current.pos.borrow().state.power_equal_start_at.unwrap() > i - 1 {
                    current.pos.borrow_mut().state.power_equal_start_at = Some(i - 2);
                }
                let total = current.pos.borrow().state.get_total();
                // if cur_position.state.power_equal_start_at.is_none() {panic!("!");}
                let n = i - current.pos.borrow().state.power_equal_start_at.unwrap();
                if total < 4 && n >= 5 { return Some(Draw4); }
                if total < 6 && n >= 30 { return Some(Draw4); }
                if total < 8 && n >= 60 { return Some(Draw4); }
            } else { current.pos.borrow_mut().state.power_equal_start_at = None; }

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
            if is_single_on_main_road(&mut *current.pos.borrow_mut()) {
                if current.pos.borrow().state.main_road_start_at.is_none() ||
                    current.pos.borrow().state.main_road_start_at.unwrap() > i {
                    current.pos.borrow_mut().state.main_road_start_at = Some(i);
                }
                if i - current.pos.borrow().state.main_road_start_at.unwrap() >= 10 {
                    return Some(FinishType::Draw5);
                }
            } else { current.pos.borrow_mut().state.main_road_start_at = None; }
        } else { current.pos.borrow_mut().state.kings_start_at = None; }
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
