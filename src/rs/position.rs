use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::mem::swap;
use std::rc::Rc;
use rand::{Rng, thread_rng};

use serde::{Deserialize, Serialize};
use crate::position_environment::PositionEnvironment;
use crate::vector::Vector;
use crate::moves::{BoardPos, PieceMove, QuietMove, StraightStrike};
use crate::moves_list::{MoveItem, MoveList};
use crate::color::Color;
use crate::piece::Piece;
use ts_rs::*;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::game::BestPos;
use crate::PositionHistory::PositionAndMove;
use crate::random;


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[derive(TS)]
#[ts(export)]
pub struct PieceCount {
    pub simple: u32,
    pub king: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[derive(TS)]
#[ts(export)]
pub struct PosState {
    pub(crate) black: PieceCount,
    pub(crate) white: PieceCount,
    pub(crate) kings_start_at: Option<usize>,
    pub(crate) kings_only_move_start_at: Option<usize>,
    pub(crate) triangle_start_at: Option<usize>,
    pub(crate) power_equal_start_at: Option<usize>,
    pub(crate) main_road_start_at: Option<usize>,
    pub(crate) repeats: u8,
}

impl PartialEq for PosState {
    fn eq(&self, other: &Self) -> bool {
        self.black == other.black && self.white == other.white
    }
}

impl Eq for PosState {}

impl PartialOrd<Self> for PosState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PosState {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.evaluate(), &other.evaluate())
    }
}

impl PosState {
    pub fn get_count(&mut self, color: Color) -> &mut PieceCount {
        if color == Color::Black { &mut self.black } else { &mut self.white }
    }
    pub fn get_total(&self) -> i32 {
        (self.black.king + self.black.simple + self.white.king + self.white.simple) as i32
    }
    pub fn get_total_color(&mut self, color: Color) -> i32 {
        let cnt = self.get_count(color);
        (cnt.king + cnt.simple) as i32
    }

    pub fn evaluate(&self) -> i32 {
        self.white.simple as i32 * 1000 + self.white.king as i32 * 3000
            - self.black.simple as i32 * 1000 - self.black.king as i32 * 3000
    }
}


#[derive(Serialize, Debug, Clone)]
#[derive(TS)]
#[ts(export)]
pub struct Position {
    pub cells: Vec<Option<Piece>>,
    pub state: PosState,
    pub next_move: Option<Color>,
    move_list: Rc<Option<MoveList>>,
    #[serde(skip_serializing)]
    pub eval: Option<i32>,
    #[serde(skip_serializing)]
    pub environment: Rc<PositionEnvironment>,
    #[serde(skip_serializing)]
    pub took_pieces: Vec<Option<Piece>>,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.cells.iter().enumerate().all(|(i, x)| Some(&other.cells[i]) == Some(&x))
            && self.next_move == other.next_move
    }
}

impl Position {
    pub fn new(environment: Rc<PositionEnvironment>) -> Position {
        let mut pos = Position {
            state: PosState {
                black: { PieceCount { king: 0, simple: 0 } },
                white: { PieceCount { king: 0, simple: 0 } },
                kings_start_at: None,
                kings_only_move_start_at: None,
                triangle_start_at: None,
                power_equal_start_at: None,
                main_road_start_at: None,
                repeats: 0,
            },
            cells: Vec::new(),
            environment,
            next_move: None,
            move_list: Rc::new(None),
            eval: None,
            took_pieces: vec![],
        };
        pos.cells = Vec::new();
        let size = pos.environment.size;
        pos.cells.resize((size * size / 2) as usize, None);
        pos
    }

    pub fn print_pos(&self) {
        let pieces: Vec<_> = self.cells.iter().filter(|x| x.is_some()).collect();
        let pieces: Vec<_> = pieces.iter().map(|x| {
            let y = x.as_ref().unwrap();
            let z = Piece { pos: self.environment.pack_to_board[y.pos], color: y.color, is_king: y.is_king, stricken: y.stricken };
            z
        }).collect();
        print!("{:?}\n", pieces);
        io::stdout().flush().unwrap();
    }

    pub fn get_move_list_cached(&mut self) -> Rc<Option<MoveList>> {
        if self.move_list.is_none() {
            let move_li = self.get_move_list(false);
            self.move_list = Rc::new(Option::from(move_li));
        }
        self.move_list.clone()
    }

    fn state_change(&mut self, piece: &Piece, sign: i32) {
        if piece.is_king {
            self.state.get_count(piece.color).king = (self.state.get_count(piece.color).king as i32 + sign) as u32;
        } else {
            self.state.get_count(piece.color).simple = (self.state.get_count(piece.color).simple as i32 + sign) as u32;
        }
    }

    fn state_change_by_king_color(&mut self, color: Color, sign: i32) {
        self.state.get_count(color).king = (self.state.get_count(color).king as i32 + sign) as u32;
        self.state.get_count(color).simple = (self.state.get_count(color).simple as i32 - sign) as u32;
    }

