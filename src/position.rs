use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::{Cell, Color, MutPiece, Piece};
use crate::game::{Game, HashRcWrap};
use ts_rs::TS;
use crate::StraightStrike::{BoardPos, StraightStrike};
use crate::vector::Vector;


#[derive(Deserialize, Serialize, Debug)]
#[derive(TS)]
pub struct Position {
    pub cells: Vec<Cell>,
    game: RefCell<Game>,
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
    pub fn new(game: RefCell<Game>) -> Position {
        let mut pos = Position { cells: Vec::new(), game, pieces: HashMap::new() };
        pos.cells = Vec::new();
        let size = pos.game.borrow_mut().size;
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
    pub fn get_piece(&mut self, pos: usize) -> Option<RefMut<'_, Piece>> {
        self.cells[pos].get_piece()
    }

    pub fn get_piece_by_v(&self, v: &Rc<Vec<BoardPos>>, i: usize) -> Option<RefMut<'_, Piece>> {
        let pos = v[i];
        self.cells[pos].get_piece()
    }
    pub fn swap(&mut self, i: BoardPos, j: BoardPos) {
        self.cells.swap(i as usize, j as usize);
        self.cells[i as usize].set_pos(i);
        self.cells[j as usize].set_pos(j);
    }

    pub fn get_strike_list(&mut self)  {
        let mut straight_strike = |v: &Rc<Vec<BoardPos>>|->Option<StraightStrike> {
            let mut piece_ = self.get_piece_by_v(&v, 0);
            if piece_.is_none() || v.len() < 3
            { None } else {
                let piece = piece_.unwrap();
                let color = piece.color;
                let max_search_steps = if piece.is_king { v.len() } else { 2 };
                let mut i: BoardPos = 2;
                while i <= max_search_steps {
                    let candidate = self.get_piece_by_v(&v, i - 1);
                    if self.get_piece_by_v(&v, i).is_none() && candidate.is_some() && candidate.unwrap().color != color {
                        let mut strike = StraightStrike{v:Vec::new(), to: i, take: i-1};
                        strike.v.push(piece.pos);
                        strike.v.push(i);
                        while self.get_piece_by_v(&v, i+1).is_none() && i+1 <= max_search_steps {
                            i += 1;
                            strike.v.push(i);
                        }
                        return Some(strike)
                    }
                }
                None
            }
        };
        let mut pos_strike = |pos: i16, direction: i8| {
            let game = self.game.borrow_mut();
            let vectors = game.get_vectors(pos as usize);
            let piece = self.cells[pos as usize].get_piece();
            if let Some(piece) = piece {
                for v in vectors {
                    let directions;
                    if !piece.is_king {
                        directions = if piece.color == Color::White { [0, 1] } else { [2, 3] };
                        if piece.is_king || directions.contains(&v.get_unwrap().direction) {
                            straight_strike(&v.get_unwrap().points);
                        }
                    }
                }
            }
        };
    }
}
