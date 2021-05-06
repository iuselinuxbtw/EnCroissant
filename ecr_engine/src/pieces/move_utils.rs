
use std::ops::Deref;
use std::rc::Rc;

use ecr_shared::coordinate::Coordinate;

use crate::board;
use crate::board::SquareInner;
use crate::pieces::{PieceColor, PieceType};

/// This macro is used to break the loop of calculating positions when the current square is
/// occupied. Breaks instantly when the square is occupied by a piece of the own color, but not
/// when the piece is the  opponents color in which case it adds the position and then breaks.
/// If it is neither of those it just adds it to the result.
#[macro_export]
macro_rules! check_square {
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
// TODO: Dumb macro name, change this
#[macro_export]
macro_rules! check_move {
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
pub fn distance_to_border(coords: &Coordinate) -> DistanceToBorder {
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
pub fn next_row(y: u8, team_color: &PieceColor, step: u8) -> u8 {
    let mut result: u8 = y.clone();
    // The next row for a pawn is higher if the piece is light and lower if the pawn is dark.
    if team_color == &PieceColor::Light {
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
    team_color: &PieceColor,
    board: &board::Board,
) -> (Option<PieceType>, bool) {
    let square = (*x as u8, *y as u8).into();
    square_check(square, team_color, board)
}

/// Checks if a square is occupied. If it is it returns Some(PieceType), if it is not, the first element of the tuple is none.
/// The second element returns true if it is an enemy piece, false otherwise.
fn square_check(
    square: Coordinate,
    // TODO: This should not be a reference(Probably optimized by the compiler, but isn't nice)
    team_color: &PieceColor,
    board: &board::Board,
) -> (Option<PieceType>, bool) {
    // We need to check if the square is occupied to avoid calculating non-reachable coordinates
    let square_occupied = piece_on_square(square, board);
    match square_occupied {
        // Check whether it is our own piece.
        Some(i) => {
            let piece_type = i.deref().borrow().get_piece().get_type();
            if i.as_ref().borrow().deref().get_color() == *team_color {
                (Some(piece_type), false)
            } else {
                (Some(piece_type), true)
            }
        }
        None => (None, false),
    }
}

// Returns the Piece a square is occupied by. If the square is not occupied it returns None
pub(crate) fn piece_on_square(square: Coordinate, board: &board::Board) -> Option<SquareInner> {
    // Get the SquareInner
    match board.get_at(square) {
        // Match it
        None => None,
        Some(i) => Some(Rc::clone(&i)),
    }
}
