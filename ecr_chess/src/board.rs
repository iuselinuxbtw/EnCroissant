use std::rc::Rc;

use crate::pieces::BoardPiece;

#[derive(Debug, Clone)]
pub struct Board {
    /// The representation of the board. A board consists of 8x8 squares. The first array is for the
    /// x, the second for the y coordinate. Since the board has 8 squares on each axis, an index of
    /// `0` to `7` is possible. Contains an `Option<BoardPiece>` since a square can be empty, which
    /// means that squares with `None` as value will be empty.
    board: [[Option<Rc<BoardPiece>>; 8]; 8],
    pieces: Vec<Rc<BoardPiece>>,
}