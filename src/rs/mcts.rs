use std::cell::RefCell;
use std::rc::Rc;
use crate::position::Position;
use crate::PositionHistory::{PositionAndMove, PositionHistory};

pub struct Node {
    W: i64,
    N: i64,
    pos_mov: RefCell<PositionAndMove>,
    childs: Vec<Node>,
}

impl Node {
    pub fn new(pos_mov: PositionAndMove) -> Node {
        Node {
            W: 0,
            N: 0,
            pos_mov: RefCell::new(pos_mov),
            childs: vec![],
        }
    }
    pub fn expand(&mut self) {
        let mut base_p = self.pos_mov.borrow().pos.clone();
        let move_list = base_p.get_move_list_cached();
        for mov in &move_list.as_ref().as_ref().unwrap().list {
            self.childs.push(Node::new(base_p.make_move_and_get_position(mov)));
            base_p.unmake_move(mov);
        }
    }
}

pub struct McTree  {
    root: Node,
    history: Rc<RefCell<PositionHistory>>,
}

impl McTree {
    pub fn new(pos: Position, history: Rc<RefCell<PositionHistory>>) -> McTree {
        McTree {
            root: Node {
                W: 0,
                N: 0,
                pos_mov: RefCell::new(PositionAndMove::from_pos(pos)),
                childs: vec![],
            },
            history,
        }
    }

    pub fn new_from_node(root: Node, history: Rc<RefCell<PositionHistory>>) -> McTree {
        McTree {
            root,
            history,
        }
    }

    pub fn search(&mut self) {
        self.history.borrow_mut().push(PositionAndMove::from_pos(self.root.pos_mov.borrow().pos.clone()));
    }
}