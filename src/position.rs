use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{Cell, Color, MutPiece, Piece};
use crate::game::{Game, HashRcWrap};
use ts_rs::TS;



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
            pieces: new_pieces
        }
    }
}

impl Position {
    pub fn new(game: RefCell<Game>) -> Position {
        let mut pos = Position { cells: Vec::new(), game, pieces: HashMap::new() };
        pos.cells = Vec::new();
        let size = pos.game.borrow_mut().size;
        pos.cells.resize((size*size / 2) as usize, Cell::None);
        pos
    }
    pub fn inset_piece(&mut self, piece: Piece) {
        let pos = piece.pos as usize;
        let color = piece.color;
        let rc_piece = HashRcWrap::new( piece);
        self.cells[pos] = Some(rc_piece.clone());
        let mut set = self.pieces.get_mut(&color);
        if set.is_none() {
            self.pieces.insert(color, HashSet::new());
            set = self.pieces.get_mut(&color);
        }
        let x = set.unwrap();
        x.insert(rc_piece.clone());
        print!("{:?}",x);
    }

    pub fn swap(&mut self, i: i16, j: i16) {
        self.cells.swap(i as usize, j as usize);
        self.cells[i as usize].set_pos(i);
        self.cells[j as usize].set_pos(j);
    }
}
