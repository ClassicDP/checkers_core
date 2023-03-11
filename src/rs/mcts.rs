use std::cell::RefCell;
use std::rc::Rc;
use crate::position::Position;

struct Node {
    W: i64,
    N: i64,
    position: Position,
    childs: Vec<Node>,
    parent: Option<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new (position: Position, parent: Option<RefCell<Node>> ) -> Node {
        Node {
            W: 0,
            N: 0,
            position,
            parent: if parent.is_none() { None} else { Some(Rc::new(parent.unwrap())) },
            childs: vec![]
        }
    }
    pub fn expand(&mut self) -> bool {
        let list = self.position.get_move_list_cached();
        if list.borrow_mut().list.len() == 0 {return false;}

        true
    }
}