use crate::pieces::{Piece, PieceColor};

pub struct Board {
    /// The representation of the board. A board consists of 8x8 squares. The first array is for the
    /// x, the second for the y coordinate. Since the board has 8 squares on each axis, an index of
    /// `0` to `7` is possible. The `Piece` can be an `Option` since a square can be empty.
    board: [[Option<(PieceColor, Box<dyn Piece>)>; 8]; 8],
}