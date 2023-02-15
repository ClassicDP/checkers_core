use std::borrow::Borrow;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::moves::{PieceMove, QuietMove, StraightStrike};
use crate::piece::Piece;
use ts_rs::TS;


#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(TS)]
#[ts(export)]
pub struct Strike {
    pub vec: Vec<StraightStrike>,
    pub took_pieces: Vec<Piece>,
    pub king_move: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(TS)]
#[ts(export)]
pub struct MoveItem {
    pub strike: Option<Strike>,
    pub mov: Option<QuietMove>,
}

pub struct MoveItemIter<'a> {
    list: Vec<Rc<&'a dyn PieceMove>>,
    ind: usize
}

impl <'a> Iterator for MoveItemIter<'a> {
    type Item = Rc<&'a dyn PieceMove>;

    fn next(&mut self) -> Option<Rc<&'a dyn PieceMove>>{
        if self.ind< self.list.len() {
            self.ind += 1;
            Some(self.list[self.ind - 1].clone())
        } else {
            None
        }
    }
}

impl <'a> IntoIterator for &'a MoveItem {
    type Item = Rc<&'a dyn PieceMove>;
    type IntoIter = MoveItemIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MoveItemIter {
            list: {
                let mut v: Vec<Rc<&'a dyn PieceMove>> = Vec::new();
                if self.mov.is_some() { v.push(Rc::new(self.borrow().mov.as_ref().unwrap())); }
                else  {
                    for x in &self.borrow().strike.as_ref().unwrap().vec {
                        v.push(Rc::new(x))
                    }
                }
                v
            },
            ind: 0,
        }
    }
}



#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(TS)]
#[ts(export)]
pub struct MoveList {
    pub list: Vec<MoveItem>,
    pub current_chain: Strike,
}


impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            list: Vec::new(),
            current_chain: Strike { vec: Vec::new(), took_pieces: Vec::new(), king_move: false },
        }
    }
}


