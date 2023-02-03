use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::game::Game;
use crate::vector::Vector;
use crate::moves::{BoardPos, QuietMove, PieceMove, StraightStrike};
use crate::moves_list::{MoveItem, MoveList};
use crate::{Color, Piece};
use ts_rs::TS;
use crate::moves_list::MoveItem::{Move, StrikeChain};

pub type Cell = Option<Piece>;

#[derive(Clone)]
pub struct PositionListItem {
    pub position: Position,
    pub move_item: MoveItem,
}

impl PartialEq for PositionListItem {
    fn eq(&self, other: &Self) -> bool {
        self.position.cells.iter().enumerate().all(|it| other.position.cells[it.0] == *it.1)
    }
}


#[derive(Deserialize, Serialize, Debug, TS, Clone)]
pub struct Position {
    pub cells: Vec<Cell>,
    pub game: Rc<Game>,
}

impl Position {
    pub fn new(game: Rc<Game>) -> Position {
        let mut pos = Position {
            cells: Vec::new(),
            game,
        };
        pos.cells = Vec::new();
        let size = pos.game.size;
        pos.cells.resize((size * size / 2) as usize, Cell::None);
        pos
    }
    pub fn inset_piece(&mut self, piece: Piece) {
        let pos = piece.pos as usize;
        self.cells[pos] = Some(piece);
    }

    fn make_strike_or_move(&mut self, mov: &mut dyn PieceMove) {
        self.swap(mov.from(), mov.to());
        if let Some(take) = mov.take() {
            if let Some(ref mut piece) = self.cells[take] {
                piece.stricken = true;
            }
        }
        if let Some(ref mut piece) = self.cells[mov.to()] {
            if !piece.is_king {
                if self.game.is_king_row(&piece) {
                    piece.is_king = true;
                    mov.set_as_king();
                }
            }
        }
    }

    fn unmake_strike_or_move(&mut self, mov: &dyn PieceMove) {
        self.swap(mov.from(), mov.to());
        if let Some(take) = mov.take() {
            if let Some(ref mut piece) = self.cells[take] {
                piece.stricken = false;
            }
        }
        if mov.is_king() {
            if let Some(ref mut piece) = self.cells[mov.from()] {
                piece.is_king = false;
            }
        }
    }

    fn get_piece_by_v(&self, v: &Rc<Vec<BoardPos>>, i: usize) -> &Cell {
        &self.cells[v[i]]
    }
    pub fn swap(&mut self, i: BoardPos, j: BoardPos) {
        self.cells.swap(i as usize, j as usize);
        let set_pos = |cell: &mut std::option::Option<Piece>, pos: BoardPos| {
            if let Some(ref mut piece) = cell {
                piece.pos = pos;
            }
        };
        set_pos(&mut self.cells[i], i);
        set_pos(&mut self.cells[j], j);
    }

    fn straight_strike(&mut self, v: &Rc<Vec<BoardPos>>) -> Option<StraightStrike> {
        if v.len() < 3 {
            return None;
        }
        if let Some(piece) = self.get_piece_by_v(v, 0) {
            let search_steps_top = if piece.is_king { v.len() } else { 3 };
            let mut i: usize = 2;
            while i < search_steps_top {
                if let Some(candidate) = self.get_piece_by_v(v, i - 1) {
                    if self.get_piece_by_v(&v, i).is_none() && candidate.color != piece.color
                        && !candidate.stricken {
                        let strike = StraightStrike {
                            v: {
                                let mut i_next = i;
                                let mut ve = Vec::new();
                                while i_next < search_steps_top && self.get_piece_by_v(&v, i_next).is_none() {
                                    ve.push(v[i_next]);
                                    i_next += 1;
                                }
                                Rc::new(ve)
                            },
                            from: v[0],
                            to: v[i],
                            i_to: 0,
                            take: v[i - 1],
                            king_move: false,
                        };
                        return Some(strike);
                    } else {
                        break;
                    }
                }
                i += 1;
            }
        }
        None
    }

