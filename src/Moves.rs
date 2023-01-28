use std::fmt::Debug;
use std::rc::Rc;
use std::vec::IntoIter;
use crate::game::HashRcWrap;

pub type BoardPos = usize;



#[derive(Debug, Clone)]
pub struct StraightStrike {
    pub(crate) v: HashRcWrap<Vec<BoardPos>>,
    pub(crate) from: BoardPos,
    pub(crate) take: BoardPos,
    pub(crate) to: BoardPos,
    pub(crate) king_move: bool
}

// impl Clone for StraightStrike {
//     fn clone(&self) -> Self {
//         StraightStrike {
//             v: self.v.clone(),
//             from: self.from,
//             to: self.to,
//             take: self.take,
//             king_move: self.king_move,
//         }
//     }
// }


pub struct StraightStrikeIter {
    v: HashRcWrap<Vec<BoardPos>>,
    rest: BoardPos,
}

impl Iterator for StraightStrikeIter {
    type Item = BoardPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest < self.v.get_unwrap().len() {
            self.rest += 1;
            Some(self.v.get_unwrap()[self.rest - 1])
        } else { None }
    }
}

impl IntoIterator for &StraightStrike {
    type Item = BoardPos;
    type IntoIter = StraightStrikeIter;

    fn into_iter(self) -> Self::IntoIter {
        StraightStrikeIter {
            rest: self.to,
            v: self.v.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    pub(crate) v: Vec<BoardPos>,
    pub(crate) from: BoardPos,
    pub(crate) to: BoardPos,
    pub(crate) king_move: bool
}

impl PieceMove for Move {
    fn from(&self) -> BoardPos {
        self.from
    }

    fn to(&self) -> BoardPos {
        self.to
    }

    fn take(&self) -> Option<BoardPos> {
        None
    }

    fn set_as_king(&mut self) {
        self.king_move = true;
    }

    fn is_king(&self) -> bool {
        self.king_move
    }
}


impl PieceMove for StraightStrike {
    fn from(&self) -> BoardPos {
        self.from
    }

    fn to(&self) -> BoardPos {
        self.to
    }

    fn take(&self) -> Option<BoardPos> {
        Some(self.take)
    }

    fn set_as_king(&mut self) {
        self.king_move = true;
    }

    fn is_king(&self) -> bool {
        self.king_move
    }
}


pub trait PieceMove: Debug {
    fn from(&self) -> BoardPos;
    fn to(&self) -> BoardPos;
    fn take(&self) -> Option<BoardPos>;
    fn set_as_king(&mut self);
    fn is_king(&self) -> bool;
}
