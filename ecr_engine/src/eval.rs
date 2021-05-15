//! Contains Functions used for evaluating the current board.

use std::ops::Deref;

use ecr_shared::pieces::PieceColor;

use crate::board;
use crate::board::{Board, ThreatenedState};
use ecr_shared::coordinate::Coordinate;

impl board::Board {
    pub fn eval(&self) -> f32 {
        let piece_value = evaluate_pieces(self);
        let position_value = position_value(self);

        (piece_value as u64+ position_value) as f32
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


///
fn position_value(board: &Board) -> u64 {
    let middle_squares = vec![
        Coordinate { y: 3, x: 3 },
        Coordinate { y: 4, x: 3 },
        Coordinate { y: 3, x: 4 },
        Coordinate { y: 4, x: 4 },
    ];

    let mut other_squares:Vec<Coordinate> = vec![];
    for x in 0..=7{
        for y in 0..=7{
            other_squares.push((x,y).into());
        }
    }
    for square in &middle_squares {
        other_squares.retain(|s| s!=square);
    }
    // For now we calculate the ThreatenedStates
    get_threatened_score(
        get_threatened_states(board, middle_squares.clone()),
        PieceColor::Light,
    ) - get_threatened_score(
        get_threatened_states(board, middle_squares),
        PieceColor::Dark,
    )
    + get_threatened_score(
        get_threatened_states(board, other_squares.clone()),
        PieceColor::Light
    )
    - get_threatened_score(get_threatened_states(board, other_squares),
        PieceColor::Dark
    )
}

fn get_threatened_states(board: &Board, coords: Vec<Coordinate>) -> Vec<ThreatenedState> {
    let mut result = vec![];
    for coord in coords {
        result.push(board.get_threatened_state(coord));
    }
    result
}

/// Gets the threats of a particular team on given squares
fn get_threatened_score(states: Vec<ThreatenedState>, team: PieceColor) -> u64 {
    let mut result:u64 = 0;
    for state in states {
        result += state.get_by_team(team) as u64;
    }
    result
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
