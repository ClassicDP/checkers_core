pub type BoardPos = usize;
pub struct StraightStrike {
    pub(crate) v: Vec<BoardPos>,
    pub(crate) take: BoardPos,
    pub(crate) to: BoardPos
}