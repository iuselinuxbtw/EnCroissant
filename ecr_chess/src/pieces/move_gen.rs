use std::convert::TryFrom;
use std::rc::Rc;

use crate::board;
use crate::board::SquareInner;
use crate::coordinate::Coordinate;
use crate::pieces::{BoardPiece, PieceColor, PieceType};

/// Defines a move.
struct Move {
    from: Coordinate,
    to: Coordinate,
    piece_type: PieceType,
    //move_type: Vec<MoveType>,
}

enum MoveType {
    Check,
    Capture,
    Evasion,
    Book,
    Sacrifice,
    Promotion,
    Castle,
}

/// Utility enum for the function explore_diagonal_moves. Assigns each direction a on the chess
/// board a cardinal direction. You can look up the cardinal directions
/// [here](https://en.wikipedia.org/wiki/Cardinal_direction).
enum DiagonalDirections {
    NW,
    // upper-left
    NE,
    // upper-right
    SE,
    // down-right
    SW, // down-left
}

enum LinearDirections {
    // up
    N,
    // right
    E,
    // down
    S,
    // left
    W,
}

/// This macro is used to break the loop of calculating positions when the current square is
/// occupied. Breaks instantly when the square is occupied by a piece of the own color, but not
/// when the piece is the  opponents color in which case it adds the position and then breaks.
/// If it is neither of those it just adds it to the result.
macro_rules! check_square {
    ($x: expr, $y: expr, $team_color: expr, $result: expr, $board: expr) => {
        let possible_square =  coordinate_check($x as &usize, $y as &usize, $team_color, $board);
        // If the square is occupied by a piece
        if possible_square.1{
            // Check if it is our own piece.
            if possible_square.0.is_none() {
                // If it is, we shouldn't add that square to the array since we can't capture our own pieces.
                break;
            }
            // It's safe to use unwrap here since we already know that it's not None.
            // If it is the enemies piece we can capture it.
            // TODO: Change the type to capture if there is an enemy piece on this square.
            $result.push(possible_square.0.unwrap());
            break;
        }
        $result.push(possible_square.0.unwrap());
    }
}

/// Returns the possible linear moves of a piece with the given coordinates as a vector of
/// coordinates, also checks whether there are pieces in the way. An example of a piece that moves
/// this way is a rook.
fn linear_moves(
    start: Coordinate,
    board: &board::Board,
    team_color: &PieceColor,
) -> Vec<Coordinate> {
    // First we initialize a new vector, which we later return
    let mut result: Vec<Coordinate> = Vec::new();

    // Bind the given coordinates to variables because we obviously can
    let from_x = start.get_x() as usize;
    let from_y = start.get_y() as usize;

    // explore all directions
    result.append(&mut explore_linear_direction(
        LinearDirections::N,
        from_x,
        from_y,
        team_color,
        board,
    ));
    result.append(&mut explore_linear_direction(
        LinearDirections::E,
        from_x,
        from_y,
        team_color,
        board,
    ));
    result.append(&mut explore_linear_direction(
        LinearDirections::S,
        from_x,
        from_y,
        team_color,
        board,
    ));
    result.append(&mut explore_linear_direction(
        LinearDirections::W,
        from_x,
        from_y,
        team_color,
        board,
    ));

    result
}

fn explore_linear_direction(
    direction: LinearDirections,
    from_x: usize,
    from_y: usize,
    team_color: &PieceColor,
    board: &board::Board,
) -> Vec<Coordinate> {
    // Create a vector that will be returned at the end.
    let mut result: Vec<Coordinate> = Vec::new();
    let mut x = from_x;
    let mut y = from_y;
    match direction {
        LinearDirections::N => {
            while y < 7 {
                y += 1;
                check_square!(&x, &y, &team_color, result, board);
            }
        }
        LinearDirections::E => {
            while x < 7 {
                x += 1;
                check_square!(&x, &y, &team_color, result, board);
            }
        }
        LinearDirections::S => {
            while y > 0 {
                y -= 1;
                check_square!(&x, &y, &team_color, result, board);
            }
        }
        LinearDirections::W => {
            while x > 0 {
                x -= 1;
                check_square!(&x, &y, &team_color, result, board);
            }
        }
    };
    result
}

