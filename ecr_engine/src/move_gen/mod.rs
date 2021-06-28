use crate::board;
use ecr_shared::coordinate::Coordinate;
use ecr_shared::pieces::{PieceColor, PieceType};

pub mod directions;
pub mod generation;
mod utils;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Capture {
    pub piece_type: PieceType,
    pub target: Coordinate,
}

/// Defines a move in the most basic form.
///
/// Only defines where the move goes and whether or not the move is a capture.
// TODO: Implement pawn promotion as maybe an Option i guess. We would have to make a new type to not always have a None type in the move.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BasicMove {
    pub to: Coordinate,
    pub capture: Option<Capture>,
}

impl BasicMove {
    pub fn get_target_square(&self) -> Coordinate {
        self.to
    }
    pub fn get_capture(&self) -> Option<Capture> {
        self.capture
    }

    /// Returns whether the target square is threatened. Useful for king movement.
    pub fn get_is_threatened(&self, board: &board::Board, team: PieceColor) -> bool {
        let state = board.get_threatened_state(self.get_target_square());
        match team {
            PieceColor::Light => state.threatened_dark > 0,
            PieceColor::Dark => state.threatened_light > 0,
        }
    }
    /// Returns whether the capture is en_passant.
    pub fn get_is_en_passant(&self) -> bool {
        // We can safely unwrap since we've checked that is is_some
        self.capture.is_some() && self.to != self.capture.unwrap().target
    }
    /// Generates a new non-capture move
    pub fn new_move(to: Coordinate) -> BasicMove {
        BasicMove { to, capture: None }
    }
    /// Generates a new capture move
    pub fn new_capture(to: Coordinate, piece_type: PieceType) -> BasicMove {
        BasicMove {
            to,
            capture: Some(Capture {
                piece_type,
                target: to,
            }),
        }
    }

    /// Generates a new en_passant move
    pub fn new_en_passant(to: Coordinate, to_capture: Coordinate) -> BasicMove {
        BasicMove {
            to,
            capture: Some(Capture {
                piece_type: PieceType::Pawn,
                target: to_capture,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CastleMove {
    pub move_type: CastleMoveType,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CastleMoveType {
    LightKingSide,
    LightQueenSide,
    DarkKingSide,
    DarkQueenSide,
}
