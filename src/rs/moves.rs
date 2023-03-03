use core::fmt;
use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::moves_list::Strike;
use tsify::{declare, Tsify};
use wasm_bindgen::prelude::*;

#[declare]
pub type BoardPos = usize;


#[derive(Clone, Serialize, Deserialize, TS)]
#[wasm_bindgen]
#[ts(export)]
pub struct StraightStrike {
    pub(crate) v: Vec<BoardPos>,
    pub(crate) from: BoardPos,
    pub(crate) take: BoardPos,
    pub(crate) to: BoardPos,
    pub(crate) king_move: bool,
}

impl Debug for StraightStrike {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\nfrom: {}, to: {}, take: {}", self.from, self.to, self.take
        )
    }
}

pub struct StraightStrikeIter<'a> {
    v: &'a Vec<BoardPos>,
    rest: BoardPos,
}

impl <'a> Iterator for StraightStrikeIter<'a> {
    type Item = BoardPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest < self.v.len() {
            self.rest += 1;
            Some(self.v[self.rest - 1])
        } else {
            None
        }
    }
}

impl <'a> IntoIterator for &'a StraightStrike {
    type Item = BoardPos;
    type IntoIter = StraightStrikeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        StraightStrikeIter {
            rest: 0,
            v: &self.v,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[ts(export)]
pub struct QuietMove {
    pub from: BoardPos,
    pub to: BoardPos,
    pub king_move: bool,
}


impl PieceMove for QuietMove {
    fn take(&self) -> Option<BoardPos> {
        None
    }
    fn from(&self) -> BoardPos {
        self.from
    }
    fn to(&self) -> BoardPos {
        self.to
    }
    fn set_as_king(&mut self) {
        self.king_move = true;
    }
    fn is_king(&self) -> bool {
        self.king_move
    }
}


impl PieceMove for StraightStrike {
    fn take(&self) -> Option<BoardPos> {
        Some(self.take)
    }
    fn from(&self) -> BoardPos {
        self.from
    }
    fn to(&self) -> BoardPos {
        self.to
    }
    fn set_as_king(&mut self) {
        self.king_move = true;
    }
    fn is_king(&self) -> bool {
        self.king_move
    }
}

impl ChainPieceMove for Strike {
    fn from(&self) -> BoardPos {
        self.vec[0].from
    }
    fn to(&self) -> BoardPos {
        self.vec[self.vec.len() - 1].to
    }
    fn set_as_king(&mut self) {
        self.king_move = true;
    }
    fn is_king_move(&self) -> bool {
        self.king_move
    }
}

impl ChainPieceMove for QuietMove {
    fn from(&self) -> BoardPos { self.from }
    fn to(&self) -> BoardPos {
        self.to
    }
    fn set_as_king(&mut self) {
        self.king_move = true;
    }
    fn is_king_move(&self) -> bool {
        self.king_move
    }
}


pub trait PieceMove: Debug {
    fn take(&self) -> Option<BoardPos>;
    fn from(&self) -> BoardPos;
    fn to(&self) -> BoardPos;
    fn set_as_king(&mut self);
    fn is_king(&self) -> bool;
}


pub trait ChainPieceMove: Debug {
    fn from(&self) -> BoardPos;
    fn to(&self) -> BoardPos;
    fn set_as_king(&mut self);
    fn is_king_move(&self) -> bool;
}