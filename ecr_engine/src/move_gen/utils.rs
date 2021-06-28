//! This file holds all the utility functions for the move_gen module.

use crate::board;
use crate::board::SquareInner;
use ecr_shared::coordinate::Coordinate;
use std::rc::Rc;

use std::ops::Deref;

use crate::pieces::{PieceColor, PieceType};
use crate::move_gen::directions::Directions;

/// This functions is useful for finding out whether or not a pawn can move forwards by returning
/// true if there is a piece in front. Steps determine how far it will go.
pub(crate) fn piece_in_front(
    from: Coordinate,
    team_color: PieceColor,
    board: &board::Board,
    step: u8,
) -> bool {
    let mut next_coordinate: Coordinate = from;
    next_coordinate.y = next_row(from.get_y(), team_color, step);

    // Return false if there is not a piece in front of it.
    piece_on_square(next_coordinate, board).is_some()
}

/// Returns true if there is no piece in the way. Useful for [`get_castle_moves`]
pub(crate) fn no_piece_in_the_way(
    board: &board::Board,
    start: Coordinate,
    direction: Directions,
    range: u8,
) -> bool {
    let x = start.get_x();
    let y = start.get_y();
    match direction {
        Directions::N => {
            for increment in 0..range {
                if piece_on_square((x, y + increment).into(), board).is_some() {
                    return false;
                }
            }
        }
        Directions::E => {
            for increment in 0..range {
                if piece_on_square((x + increment, y).into(), board).is_some() {
                    return false;
                }
            }
        }
        Directions::S => {
            for decrement in 0..range {
                if piece_on_square((x, y - decrement).into(), board).is_some() {
                    return false;
                }
            }
        }
        Directions::W => {
            for decrement in 0..range {
                if piece_on_square((x - decrement, y).into(), board).is_some() {
                    return false;
                }
            }
        }
        _ => {todo!()}
    }
    true
}

// Returns the Piece a square is occupied by. If the square is not occupied it returns None
pub(crate) fn piece_on_square(square: Coordinate, board: &board::Board) -> Option<SquareInner> {
    // Get the SquareInner
    board.get_at(square).map(|i| Rc::clone(&i))
}

/// This macro is used to break the loop of calculating positions when the current square is
/// occupied. Breaks instantly when the square is occupied by a piece of the own color, but not
/// when the piece is the  opponents color in which case it adds the position and then breaks.
/// If it is neither of those it just adds it to the result.
#[macro_export]
macro_rules! check_square_in_loop {
    ($x: expr, $y: expr, $team_color: expr, $result: expr, $board: expr) => {
        let possible_square =  coordinate_check(&$x, &$y, $team_color, $board);
        // If the square is occupied by a piece
        if possible_square.0.is_some() {
            // Check if it is our own piece.
            if !possible_square.1 {
                // If it is, we shouldn't add that square to the array since we can't capture our own pieces.
                break;
            }
            // It's safe to use unwrap here since we already know that it's not None.
            // If it is the enemies piece we can capture it.
            $result.push(BasicMove{to: ($x, $y).into(), capture: Some(Capture{piece_type: possible_square.0.unwrap(), target: ($x,$y).into()})});
            break;
        }
        $result.push(BasicMove{to: ($x, $y).into(), capture: None});
    }
}

/// This macro is essentially the same as check_square without the 'break' statements so that it can
/// be used outside of a loop.
#[macro_export]
macro_rules! check_this_move {
    ($x: expr, $y: expr, $team_color: expr, $result: expr, $board: expr) => {
        let possible_square =  coordinate_check(&$x, &$y , $team_color, $board);
        // If the square is occupied by a piece
        if possible_square.0.is_some(){
            // Check if it is our own piece.
            if !possible_square.1 {
                // If it is, we shouldn't add that square to the array since we can't capture our own pieces.
                return $result
            }
            // It's safe to use unwrap here since we already know that it's not None.
            // If it is the enemies piece we can capture it.
            $result.push(BasicMove{to: ($x, $y).into(), capture: Some(Capture{piece_type: possible_square.0.unwrap(), target: ($x,$y).into()})});            return $result
        }
        $result.push(BasicMove{to: ($x, $y).into(), capture: None});
    }
}