    fn get_vectors(&self, pos: BoardPos, ban_directions: &Vec<i8>) -> Vec<Rc<Vector<BoardPos>>> {
        let d2_4 = match &self.cells[pos] {
            Some(piece) => {
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
            None => vec![0, 1, 2, 3]
        };
        let vectors = self.game.get_vectors(pos);
        let mut res = Vec::new();
        for v in vectors {
            if d2_4.contains(&v.direction) && !ban_directions.contains(&v.direction) { res.push(v.clone()); }
        }
        res
    }

    pub fn get_quiet_move_list(
        &mut self,
        pos: BoardPos,
        move_list: &mut MoveList,
    ) -> bool {
        let vectors: Vec<_> = self.get_vectors(pos, &vec![]);
        for vector in vectors {
            for point in &(vector.points)[1..] {
                if !self.cells[*point].is_none() { break; }
                move_list.list.push(MoveItem::Move(QuietMove { from: pos, to: *point, king_move: false }))
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
        let vectors: Vec<_> = self.get_vectors(pos, ban_directions);
        for v in vectors {
            let points = &v.points;
            let strike = self.straight_strike(points);
            if let Some(straight_strike) = strike {
                success_call = true;
                let mut ban_directions = vec![v.get_ban_direction()];
                let mut recurrent_chain = false;
                let mut strike_move = straight_strike.clone();
                for pos in &straight_strike {
                    strike_move.to = pos;
                    self.make_strike_or_move(&mut strike_move);
                    move_list.current_chain.vec.push(strike_move.clone());
                    if strike_move.king_move { move_list.current_chain.king_move = true; }
                    if self.get_strike_list(pos, move_list, &ban_directions) {
                        recurrent_chain = true;
                    }
                    move_list.current_chain.vec.pop();
                    self.unmake_strike_or_move(&strike_move);
                    if ban_directions.len() < 2 {
                        ban_directions.push(v.direction);
                    }
                }
                if !recurrent_chain {
                    for pos in &straight_strike {
                        let mut strike_move = straight_strike.clone();
                        strike_move.to = pos;
                        let mut chain = move_list.current_chain.clone();
                        if strike_move.king_move { chain.king_move = true; }
                        chain.vec.push(strike_move);
                        move_list.list.push(MoveItem::StrikeChain(chain));
                    }
                }
            }
        }
        success_call
    }

    pub fn make_move(&mut self, move_item: &mut MoveItem) {
        match move_item {
            Move(mov) => {
                self.make_strike_or_move(mov);
            }
            StrikeChain(chain) => {
                let take_pos_list: Vec<BoardPos> = chain.vec.iter().map(|it| it.take).collect();
                if self.cells[take_pos_list[0]].is_some() {
                    take_pos_list.iter().for_each(|pos| {
                        if let Some(ref mut piece) = self.cells[*pos] {
                            chain.took_pieces.push(piece.clone());
                        }
                        self.cells[*pos] = None;
                    });
                    let n = chain.vec.len() - 1;
                    let ref mut mov = QuietMove { from: chain.vec[0].from, to: chain.vec[n].to, king_move: false };
                    self.make_strike_or_move(mov);
                }
            }
        }
    }

    pub fn unmake_move(&mut self, move_item: &MoveItem) {
        match move_item.clone() {
            Move(ref mov) => {
                self.unmake_strike_or_move(mov);
            }
            StrikeChain(chain) => {
                let from = chain.vec[0].from;
                let to = chain.vec[chain.vec.len() - 1].to;
                for piece in chain.took_pieces {
                    let pos = piece.pos;
                    self.cells[pos] = Some(piece);
                }
                let ref mut mov = QuietMove { from, to, king_move: chain.king_move };
                self.unmake_strike_or_move(mov);
            }
        }
    }

    pub fn make_move_and_get_position(&mut self, move_item: &mut MoveItem) -> PositionListItem {
        self.make_move(move_item);
        PositionListItem { position: self.clone(), move_item: move_item.clone() }
    }
}
