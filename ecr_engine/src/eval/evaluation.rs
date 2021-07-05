//! Contains Functions used for evaluating the current board.

use std::ops::Deref;

use ecr_shared::pieces::PieceColor;

use crate::board;
use crate::board::Board;
use crate::pieces::BoardPiece;
use crate::utils::get_all_squares;
use ecr_shared::coordinate::Coordinate;
use std::cell::RefCell;
use crate::eval::utils::{get_middle_squares, get_threatened_score, get_threatened_states};

impl board::Board {
    pub fn eval(&self) -> f32 {
        let piece_value = evaluate_pieces(self);
        let position_value = position_value(self);

        (piece_value + position_value) as f32
    }
}

/// MiniMax Evaluation of the pieces on the board.
fn evaluate_pieces(board: &Board) -> i32 {
    // Get pieces of each team
    let light_pieces = board.get_team_pieces(PieceColor::Light);
    let dark_pieces = board.get_team_pieces(PieceColor::Dark);

    // Calculate the values of the pieces of each team
    let value_light: i32 = eval_pieces(light_pieces);
    let value_dark: i32 = eval_pieces(dark_pieces);

    value_light - value_dark
}

/// Returns the combined value of the given array of pieces
fn eval_pieces(pieces: Vec<&RefCell<BoardPiece>>) -> i32 {
    let mut result = 0;
    for piece in pieces {
        result += piece.borrow().deref().get_piece().get_value() as i32;
    }
    result
}

/// Used to evaluate a position. Right now this is only using the ThreatenedStates of the middle squares
fn position_value(board: &Board) -> i32 {
    // For now we calculate the ThreatenedStates
    let middle_squares_score = middle_squares_score(board);
    let all_squares_score = all_squares_score(board);
    middle_squares_score + all_squares_score
}

/// Returns the Score of who has more Threats in the four middle squares following the MiniMax
/// Principle
fn middle_squares_score(board: &Board) -> i32 {
    let middle_squares = get_middle_squares();
    let light_score = get_threatened_score(
        get_threatened_states(board, middle_squares.clone()),
        PieceColor::Light,
    );
    let dark_score = get_threatened_score(
        get_threatened_states(board, middle_squares),
        PieceColor::Dark,
    );
    light_score as i32 - dark_score as i32
}

/// Returns the Score of who has more Threats all squares following the MiniMax Principle
fn all_squares_score(board: &Board) -> i32 {
    let all_squares: Vec<Coordinate> = get_all_squares();
    let light_score = get_threatened_score(
        get_threatened_states(board, all_squares.clone()),
        PieceColor::Light,
    );
    let dark_score =
        get_threatened_score(get_threatened_states(board, all_squares), PieceColor::Dark);
    light_score as i32 - dark_score as i32
}



mod tests {
    use std::str::FromStr;

    use super::*;
    use ecr_formats::fen::Fen;
    use ecr_shared::pieces::PieceType;

    #[test]
    fn test_evaluate_pieces() {
        let default_board = Board::default();
        assert_eq!(0, evaluate_pieces(&default_board));
        let empty_board = Board::empty();
        assert_eq!(0, evaluate_pieces(&empty_board));
        let board: Board =
            Fen::from_str("r1b4r/ppp3pp/1bnk4/3Np3/4P3/3PBN2/PPP2PPP/2KR3R w - - 5 14")
                .unwrap()
                .into();
        assert_eq!(26, board.get_pieces().len());
        assert_eq!(15, evaluate_pieces(&board));
    }

    #[test]
    fn test_eval_pieces() {
        let first_piece = RefCell::new(BoardPiece::new_from_type(
            PieceType::Pawn,
            (0, 0).into(),
            PieceColor::Light,
        ));
        let second_piece = RefCell::new(BoardPiece::new_from_type(
            PieceType::Queen,
            (1, 0).into(),
            PieceColor::Light,
        ));
        let piece_vector = vec![&first_piece, &second_piece];
        assert_eq!(100, eval_pieces(piece_vector));
    }

    #[test]
    fn test_eval_board() {
        let default_board: Board = board::Board::default();
        let result = default_board.eval_board();
        // See https://floating-point-gui.de/errors/comparison/
        assert_eq!(0, result as i8);
        // TODO: More Tests
    }
}
