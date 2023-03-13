use std::cell::RefCell;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;
use crate::position::Position;
use crate::PositionHistory::{FinishType, PositionAndMove, PositionHistory};
use rand::{Rng};
use schemars::_private::NoSerialize;
use crate::color::Color;

#[derive(Debug)]
pub struct Node {
    W: i64,
    N: i64,
    passed_completely: bool,
    pos_mov: Rc<RefCell<PositionAndMove>>,
    childs: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(pos_mov: PositionAndMove) -> Node {
        Node {
            W: 0,
            N: 0,
            passed_completely: false,
            pos_mov: Rc::new(RefCell::new(pos_mov)),
            childs: vec![],
        }
    }
    pub fn expand(&mut self) {
        if self.childs.len() > 0 { return; }
        let mut base_p = self.pos_mov.borrow().pos.clone();
        let move_list = base_p.get_move_list_cached();
        for mov in &move_list.as_ref().as_ref().unwrap().list {
            self.childs.push(Rc::new(
                RefCell::new(Node::new(base_p.make_move_and_get_position(mov)))));
            base_p.unmake_move(mov);
        }
    }
}

#[derive(Debug)]
pub struct McTree {
    root: Rc<RefCell<Node>>,
    history: Rc<RefCell<PositionHistory>>,
}

impl McTree {
    pub fn new(pos: Position, history: Rc<RefCell<PositionHistory>>) -> McTree {
        McTree {
            root: Rc::new(RefCell::new(Node {
                W: 0,
                N: 0,
                passed_completely: false,
                pos_mov: Rc::new(RefCell::new(PositionAndMove::from_pos(pos))),
                childs: vec![],
            })),
            history,
        }
    }

    pub fn new_from_node(root: Rc<RefCell<Node>>, history: Rc<RefCell<PositionHistory>>) -> McTree {
        McTree {
            root,
            history,
        }
    }

    pub fn search(&mut self, max_passes: i32) -> Option<Rc<RefCell<Node>>> {
        let mut track: Vec<Rc<RefCell<Node>>> = vec![];
        let hist_len = self.history.borrow().len();
        pub fn back_propagation (mut res: i64, track: &mut Vec<Rc<RefCell<Node>>>,
                                 history: &Rc<RefCell<PositionHistory>>, hist_len: usize) {
            if res != 0 {
                for node in track.iter().rev() {
                    node.borrow_mut().W += res;
                    res = -res;
                }
            }
            history.borrow_mut().cut_to(hist_len);
            *track = vec![];
        }
        let mut pass = 0;
        while pass < max_passes && !self.root.borrow().passed_completely {
            let mut node = self.root.clone();
            loop {
                node.borrow_mut().N += 1;
                self.history.borrow_mut().push_rc(node.borrow().pos_mov.clone());
                track.push(node.clone());
                // if finish achieved
                let finish = self.history.borrow_mut().finish_check();
                if let Some(finish) = finish {
                    node.borrow_mut().passed_completely = true;
                    back_propagation({
                        let fr = if finish == FinishType::WhiteWin { 1 } else if finish == FinishType::BlackWin { -1 } else { 0 };
                        let sing = if node.borrow().pos_mov.borrow().pos.next_move.unwrap() == Color::White { 1 } else { -1 };
                        fr * sing
                    }, &mut track, &self.history, hist_len);
                    break;
                }
                node.borrow_mut().expand();
                let u = |child: &Node|
                    1.4 * f64::sqrt(f64::ln(node.borrow().N as f64) / (child.N as f64 + 1.0));
                let u_max = |node: &Node| node.W as f64 / (node.N as f64 + 1.0) + u(node);
                let mut childs = vec![];
                for child in &node.borrow().childs {
                    if !child.borrow().passed_completely { childs.push(child.clone()); }
                }
                if childs.len() > 0 {
                    node = {
                        if childs.iter().all(|x| x.borrow().N == 0) {
                            childs[rand::thread_rng().gen_range(0..childs.len())].clone()
                        } else {
                            childs.iter().max_by(|a, b|
                                if u_max(&*a.borrow()) < u_max(&*b.borrow()) { Ordering::Less } else { Ordering::Greater }).unwrap().clone()
                        }
                    };
                } else {
                    node.borrow_mut().passed_completely = true;
                    back_propagation(track.pop().unwrap().borrow().W, &mut track, &self.history, hist_len);
                    break;
                }
            }
            pass += 1;
            print!("{}", pass);
        }
        if self.root.borrow().childs.len() > 0 {
            Some(self.root.borrow().childs
                .iter().max_by(|x,y|x.borrow().W.cmp(&y.borrow().W)).unwrap().clone())
        } else {
            None
        }
    }
}