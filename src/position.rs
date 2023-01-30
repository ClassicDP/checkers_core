use js_sys::Math::min;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::game::Game;
use crate::vector::Vector;
use crate::Moves::{BoardPos, PieceMove, StraightStrike};
use crate::MovesList::{MoveItem, MoveList};
use crate::{Color, Piece};
use ts_rs::TS;
use crate::HashRcWrap::HashRcWrap;

pub type Cell = Option<HashRcWrap<Piece>>;
#[derive(Deserialize, Serialize, Debug, TS)]
pub struct Position {
    pub cells: Vec<Cell>,
    pub game: HashRcWrap<Game>,
    #[serde(skip_serializing, skip_deserializing)]
    pub pieces: HashMap<Color, HashSet<HashRcWrap<Piece>>>,
}

impl Clone for Position {
    fn clone(&self) -> Self {
        let mut new_pieces: HashMap<Color, HashSet<HashRcWrap<Piece>>> = HashMap::new();
        for (col, hash_set) in &self.pieces {
            let mut new_hashset: HashSet<HashRcWrap<Piece>> = HashSet::new();
            for x in hash_set {
                new_hashset.insert(x.clone());
            }
            new_pieces.insert(col.clone(), new_hashset);
        }
        Position {
            cells: self.cells.clone(),
            game: self.game.clone(),
            pieces: new_pieces,
        }
    }
}

impl Position {
    pub fn new(game: HashRcWrap<Game>) -> Position {
        let mut pos = Position {
            cells: Vec::new(),
            game,
            pieces: HashMap::new(),
        };
        pos.cells = Vec::new();
        let size = pos.game.get_unwrap().size;
        pos.cells.resize((size * size / 2) as usize, Cell::None);
        pos
    }
    pub fn inset_piece(&mut self, piece: Piece) {
        let pos = piece.pos as usize;
        let color = piece.color;
        let rc_piece = HashRcWrap::new(piece);
        self.cells[pos] = Some(rc_piece.clone());
        let mut set = self.pieces.get_mut(&color);
        if set.is_none() {
            self.pieces.insert(color, HashSet::new());
            set = self.pieces.get_mut(&color);
        }
        let x = set.unwrap();
        x.insert(rc_piece.clone());
        print!("{:?}", x);
    }

    pub fn make_move(&mut self, mov: &mut dyn PieceMove) {
        self.swap(mov.from(), mov.to());
        if let Some(take) = mov.take() {
            if let Some(cell) = &self.cells[take] {
                cell.get_unwrap_mut().stricken = true;
            }
        }
        if let Some(piece) = &self.cells[mov.to()] {
            let mut piece = piece.get_unwrap_mut();
            if !piece.is_king {
                if self.game.get_unwrap().is_king_row(piece.deref()) {
                    piece.is_king = true;
                    mov.set_as_king();
                }
            }
        }
    }

    pub fn ummake_move(&mut self, mov: &dyn PieceMove) {
        self.swap(mov.from(), mov.to());
        if let Some(take) = mov.take() {
            if let Some(cell) = &self.cells[take] {
                cell.get_unwrap_mut().stricken = false;
            }
        }
        if mov.is_king() {
            if let Some(piece) = &self.cells[mov.from()] {
                piece.get_unwrap_mut().is_king = false;
            }
        }
    }

    pub fn get_piece_by_v(&self, v: &Rc<Vec<BoardPos>>, i: usize) -> Option<HashRcWrap<Piece>> {
        let pos = v[i];
        if let Some(piece) = self.cells[pos].clone() {
            Some(piece)
        } else {
            None
        }
    }
    pub fn swap(&mut self, i: BoardPos, j: BoardPos) {
        self.cells.swap(i as usize, j as usize);
        let set_pos = |cell: &Cell, pos: BoardPos| {
            if let Some(cell) = cell {
                cell.get_unwrap_mut().pos = pos;
            }
        };
        set_pos(&self.cells[i], i);
        set_pos(&self.cells[j], j);
    }

    fn straight_strike(&mut self, v: &Rc<Vec<BoardPos>>) -> Option<StraightStrike> {
        if v.len() < 3 {
            return None;
        }
        if let Some(piece) = self.get_piece_by_v(&v, 0) {
            let piece = piece.get_unwrap();
            let color = piece.color;
            let max_search_steps = if piece.is_king { v.len() } else { 3 };
            let mut i: usize = 2;
            while i <= max_search_steps {
                if let Some(take_candidate) = self.get_piece_by_v(&v, i - 1) {
                    if self.get_piece_by_v(&v, i).is_none() {
                        let candidate = take_candidate.get_unwrap();
                        if candidate.color != color && !candidate.stricken {
                            let strike = StraightStrike {
                                v: HashRcWrap::new(Vec::new()),
                                from: v[0],
                                to: v[i],
                                i_to: 0,
                                take: v[i - 1],
                                king_move: false,
                            };
                            {
                                let mut ve = strike.v.get_unwrap_mut();
                                while i < max_search_steps && self.get_piece_by_v(&v, i).is_none() {
                                    ve.push(v[i]);
                                    i += 1;
                                }
                            }
                            return Some(strike);
                        }
                    } else {
                        break;
                    }
                }
                i += 1;
            }
        }
        None
    }

    pub fn get_strike_list(
        &mut self,
        pos: BoardPos,
        ban_directions: &Vec<i8>,
        move_list: &mut MoveList,
    ) -> bool {
        let mut success_call = false;
        if let Some(piece) = &self.cells[pos] {
            let directions = {
                let piece = piece.get_unwrap();
                if !piece.is_king {
                    if piece.color == Color::White {
                        vec![0, 1]
                    } else {
                        vec![2, 3]
                    }
                } else {
                    vec![0, 1, 2, 3]
                }
            };
            let vectors: Vec<HashRcWrap<Vector<BoardPos>>> = (&self.game)
                .get_unwrap()
                .get_vectors(pos)
                .into_iter()
                .filter(|v| {
                    let v_direction = &v.get_unwrap().direction;
                    !ban_directions.contains(v_direction) && directions.contains(v_direction)
                })
                .collect();
            for v in vectors {
                let strike = self.straight_strike(&v.get_unwrap().points);
                if let Some(mut strike) = strike {
                    success_call = true;
                    let mut ban_directions = vec![v.get_unwrap().get_ban_direction()];
                    let mut recurrent_chain = false;
                    for pos in &strike {
                        let mut strike_move = strike.clone();
                        strike_move.to = pos;
                        self.make_move(&mut strike_move);
                        move_list.current_chain.push(strike_move.clone());
                        if self.get_strike_list(pos, &ban_directions, move_list) {
                            recurrent_chain = true;
                        }
                        move_list.current_chain.pop();
                        self.ummake_move(&strike_move);
                        if ban_directions.len() < 2 {
                            ban_directions.push(v.get_unwrap().direction);
                        }
                    }
                    if !recurrent_chain {
                        for pos in &strike {
                            let mut strike_move = strike.clone();
                            strike_move.to = pos;
                            let mut chain = move_list.current_chain.clone();
                            chain.push(strike_move);
                            move_list.list.push(MoveItem::StrikeChain(chain));
                        }
                    }
                }
            }
        }
        success_call
    }
}