/// Returns the possible diagonal moves of a piece with the given coordinates as a vector of
/// coordinates, also checks whether there are pieces in the way. An example of a piece that moves
/// this way is a bishop.
fn diagonal_moves(
    start: &Coordinate,
    team_color: &PieceColor,
    board: &board::Board,
) -> Vec<Coordinate> {
    // Create a vector that will be returned at the end.
    let mut result: Vec<Coordinate> = Vec::new();

    // Bind the starting coordinates to variables
    let from_x = start.get_x() as usize;
    let from_y = start.get_y() as usize;

    // Explore the moves in all directions.
    result.append(&mut explore_diagonal_direction(
        DiagonalDirections::NW,
        &from_x,
        &from_y,
        team_color,
        board,
    ));
    result.append(&mut explore_diagonal_direction(
        DiagonalDirections::NE,
        &from_x,
        &from_y,
        team_color,
        board,
    ));
    result.append(&mut explore_diagonal_direction(
        DiagonalDirections::SE,
        &from_x,
        &from_y,
        team_color,
        board,
    ));
    result.append(&mut explore_diagonal_direction(
        DiagonalDirections::SW,
        &from_x,
        &from_y,
        team_color,
        board,
    ));
    result
}

/// This function returns all moves into a particular direction
fn explore_diagonal_direction(
    direction: DiagonalDirections,
    from_x: &usize,
    from_y: &usize,
    team_color: &PieceColor,
    board: &board::Board,
) -> Vec<Coordinate> {
    let mut x = *from_x as i32;
    let mut y = *from_y as i32;
    let mut result = Vec::new();
    match direction {
        // upper-left
        DiagonalDirections::NW => {
            while x > 0 && y < 7 {
                // First we modify the coordinates so we can calculate the new possible coordinates
                x -= 1;
                y += 1;
                // We can safely unwrap here since the variables can't be less than 0
                check_square!(
                    &usize::try_from(x).unwrap(),
                    &usize::try_from(y).unwrap(),
                    &team_color,
                    result,
                    board
                );
            }
        }
        // upper-right
        DiagonalDirections::NE => {
            while x < 7 && y < 7 {
                x += 1;
                y += 1;
                // We can safely unwrap here since the variables can't be less than 0
                check_square!(
                    &usize::try_from(x).unwrap(),
                    &usize::try_from(y).unwrap(),
                    &team_color,
                    result,
                    board
                );
            }
        }
        // down-right
        DiagonalDirections::SE => {
            while x < 7 && y > 0 {
                x += 1;
                y -= 1;
                // We can safely unwrap here since the variables can't be less than 0
                check_square!(
                    &usize::try_from(x).unwrap(),
                    &usize::try_from(y).unwrap(),
                    &team_color,
                    result,
                    board
                );
            }
        }
        // down-left
        DiagonalDirections::SW => {
            while x > 0 && y > 0 {
                x -= 1;
                y -= 1;
                // We can safely unwrap here since the variables can't be less than 0
                check_square!(
                    &usize::try_from(x).unwrap(),
                    &usize::try_from(y).unwrap(),
                    &team_color,
                    result,
                    board
                );
            }
        }
    }
    result
}

/// Calculates a square and then just calls square_check()
fn coordinate_check(
    x: &usize,
    y: &usize,
    team_color: &PieceColor,
    board: &board::Board,
) -> (Option<Coordinate>, bool) {
    let square = (*x as u8, *y as u8).into();
    square_check(&square, team_color, board)
}

/// Checks if a square is occupied and if it is checks whether it can be captured
/// or if it is the teams own piece, in which case it returns None. The bool returns true if the
/// square is occupied.
fn square_check(
    square: &Coordinate,
    team_color: &PieceColor,
    board: &board::Board,
) -> (Option<Coordinate>, bool) {
    // We need to check if the square is occupied to avoid calculating non-reachable coordinates
    let square_occupied = piece_is_on_square(square, board);
    match square_occupied {
        // Check whether it is our own piece.
        Some(i) => {
            if &i.borrow().get_color() == team_color {
                (None, true)
            } else {
                (Some(*square), true)
            }
        }
        None => (Some(*square), false),
    }
}

