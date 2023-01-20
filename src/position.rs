use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::ops::DerefMut;
use std::rc::Rc;
use js_sys::Math::min;

use serde::{Deserialize, Serialize};

use crate::{Cell, Color, Piece};
use crate::game::{Game, HashRcWrap};
use ts_rs::TS;
use crate::Moves::{BoardPos, PieceMove, StraightStrike};
use crate::vector::Vector;


#[derive(Deserialize, Serialize, Debug)]
#[derive(TS)]
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
        let mut pos = Position { cells: Vec::new(), game, pieces: HashMap::new() };
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

    pub fn make_move<T: PieceMove>(&mut self, mov: &T) {
        self.swap(mov.from(), mov.to());
        if let Some(take) = mov.take() {
            if let Some(cell) = &self.cells[take] {
                cell.get_unwrap_mut().stricken = true;
            }
        }
    }

    pub fn ummake_move<T: PieceMove>(&mut self, mov: &T) {
        self.swap(mov.from(), mov.to());
        if let Some(take) = mov.take() {
            if let Some(cell) = &self.cells[take] {
                cell.get_unwrap_mut().stricken = false;
            }
        }
    }

    pub fn get_piece_by_v(&self, v: &Rc<Vec<BoardPos>>, i: usize) -> Option<HashRcWrap<Piece>> {
        let pos = v[i];
        if let Some(piece) = self.cells[pos].clone() {
            Some(piece)
        } else { None }
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
        if v.len() < 3 { return None; }

        if let Some(piece) = self.get_piece_by_v(&v, 0) {
            let piece = piece.get_unwrap();
            let color = piece.color;
            let max_search_steps = if piece.is_king { v.len() - 1 } else { 2 };
            let mut i: BoardPos = 2;
            while i <= max_search_steps {
                if let Some(candidate) = self.get_piece_by_v(&v, i - 1) {
                    if self.get_piece_by_v(&v, i).is_none() {
                        let candidate = candidate.get_unwrap();
                        if candidate.color !=color && !candidate.stricken {
                            let mut strike = StraightStrike { v: Vec::new(), from: v[0], to: v[i], take: v[i - 1] };
                            strike.v.push(piece.pos);
                            strike.v.push(v[i]);
                            while i + 1 <= max_search_steps && self.get_piece_by_v(&v, i + 1).is_none() {
                                i += 1;
                                strike.v.push(v[i]);
                            }
                            return Some(strike);
                        }
                    } else { break }
                }
                i += 1;
            }
        }
        None
    }

    pub fn get_strike_list(&mut self, pos: BoardPos, ban_direction: i8) {
        let game = &self.game;// self.game.borrow_mut();
        let vectors: Vec<HashRcWrap<Vector<BoardPos>>> =
            game.get_unwrap().get_vectors(pos).into_iter().filter(|v|
                v.get_unwrap().direction != ban_direction).collect();
        if vectors.len() > 0 {
            let piece = self.get_piece_by_v(&vectors[0].get_unwrap().points, 0);
            if let Some(piece) = piece {
                for v in vectors {
                    let directions = {
                        let piece = piece.get_unwrap();
                        if !piece.is_king {
                            if piece.color == Color::White { vec![0, 1] } else { vec![2, 3] }
                        } else { vec![0, 1, 2, 3] }
                    };
                    let strike = {
                        let v = v.get_unwrap();
                        if directions.contains(&v.direction) {
                            self.straight_strike(&v.points)
                        } else { None }
                    };
                    if let Some(strike) = strike {
                        self.make_move(&strike);
                    }
                }
            }
        }
    }
}

