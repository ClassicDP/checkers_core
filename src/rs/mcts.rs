use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
use js_sys::Math::{log, log2, sqrt};
use crate::position::Position;
use crate::PositionHistory::{FinishType, PositionAndMove, PositionHistory};
use rand::{Rng};

pub struct Node {
    W: i64,
    N: i64,
    pos_mov: Rc<RefCell<PositionAndMove>>,
    childs: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(pos_mov: PositionAndMove) -> Node {
        Node {
            W: 0,
            N: 0,
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

    pub fn search(&mut self) {
        let mut track: Vec<Rc<RefCell<Node>>> = vec![];
        let mut node = self.root.clone();
        track.push(node.clone());
        node.borrow_mut().expand();
        let u = sqrt(log(node.borrow().N as f64) / (node.borrow().N as f64 + 1.0));
        let u_max = |node: &Node| node.W as f64 / (node.N as f64 + 1.0) + u;
        if node.borrow().childs.iter().all(|x| x.borrow().N == 0) {
            node.borrow_mut().childs.sort_by(|_a, _b| if rand::thread_rng().gen_range(0..100) < 50
            { Ordering::Less } else { Ordering::Greater });
        } else {
            node.borrow_mut().childs.sort_by(|a, b|
                if u_max(&*a.borrow()) < u_max(&*b.borrow()) { Ordering::Less } else { Ordering::Greater });
        }
        node = {
            let _x = node.borrow().childs.last().unwrap().clone();
            _x
        };
        self.history.borrow_mut().push_rc(node.borrow().pos_mov.clone());
        self.history.borrow_mut()
            .push(PositionAndMove::from_pos(self.root.borrow().pos_mov.borrow().pos.clone()));
        if let Some(finish)= self.history.borrow_mut().finish_check() {
            let result = if finish==FinishType::WhiteWin {1} else if finish == FinishType::BlackWin {-1 } else { 0 };
        }
    }
}