/// This struct holds the distance to the different borders of a coordinate. Useful for calculating
/// in which directions the knight can go.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct DistanceToBorder {
    // Distance to the upper border
    pub(crate) up: u8,
    // Distance to the right border
    pub(crate) right: u8,
    // Distance to the lower border
    pub(crate) down: u8,
    // Distance to the left border
    pub(crate) left: u8,
}

/// Returns the distance of a coordinate to every border.
pub fn distance_to_border(coords: Coordinate) -> DistanceToBorder {
    let x = coords.get_x();
    let y = coords.get_y();
    let up = 7 - y;
    let right = 7 - x;
    let down = y;
    let left = x;
    DistanceToBorder {
        up,
        right,
        down,
        left,
    }
}

/// This function returns the next row of the corresponding team. (If the team_color is white it's
/// higher, otherwise it's lower). So far there is no check whether the returning row is valid but in
/// most variants it is impossible since the pawn promotes when reaching the last row.
pub fn next_row(y: u8, team_color: PieceColor, step: u8) -> u8 {
    let mut result: u8 = y;
    // The next row for a pawn is higher if the piece is light and lower if the pawn is dark.
    if team_color == PieceColor::Light {
        result += step;
    } else {
        result -= step;
    }
    result as u8
}

/// Calculates a square and then just calls square_check()
pub fn coordinate_check(
    x: &u8,
    y: &u8,
    team_color: PieceColor,
    board: &board::Board,
) -> (Option<PieceType>, bool) {
    let square = (*x as u8, *y as u8).into();
    check_square(square, team_color, board)
}

/// Checks if a square is occupied. If it is it returns Some(PieceType), if it is not, the first element of the tuple is none.
/// The second element returns true if it is an enemy piece, false otherwise.
fn check_square(
    square: Coordinate,
    team_color: PieceColor,
    board: &board::Board,
) -> (Option<PieceType>, bool) {
    // We need to check if the square is occupied to avoid calculating non-reachable coordinates
    let square_occupied = piece_on_square(square, board);
    match square_occupied {
        // Check whether it is our own piece.
        Some(i) => {
            let piece_type = i.deref().borrow().get_piece().get_type();
            if i.as_ref().borrow().deref().get_color() == team_color {
                (Some(piece_type), false)
            } else {
                (Some(piece_type), true)
            }
        }
        None => (None, false),
    }
}

mod tests {
    use super::*;
    mod macros {
        use super::*;
        use crate::board::Board;
        use crate::pieces::BoardPiece;

        #[test]
        fn test_piece_is_on_square() {
            let default_board = Board::default();
            // Check where the pawn is in the default position
            let pawn_coords: Coordinate = (0, 1).into();
            let pawn = BoardPiece::new_from_type(PieceType::Pawn, pawn_coords, PieceColor::Light);
            let piece = piece_on_square(pawn_coords, &default_board);
            assert_eq!(*piece.unwrap().as_ref().borrow().deref(), pawn);

            let king_coords: Coordinate = (4, 7).into();
            let king = BoardPiece::new_from_type(PieceType::King, king_coords, PieceColor::Dark);
            let piece2 = piece_on_square(king_coords, &default_board);
            assert_eq!(king, *piece2.unwrap().as_ref().borrow().deref());
        }

        #[test]
        fn test_next_row() {
            assert_eq!(5, next_row(4, PieceColor::Light, 1));
            assert_eq!(3, next_row(4, PieceColor::Dark, 1));
            assert_eq!(2, next_row(4, PieceColor::Dark, 2));
            assert_eq!(1, next_row(0, PieceColor::Light, 1));
        }

        #[test]
        fn test_distance_to_borders() {
            assert_eq!(
                DistanceToBorder {
                    up: 7,
                    right: 7,
                    down: 0,
                    left: 0
                },
                distance_to_border((0, 0).into())
            );
        }
    }
}
