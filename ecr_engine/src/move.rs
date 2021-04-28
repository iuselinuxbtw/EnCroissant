use crate::coordinate::Coordinate;
use crate::pieces::move_gen::BasicMove;
use crate::pieces::PieceType;

/// The type of a move. Can contain various information about
#[derive(Debug, PartialEq, Clone)]
pub enum MoveType {
    Move {
        from: Coordinate,
        to: Coordinate,
    },
    Capture {
        from: Coordinate,
        to: Coordinate,
        capture_at: Coordinate,
        en_passant: bool,
    },
    Castle {
        king_from: Coordinate,
        queen_side: bool,
    },
}

/// Represents the possible Moves of a Piece on the board with the starting coordinate of that
/// piece.
#[derive(Debug, PartialEq, Clone)]
pub struct Moves {
    pub from: Coordinate,
    pub basic_move: Vec<BasicMove>,
}

/// Represents a move. Can be used to modify the positions of pieces on the board. Does not do any
/// validity detection and just holds the move that should be done.
#[derive(Debug, PartialEq, Clone)]
pub struct Move {
    pub move_type: MoveType,
    pub promotion: Option<PieceType>,
    pub draw_offer: bool,
    pub check: bool,
    pub check_mate: bool,
}
