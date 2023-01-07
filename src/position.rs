use std::cell::RefCell;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

use crate::{Cell, Figure, MutFigure};
use crate::Cell::CellFigure;
use crate::game::Game;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Position {
    cells: Vec<Cell>,
    game: RefCell<Game>,
}


impl Position {
    pub fn new(game: RefCell<Game>) -> Position {
        let mut pos = Position { cells: Vec::new(), game };
        pos.cells = Vec::new();
        pos.cells.resize((pos.game.borrow_mut().size *
            pos.game.borrow_mut().size / 2) as usize, Cell::None);
        pos
    }
    pub fn inset_fig(&mut self, fig: RefCell<Figure>) {
        let pos = fig.borrow_mut().pos as usize;
        self.cells[pos] = CellFigure(fig);
    }

    pub fn swap(&mut self, i: i16, j: i16) {
        self.cells.swap(i as usize, j as usize);
        self.cells[i as usize].set_pos(i);
        self.cells[j as usize].set_pos(j);
    }
}
