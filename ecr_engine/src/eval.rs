//! Contains Functions used for evaluating the current board.

use std::ops::Deref;

use ecr_shared::pieces::PieceColor;

use crate::board::Board;

pub fn eval(board: &Board) -> f32 {
    let piece_value = evaluate_pieces(board);
    let position_value = position_value(board);

    piece_value as f32 + position_value
}

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