// Returns the Piece a square is occupied by. If the square is not occupied it returns None
fn piece_is_on_square(square: &Coordinate, board: &board::Board) -> Option<SquareInner> {
    // Get the SquareInner
    match board.get_at(*square) {
        // Match it
        None => None,
        Some(i) => Some(Rc::clone(&i)),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::board::Board;
    use crate::formats::fen::*;

    use super::*;

    #[test]
    fn test_linear_moves() {
        let board = board::Board::default();
        let mut result = linear_moves((4, 3).into(), &board, &PieceColor::Light);
        // Make a new Vector and fill it with all possible Coordinates
        let expected = vec![
            // North
            (4, 4).into(),
            (4, 5).into(),
            (4, 6).into(),
            // East
            (5, 3).into(),
            (6, 3).into(),
            (7, 3).into(),
            // South
            (4, 2).into(),
            // West
            (3, 3).into(),
            (2, 3).into(),
            (1, 3).into(),
            (0, 3).into(),
        ];

        assert_eq!(result, expected);

        let gotc: Board =
            Fen::from_str("r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R b - - 3 17")
                .unwrap()
                .into();
        let moves_a1 = linear_moves((0, 7).into(), &gotc, &PieceColor::Dark);
        let expected_moves_a1: Vec<Coordinate> = vec![(1, 7).into(), (2, 7).into(), (3, 7).into()];
        assert_eq!(moves_a1, expected_moves_a1);
    }

    #[test]
    fn test_explore_diagonal_moves() {
        let empty_board = board::Board::empty();
        // Calculate the moves in the North-east (upper-right) direction from 3,2(d3)
        let result = explore_diagonal_direction(
            DiagonalDirections::NE,
            &3,
            &2,
            &PieceColor::Light,
            &empty_board,
        );
        let expected: Vec<Coordinate> =
            vec![(4, 3).into(), (5, 4).into(), (6, 5).into(), (7, 6).into()];
        assert_eq!(result, expected);

        // Do the same for the North-west (upper-left) direction from h1
        let result2 = explore_diagonal_direction(
            DiagonalDirections::NW,
            &7,
            &0,
            &PieceColor::Dark,
            &empty_board,
        );
        let expected2: Vec<Coordinate> = vec![
            (6, 1).into(),
            (5, 2).into(),
            (4, 3).into(),
            (3, 4).into(),
            (2, 5).into(),
            (1, 6).into(),
            (0, 7).into(),
        ];
        assert_eq!(result2, expected2);

        // Now do the whole thing with a filled board.
        let default_board = Board::default();
        let result3 = explore_diagonal_direction(
            DiagonalDirections::NW,
            &4,
            &2,
            &PieceColor::Light,
            &default_board,
        );
        let expected3: Vec<Coordinate> =
            vec![(3, 3).into(), (2, 4).into(), (1, 5).into(), (0, 6).into()];
        assert_eq!(result3, expected3);

        let result4 = explore_diagonal_direction(
            DiagonalDirections::SE,
            &3,
            &2,
            &PieceColor::Light,
            &default_board,
        );
        let expected4: Vec<Coordinate> = vec![];
        assert_eq!(result4, expected4);
    }

    #[test]
    fn test_diagonal_moves() {
        let board = Board::empty();
        let result = diagonal_moves(&(4, 3).into(), &PieceColor::Dark, &board);
        let expected: Vec<Coordinate> = vec![
            // North-west (upper left)
            (3, 4).into(),
            (2, 5).into(),
            (1, 6).into(),
            (0, 7).into(),
            // North-east (upper right)
            (5, 4).into(),
            (6, 5).into(),
            (7, 6).into(),
            // South-east (lower right)
            (5, 2).into(),
            (6, 1).into(),
            (7, 0).into(),
            // South-west (lower left)
            (3, 2).into(),
            (2, 1).into(),
            (1, 0).into(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_piece_is_on_square() {
        let default_board = board::Board::default();
        // Check where the pawn is in the default position
        let pawn_coords: Coordinate = (0, 1).into();
        let pawn = BoardPiece::new_from_type(PieceType::Pawn, pawn_coords, PieceColor::Light);
        let mut piece = piece_is_on_square(&pawn_coords, &default_board);
        assert_eq!(piece.unwrap().borrow().clone(), pawn);

        let king_coords: Coordinate = (4, 7).into();
        let king = BoardPiece::new_from_type(PieceType::King, king_coords, PieceColor::Dark);
        piece = piece_is_on_square(&king_coords, &default_board);
        assert_eq!(piece.unwrap().borrow().clone(), king);
    }
}
