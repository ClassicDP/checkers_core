use std::collections::{HashMap, HashSet};
use std::ops::{Deref};
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::game::Game;
use crate::vector::Vector;
use crate::Moves::{BoardPos, QuietMove, PieceMove, StraightStrike};
use crate::MovesList::{MoveItem, MoveList};
use crate::{Color, Piece};
use ts_rs::TS;
use crate::HashRcWrap::HashRcWrap;
use crate::MovesList::MoveItem::{Move, StrikeChain};

pub type Cell = Option<HashRcWrap<Piece>>;

#[derive(Deserialize, Serialize, Debug, TS, Clone)]
pub struct Position {
    pub cells: Vec<Cell>,
    pub game: HashRcWrap<Game>,
    #[serde(skip_serializing, skip_deserializing)]
    pub pieces: HashMap<Color, HashSet<HashRcWrap<Piece>>>,
}

// impl Clone for Position {
//     fn clone(&self) -> Self {
//         let mut new_pieces: HashMap<Color, HashSet<HashRcWrap<Piece>>> = HashMap::new();
//         for (col, hash_set) in &self.pieces {
//             let mut new_hashset: HashSet<HashRcWrap<Piece>> = HashSet::new();
//             for x in hash_set {
//                 new_hashset.insert(x.clone());
//             }
//             new_pieces.insert(col.clone(), new_hashset);
//         }
//         Position {
//             cells: self.cells.clone(),
//             game: self.game.clone(),
//             pieces: self.pieces.clone(),
//             took_pieces: self.took_pieces.clone()
//         }
//     }
// }

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
    }

    pub fn make_strike_or_move(&mut self, mov: &mut dyn PieceMove) {
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

    pub fn unmake_strike_or_move(&mut self, mov: &dyn PieceMove) {
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
        self.cells[v[i]].clone()
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
            while i < max_search_steps {
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

    fn get_directions(&self, piece: &Piece) -> Vec<i8> {
        if !piece.is_king {
            if piece.color == Color::White {
                vec![0, 1]
            } else {
                vec![2, 3]
            }
        } else {
            vec![0, 1, 2, 3]
        }
    }
    pub fn get_quiet_move_list(
        &mut self,
        pos: BoardPos,
        move_list: &mut MoveList,
    ) -> bool {
        if let Some(piece) = &self.cells[pos] {
            let directions = self.get_directions(&piece.get_unwrap());
            let vectors: Vec<HashRcWrap<Vector<BoardPos>>> = (&self.game)
                .get_unwrap()
                .get_vectors(pos)
                .into_iter()
                .filter(|v| {
                    let v_direction = &v.get_unwrap().direction;
                    directions.contains(v_direction)
                })
                .collect();
            for vector in vectors {
                for point in &(*vector.get_unwrap().points)[1..] {
                    if !self.cells[*point].is_none() { break; }
                    move_list.list.push(MoveItem::Move(QuietMove { from: pos, to: *point, king_move: false }))
                }
            }
        }
        move_list.list.len() > 0
    }
    pub fn get_strike_list(
        &mut self,
        pos: BoardPos,
        move_list: &mut MoveList,
        ban_directions: &Vec<i8>,
    ) -> bool {
        let mut success_call = false;
        if let Some(piece) = &self.cells[pos] {
            let directions = self.get_directions(&piece.get_unwrap());
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
                        self.make_strike_or_move(&mut strike_move);
                        move_list.current_chain.vec.push(strike_move.clone());
                        if self.get_strike_list(pos, move_list, &ban_directions) {
                            recurrent_chain = true;
                        }
                        move_list.current_chain.vec.pop();
                        self.unmake_strike_or_move(&strike_move);
                        if ban_directions.len() < 2 {
                            ban_directions.push(v.get_unwrap().direction);
                        }
                    }
                    if !recurrent_chain {
                        for pos in &strike {
                            let mut strike_move = strike.clone();
                            strike_move.to = pos;
                            let mut chain = move_list.current_chain.clone();
                            chain.vec.push(strike_move);
                            move_list.list.push(MoveItem::StrikeChain(chain));
                        }
                    }
                }
            }
        }
        success_call
    }

    pub fn make_move (&mut self, move_item: &mut MoveItem) {
        match move_item {
            Move(mov) => {
                self.make_strike_or_move(mov);
            },
            StrikeChain(chain) => {
                let take_pos_list: Vec<BoardPos> = chain.vec.iter().map(|it| it.take).collect();
                if let Some(piece) = &self.cells[take_pos_list[0]] {
                    let took_color = piece.get_unwrap().color;
                    let set = self.pieces.get_mut(&took_color).unwrap();
                    take_pos_list.iter().for_each(|pos| {
                        if let Some(piece) = &self.cells[*pos] {
                            set.remove(piece);
                            chain.took_pieces.insert(piece.clone());
                        }
                        self.cells[*pos] = None;
                    });
                    let n = chain.vec.len() -1;
                    let ref mut mov = QuietMove{from: chain.vec[0].from, to: chain.vec[n].to, king_move: false};
                    self.make_strike_or_move(mov);
                }

            }
        }
    }

    pub fn unmake_move (&mut self, move_item: &mut MoveItem) {
        match move_item {
            Move(mov) => {
                self.unmake_strike_or_move(mov);
            },
            StrikeChain(chain) => {
                chain.took_pieces.iter().for_each(|piece|{
                   self.cells[piece.get_unwrap().pos] = Some(piece.clone());
                    self.pieces.get_mut(&piece.get_unwrap().color).unwrap().insert(piece.clone());
                });
                chain.took_pieces.clear();
            }
        }
    }

}