    pub fn insert_piece(&mut self, piece: Piece) {
        let pos = piece.pos as usize;
        self.state_change(&piece, 1);
        self.cells[pos] = Some(piece);
        self.move_list = Rc::new(None);
        self.eval = None;
    }

    pub fn remove_piece(&mut self, pos: BoardPos) -> bool {
        if let Some(piece) = self.cells[pos].clone() {
            self.state_change(&piece, -1);
            self.cells[pos] = None;
            self.move_list = Rc::new(None);
            self.eval = None;
            return true;
        }
        false
    }


    fn make_strike_or_move(&mut self, mov: &dyn PieceMove) {
        self.swap(mov.from(), mov.to());
        if let Some(take) = mov.take() {
            if let Some(ref mut piece) = self.cells[take] {
                piece.stricken = true;
            }
        }
        if mov.is_king() {
            let color = {
                let piece = self.cells[mov.to()].as_mut().unwrap();
                piece.is_king = true;
                piece.color
            };
            self.state_change_by_king_color(color, 1);
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
            let color = {
                let piece = self.cells[mov.from()].as_mut().unwrap();
                piece.is_king = false;
                piece.color
            };
            self.state_change_by_king_color(color, -1);
        }
    }

    fn get_piece_by_v(&self, v: &Rc<Vec<BoardPos>>, i: usize) -> &Option<Piece> {
        &self.cells[v[i]]
    }
    pub fn swap(&mut self, i: BoardPos, j: BoardPos) {
        self.cells.swap(i as usize, j as usize);
        let set_pos = |cell: &mut Option<Piece>, pos: BoardPos| {
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
                    if self.get_piece_by_v(v, i).is_none() && candidate.color != piece.color
                        && !candidate.stricken {
                        let strike = StraightStrike {
                            v: {
                                let mut i_next = i;
                                let mut ve = Vec::new();
                                while i_next < search_steps_top && self.get_piece_by_v(&v, i_next).is_none() {
                                    ve.push(v[i_next]);
                                    i_next += 1;
                                }
                                ve
                            },
                            from: v[0],
                            to: v[i],
                            take: v[i - 1],
                            king_move: self.environment.is_king_move_for(piece, v[i]),
                        };
                        return Some(strike);
                    } else { break; }
                }
                i += 1;
            }
        }
        None
    }

    fn get_vectors(&self, piece: &Piece, ban_directions: &Vec<i8>, for_strike: bool) -> Vec<Rc<Vector<BoardPos>>> {
        let d2_4 = {
            if piece.is_king || for_strike { vec![0, 1, 2, 3] } else if piece.color == Color::White {
                vec![0, 1]
            } else {
                vec![2, 3]
            }
        };
        let vectors = self.environment.get_vectors(piece.pos);
        let mut res = Vec::new();
        for v in vectors {
            if d2_4.contains(&v.direction) && !ban_directions.contains(&v.direction) { res.push(v.clone()); }
        }
        res
    }

    pub fn get_piece_in_pos(&self, pos: BoardPos) -> &Piece {
        if let Some(x) = &self.cells[pos] {
            x
        } else {
            panic!("error in get_piece_of_move_item")
        }
    }

    pub fn get_quiet_move_list(
        &mut self,
        pos: BoardPos,
        move_list: &mut MoveList,
    ) -> bool {
        if let Some(piece) = &self.cells[pos] {
            let vectors: Vec<_> = self.get_vectors(piece, &vec![], false);
            for vector in vectors {
                for point in {
                    if piece.is_king { &(vector.points)[1..] } else { &(vector.points)[1..2] }
                } {
                    if self.cells[*point].is_some() { break; }
                    move_list.list.push(
                        MoveItem {
                            mov: Some(QuietMove {
                                from: pos,
                                to: *point,
                                king_move: self.environment.is_king_move_for(piece, *point),
                            }),
                            strike: None,
                        })
                }
            }
            return move_list.list.len() > 0;
        }
        false
    }

    pub fn evaluate(&mut self) -> i32 {
        if self.eval.is_some() { return self.eval.unwrap(); }
        // white advantage if positive signature of evaluate, black - negative
        let mut eval: i32 =
            if self.get_move_list_cached().as_ref().as_ref().unwrap().list.len() == 0 {
                if self.next_move.is_some() && self.next_move.unwrap() == Color::White {
                    i32::MIN / 2
                } else { i32::MAX / 2 }
            } else { 0 };

        for cell in &self.cells {
            if let Some(ref piece) = cell {
                let v = self.get_vectors(piece, &vec![], false);
                let s: i32 = if piece.color == Color::White { 1 } else { -1 };
                v.iter().for_each(|v|
                    for point in &(v.points)[1..] {
                        if self.cells[*point].is_some() { break; }
                        eval += s;
                    })
            }
        }
        eval += self.state.evaluate() + thread_rng().gen_range(-2..2);
        self.eval = Some(eval);
        eval
    }

    pub fn get_strike_list(
        &mut self,
        pos: BoardPos,
        move_list: &mut MoveList,
        ban_directions: &Vec<i8>,
        for_front: bool,
    ) -> bool {
        let mut success_call = false;
        if let Some(piece) = &self.cells[pos] {
            let vectors: Vec<_> = self.get_vectors(piece, ban_directions, true);
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
                        if self.get_strike_list(pos, move_list, &ban_directions, for_front) {
                            recurrent_chain = true;
                        }
                        move_list.current_chain.vec.pop();
                        if strike_move.king_move { move_list.current_chain.king_move = false; }
                        self.unmake_strike_or_move(&strike_move);
                        if !for_front && ban_directions.len() < 2 {
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
                            move_list.list.push(MoveItem { strike: Some(chain), mov: None });
                        }
                    }
                }
            }
        }
        success_call
    }

    pub fn make_move(&mut self, move_item: &MoveItem) {
        if let Some(ref mov) = move_item.mov {
            self.make_strike_or_move(mov);
        } else if let Some(ref strike) = move_item.strike {
            self.took_pieces = vec![None; strike.vec.len()];
            for (i, straight_strike) in strike.vec.iter().enumerate() {
                swap(&mut self.took_pieces[i], &mut self.cells[straight_strike.take]);
                self.state_change(self.took_pieces[i].clone().as_ref().unwrap(), -1);
            };

            let ref mut mov = QuietMove {
                from: strike.vec[0].from,
                to: strike.vec[strike.vec.len() - 1].to,
                king_move: strike.king_move,
            };
            self.make_strike_or_move(mov);
        }
        if self.next_move.is_some() { self.next_move = Some(!self.next_move.unwrap()) }
        self.move_list = Rc::new(None);
        self.eval = None;
    }


    pub fn unmake_move(&mut self, move_item: &MoveItem) {
        if let Some(ref mov) = move_item.mov {
            self.unmake_strike_or_move(mov);
        } else if let Some(ref strike) = move_item.strike {
            for (i, straight_strike) in strike.vec.iter().enumerate() {
                self.state_change(self.took_pieces[i].clone().as_ref().unwrap(), 1);
                swap(&mut self.took_pieces[i], &mut self.cells[straight_strike.take]);
            };
            let ref mut mov = QuietMove {
                from: strike.vec[0].from,
                to: strike.vec[strike.vec.len() - 1].to,
                king_move: strike.king_move,
            };
            self.unmake_strike_or_move(mov);
        }
        if self.next_move.is_some() { self.next_move = Some(!self.next_move.unwrap()) }
    }

    pub fn make_move_and_get_position(&mut self, move_item: &MoveItem) -> PositionAndMove {
        self.make_move(move_item);
        PositionAndMove::from(self.clone(), move_item.clone())
    }

    pub fn get_move_list(&mut self, for_front: bool) -> MoveList {
        let color = self.next_move.unwrap_or_else(|| panic!("Color of next move undefined!"));
        let pieces_pos: Vec<_> = self.cells.iter()
            .filter(|piece| if let Some(piece) = piece { piece.color == color } else { false })
            .map(|piece| if let Some(piece) =
                piece { piece.pos } else { panic!("Position problem in get_move_list"); })
            .collect();
        let mut move_list = MoveList::new();
        for pos in &pieces_pos {
            self.get_strike_list(*pos, &mut move_list, &vec![], for_front);
        }
        if move_list.list.is_empty() {
            for pos in pieces_pos {
                self.get_quiet_move_list(pos, &mut move_list);
            }
        }
        move_list
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::game::Game;
    use crate::piece::Piece;

    #[test]
    fn positions_eq() {
        let mut g1 = Game::new(8);
        let mut g2 = Game::new(8);
        g1.insert_piece(Piece::new(0, Color::White, true));
        g2.insert_piece(Piece::new(0, Color::White, true));
        assert_eq!(g1.current_position, g2.current_position);
        g1.insert_piece(Piece::new(1, Color::White, true));
        g2.insert_piece(Piece::new(1, Color::White, true));
        assert_eq!(g1.current_position, g2.current_position);
        g1.insert_piece(Piece::new(3, Color::White, true));
        g2.insert_piece(Piece::new(3, Color::White, false));
        assert_ne!(g1.current_position, g2.current_position);
        g1.remove_piece(3);
        assert_ne!(g1.current_position, g2.current_position);
        g2.remove_piece(3);
        assert_eq!(g1.current_position, g2.current_position);
        g1.insert_piece(Piece::new(3, Color::White, true));
        g2.insert_piece(Piece::new(1, Color::White, true));
        assert_ne!(g1.current_position, g2.current_position);
        g1.remove_piece(3);
        g2.remove_piece(3);
        assert_eq!(g1.current_position, g2.current_position);
    }
}