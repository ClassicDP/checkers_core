pub type BoardPos = usize;
pub struct StraightStrike {
    pub(crate) v: Vec<BoardPos>,
    pub(crate) from: BoardPos,
    pub(crate) take: BoardPos,
    pub(crate) to: BoardPos
}

pub struct Move {
    pub(crate) v: Vec<BoardPos>,
    pub(crate) from: BoardPos,
    pub(crate) to: BoardPos
}

impl PieceMove for Move {
    fn from(&self) -> BoardPos {
        self.from
    }

    fn to(&self) -> BoardPos {
        self.to
    }

    fn take(&self) -> Option<BoardPos> {
        None
    }
}


impl PieceMove for StraightStrike {
    fn from(&self) -> BoardPos {
        self.from
    }

    fn to(&self) -> BoardPos {
        self.to
    }

    fn take(&self) -> Option<BoardPos> {
        Some(self.take)
    }
}

pub trait PieceMove {
    fn from(&self)-> BoardPos;
    fn to(&self)-> BoardPos;
    fn take(&self)-> Option<BoardPos>;
}
