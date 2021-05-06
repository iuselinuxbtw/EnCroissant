//! Contains Functions used for evaluating the current board.

use std::ops::Deref;

use ecr_shared::pieces::PieceColor;

use crate::board;
use crate::board::Board;

impl board::Board {
    pub fn eval(&self) -> f32 {
        let piece_value = evaluate_pieces(self);
        let position_value = position_value(self);

        piece_value as f32 + position_value
    }
}

/// MiniMax Evaluation
fn evaluate_pieces(board: &Board) -> u8 {
    let light_pieces = board.get_team_pieces(PieceColor::Light);
    let dark_pieces = board.get_team_pieces(PieceColor::Dark);
    let mut value_light = 0;
    let mut value_dark = 0;
    for piece in light_pieces {
        value_light = piece.borrow().deref().get_piece().get_value();
    }
    for piece in dark_pieces {
        value_dark = piece.borrow().deref().get_piece().get_value();
    }

    value_light - value_dark
}

fn position_value(board: &Board) -> f32 {
    //TODO
    0.0
}

mod tests {
    use super::*;

    #[test]
    fn test_evaluate_pieces() {
        let default_board = Board::default();
        assert_eq!(0, evaluate_pieces(&default_board));
        let empty_board = Board::empty();
        assert_eq!(0, evaluate_pieces(&empty_board));
    }
}
