use std::rc::Rc;
use crate::{Cell, Figure};
use crate::game::Game;

#[derive(Clone)]
pub struct Position {
    cells: Vec<Cell>,
    game: Rc<Game>,
}


impl Position {
    pub fn new(game: Rc<Game>) -> Position {
        let mut pos = Position { cells: Vec::new(), game };
        pos.cells = Vec::new();
        pos.cells.resize((pos.game.size * pos.game.size / 2) as usize, Cell::None);
        pos
    }
    pub fn inset_fig(&mut self, fig: Rc<Figure>) {
        let pos = fig.pos as usize;
        self.cells[pos] = Cell::Figure(fig);
    }

}
