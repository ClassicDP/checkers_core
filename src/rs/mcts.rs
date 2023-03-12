use std::cell::RefCell;
use crate::position::Position;
use crate::PositionHistory::{PositionAndMove, PositionHistory};

struct Node {
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
        if let Some(ref move_list) = base_p.get_move_list_cached().as_ref() {
            for mov in &move_list.list {
                self.childs.push(Node::new(base_p.make_move_and_get_position(mov)));
                base_p.unmake_move(mov);
            }
        } else {
            panic!("Move list empty")
        }

    }
}

struct Tree {
    root: Node,
    history: PositionHistory,
}

impl Tree {
    pub fn new(pos: Position, history: PositionHistory) -> Tree {
        Tree {
            root: Node {
                W: 0,
                N: 0,
                pos_mov: RefCell::new(PositionAndMove::from_pos(pos)),
                childs: vec![],
            },
            history,
        }
    }

    pub fn new_from_node (root: Node, history: PositionHistory) -> Tree {
        Tree {
            root,
            history
        }
    }
}