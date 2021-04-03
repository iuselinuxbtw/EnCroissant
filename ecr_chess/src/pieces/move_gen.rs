//! Pseudo-legal moves are generated here. For moves during check we'll use another generator.
// TODO: As these functions are used often and use a lot of resources they have to be optimized well, so implementing benchmarks here would be great.

use std::convert::TryFrom;
use std::rc::Rc;

use crate::board;
use crate::board::SquareInner;
use crate::coordinate::Coordinate;
use crate::pieces::PieceColor;
use std::ops::Deref;

/// Defines a move in the most basic form.
///
/// Only defines where the move goes and whether or not the move is a capture.
#[derive(Debug, PartialEq, Copy, Clone)]
struct BasicMove {
    to: Coordinate,
    capture: bool,
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

/// Utility enum for the function explore_diagonal_moves. Assigns each diagonal direction a on the
/// chess board a cardinal direction. You can look up the cardinal directions
/// [here](https://en.wikipedia.org/wiki/Cardinal_direction).
enum DiagonalDirections {
    // upper-left
    NW,
    // upper-right
    NE,
    // down-right
    SE,
    // down-left
    SW,
}

/// Utility enum for the function explore_linear_moves. Assigns each linear direction a on the chess
/// board a cardinal direction. You can look up the cardinal directions
/// [here](https://en.wikipedia.org/wiki/Cardinal_direction).
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

/// This enum combines LinearDirections and DiagonalDirections. Useful for the explore_knight_moves.
/// The first direction always refers to the direction where the knight jumps further. These are
/// cardinal directions, which you can look up [here](https://en.wikipedia.org/wiki/Cardinal_direction).
enum KnightDirections {
    // First the linear directions.
    // left-then-up
    WN,
    // right-then-up
    EN,
    // right-then-down
    ES,
    // left-then-down
    WS,
    // And the diagonal ones as well.
    // up-then-left
    NW,
    // up-then-right
    NE,
    // down-then-right
    SE,
    // down-then-left
    SW,
}
/// This enum holds the combined directions of LinearDirections and DiagonalDirections. Used for
/// e.g. KingDirections
enum Directions {
    // Linear Directions
    // up
    N,
    // right
    E,
    // down
    S,
    // left
    W,
    // Diagonal Directions
    // upper-left
    NW,
    // upper-right
    NE,
    // down-right
    SE,
    // down-left
    SW,
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
            $result.push(BasicMove{to: possible_square.0.unwrap(), capture: true});
            break;
        }
        $result.push(BasicMove{to: possible_square.0.unwrap(), capture: false});
    }
}

/// Returns the possible linear moves of a piece with the given coordinates as a vector of
/// coordinates, also checks whether there are pieces in the way. An example of a piece that moves
/// this way is a rook.
fn linear_moves(
    start: Coordinate,
    board: &board::Board,
    team_color: &PieceColor,
) -> Vec<BasicMove> {
    // First we initialize a new vector, which we later return
    let mut result: Vec<BasicMove> = Vec::new();

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
) -> Vec<BasicMove> {
    // Create a vector that will be returned at the end.
    let mut result: Vec<BasicMove> = Vec::new();
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

/// Used for generating moves for pawns.
fn pawn_moves(
    start: &Coordinate,
    team_color: &PieceColor,
    board: &board::Board,
    has_moved: bool,
) -> Vec<BasicMove> {
    let mut result: Vec<BasicMove> = Vec::new();
    let from_x = start.get_x() as u8;
    let from_y = start.get_y() as u8;

    let next_r = next_row(from_y, team_color, 1);

    // If there is no piece in front of our pawn we can move there.
    if !piece_in_front(start, team_color, board, 1) {
        &result.push(BasicMove {
            to: (from_x, next_r).into(),
            capture: false,
        });
        // If this is the first move of the pawn and there is not a Piece in the way we can move two squares.
        if !piece_in_front(start, team_color, board, 2) && !has_moved {
            &result.push(BasicMove {
                to: (from_x, next_row(from_y, team_color, 2)).into(),
                capture: false,
            });
        }
    }

    // Pawns can capture diagonally
    // This could be moved into a function that returns whether the piece on the square is the own team color.
    let capture_diagonal: Vec<Coordinate>;
    if from_x == 0 {
        capture_diagonal = vec![(from_x + 1, next_r).into()];
    } else {
        capture_diagonal = vec![(from_x - 1, next_r).into(), (from_x + 1, next_r).into()];
    }
    for possible_capture in capture_diagonal {
        let square_inner = piece_on_square(&possible_capture, board);
        if let Some(e) = square_inner {
            if &e.as_ref().borrow().deref().get_color() != team_color {
                &result.push(BasicMove {
                    to: e.as_ref().borrow().deref().get_coordinate(),
                    capture: true,
                });
            }
        }
    }
    result
}

fn knight_moves(
    start: &Coordinate,
    team_color: &PieceColor,
    board: &board::Board,
) -> Vec<BasicMove> {
    let mut all_directions: Vec<KnightDirections> = vec![
        KnightDirections::NW,
        KnightDirections::NE,
        KnightDirections::SW,
        KnightDirections::SE,
        KnightDirections::ES,
        KnightDirections::EN,
        KnightDirections::WN,
        KnightDirections::WS,
    ];
    // This queue is used to add the directions which can be scanned without resulting in invalid coordinates.
    let mut queue: Vec<KnightDirections> = vec![];
    // TODO: Return whether the moves contain a fork
    let mut result: Vec<BasicMove> = Vec::new();
    let border_distances = distance_to_border(start);
    // TODO: Make this another function and the directions as macros
    // This covers the positions from the fight against the clock to the left and then down
    if border_distances.right > 1 {
        if border_distances.up > 0 {
            &queue.push(KnightDirections::ES);
        }
        if border_distances.down > 0 {
            &queue.push(KnightDirections::EN);
        }
    }
    if border_distances.up > 1 {
        if border_distances.left > 0 {
            &queue.push(KnightDirections::NE);
        }
        if border_distances.right > 0 {
            &queue.push(KnightDirections::NW);
        }
    }
    if border_distances.left > 1 {
        if border_distances.left > 0 {
            &queue.push(KnightDirections::WN);
        }
        if border_distances.right > 0 {
            &queue.push(KnightDirections::WS);
        }
    }
    if border_distances.down > 1 {
        if border_distances.left > 0 {
            &queue.push(KnightDirections::SW);
        }
        if border_distances.right > 0 {
            &queue.push(KnightDirections::SE);
        }
    }
    for e in queue {
        result.append(&mut explore_knight_moves(start, team_color, board, e));
    }
    result
}
/// This macro is essentially the same as check_square without the 'break' statements so that it can
/// be used outside of a loop.
macro_rules! check_move {
    ($x: expr, $y: expr, $team_color: expr, $result: expr, $board: expr) => {
        let possible_square =  coordinate_check($x as &usize, $y as &usize, $team_color, $board);
        // If the square is occupied by a piece
        if possible_square.1{
            // Check if it is our own piece.
            if possible_square.0.is_none() {
                // If it is, we shouldn't add that square to the array since we can't capture our own pieces.
                return $result
            }
            // It's safe to use unwrap here since we already know that it's not None.
            // If it is the enemies piece we can capture it.
            $result.push(BasicMove{to: possible_square.0.unwrap(), capture: true});
            return $result
        }
        $result.push(BasicMove{to: possible_square.0.unwrap(), capture: false});
    }
}

/// This function returns the knight moves in a particular direction. This function does not check
/// whether or the square is valid so to avoid overflows check the corner distance and call the
/// directions accordingly.
fn explore_knight_moves(
    start: &Coordinate,
    team_color: &PieceColor,
    board: &board::Board,
    direction: KnightDirections,
) -> Vec<BasicMove> {
    let from_x: usize = start.get_x() as usize;
    let from_y: usize = start.get_y() as usize;
    let mut result: Vec<BasicMove> = vec![];
    match direction {
        KnightDirections::WN => {
            check_move!(&(from_x - 2), &(from_y + 1), team_color, result, board);
        }
        KnightDirections::EN => {
            check_move!(&(from_x + 2), &(from_y + 1), team_color, result, board);
        }
        KnightDirections::ES => {
            check_move!(&(from_x + 2), &(from_y - 1), team_color, result, board);
        }
        KnightDirections::WS => {
            check_move!(&(from_x - 2), &(from_y - 1), team_color, result, board);
        }
        KnightDirections::NW => {
            check_move!(&(from_x - 1), &(from_y + 2), team_color, result, board);
        }
        KnightDirections::NE => {
            check_move!(&(from_x + 1), &(from_y + 2), team_color, result, board);
        }
        KnightDirections::SE => {
            check_move!(&(from_x + 1), &(from_y - 2), team_color, result, board);
        }
        KnightDirections::SW => {
            check_move!(&(from_x - 1), &(from_y - 2), team_color, result, board);
        }
    }
    result
}
/// This function gives back the possible moves for the king (For now?) without castling.
fn king_moves(start: &Coordinate, team_color: &PieceColor, board: &board::Board) -> Vec<BasicMove> {
    let mut result: Vec<BasicMove> = vec![];
    let border_distances = distance_to_border(start);
    let mut queue: Vec<Directions> = vec![];

    // This can be made smarter by only adding the linear directions and filling the diagonals afterwards
    if border_distances.right > 0 {
        &queue.push(Directions::E);
        if border_distances.up > 0 {
            &queue.push(Directions::NE);
        }
    }
    if border_distances.up < 0 {
        &queue.push(Directions::N);
        if border_distances.left > 0 {
            &queue.push(Directions::NW);
        }
    }
    if border_distances.left < 0 {
        &queue.push(Directions::W);
        if border_distances.down > 0 {
            &queue.push(Directions::SW);
        }
    }
    if border_distances.down < 0 {
        &queue.push(Directions::S);
        if border_distances.right > 0 {
            &queue.push(Directions::SE);
        }
    }
    // Now we iterate through the possible directions and check if the positions are possible.
    for d in queue {
        result.append(&mut explore_king_moves(start, team_color, board, d));
    }
    result
}

fn explore_king_moves(
    start: &Coordinate,
    team_color: &PieceColor,
    board: &board::Board,
    direction: Directions,
) -> Vec<BasicMove> {
    let result: Vec<BasicMove> = vec![];
    let from_x = start.get_x();
    let from_y = start.get_y();
    match direction {
        Directions::N => {
            check_move!(&(from_x), &(from_y + 1), team_color, result, board);
        }
        Directions::E => {
            check_move!(&(from_x + 1), &(from_y), team_color, result, board);
        }
        Directions::S => {
            check_move!(&(from_x), &(from_y - 1), team_color, result, board);
        }
        Directions::W => {
            check_move!(&(from_x - 1), &(from_y), team_color, result, board);
        }
        Directions::NW => {
            check_move!(&(from_x - 1), &(from_y + 1), team_color, result, board);
        }
        Directions::NE => {
            check_move!(&(from_x + 1), &(from_y + 1), team_color, result, board);
        }
        Directions::SE => {
            check_move!(&(from_x + 1), &(from_y - 1), team_color, result, board);
        }
        Directions::SW => {
            check_move!(&(from_x - 1), &(from_y - 1), team_color, result, board);
        }
    }
    result
}

/// This struct holds the distance to the different borders of a coordinate. Useful for calculating
/// in which directions the knight can go.
struct DistanceToBorder {
    // Distance to the upper border
    up: usize,
    // Distance to the right border
    right: usize,
    // Distance to the lower border
    down: usize,
    // Distance to the left border
    left: usize,
}

/// Returns the distance of a coordinate to every border.
fn distance_to_border(coords: &Coordinate) -> DistanceToBorder {
    let x = coords.get_x() as usize;
    let y = coords.get_y() as usize;
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

fn next_row(y: u8, team_color: &PieceColor, step: usize) -> u8 {
    let mut result: usize = y.clone() as usize;
    // The next row for a pawn is higher if the piece is light and lower if the pawn is dark.
    if team_color == &PieceColor::Light {
        result += step;
    } else {
        result -= step;
    }
    result as u8
}

/// This functions is useful for finding out whether or not a pawn can move forwards by returning
/// true if there is a piece in front. Steps determine how far it will go.
fn piece_in_front(
    from: &Coordinate,
    team_color: &PieceColor,
    board: &board::Board,
    step: usize,
) -> bool {
    let mut next_coordinate: Coordinate = from.clone();

    next_coordinate.y = next_row(from.get_y(), team_color, step);
    // Return false if there is not a piece in front of it.
    if piece_on_square(&next_coordinate, board).is_none() {
        false
    } else {
        true
    }
}

/// Returns the possible diagonal moves of a piece with the given coordinates as a vector of
/// coordinates, also checks whether there are pieces in the way. An example of a piece that moves
/// this way is a bishop.
fn diagonal_moves(
    start: &Coordinate,
    team_color: &PieceColor,
    board: &board::Board,
) -> Vec<BasicMove> {
    // Create a vector that will be returned at the end.
    let mut result: Vec<BasicMove> = Vec::new();

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

/// This function returns all moves into a particular diagonal direction
fn explore_diagonal_direction(
    direction: DiagonalDirections,
    from_x: &usize,
    from_y: &usize,
    team_color: &PieceColor,
    board: &board::Board,
) -> Vec<BasicMove> {
    let mut x = *from_x as i32;
    let mut y = *from_y as i32;
    let mut result: Vec<BasicMove> = Vec::new();
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
    let square_occupied = piece_on_square(square, board);
    match square_occupied {
        // Check whether it is our own piece.
        Some(i) => {
            if i.as_ref().borrow().deref().get_color() == *team_color {
                (None, true)
            } else {
                (Some(*square), true)
            }
        }
        None => (Some(*square), false),
    }
}

// Returns the Piece a square is occupied by. If the square is not occupied it returns None
fn piece_on_square(square: &Coordinate, board: &board::Board) -> Option<SquareInner> {
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
    use crate::pieces::{BoardPiece, PieceType};

    #[test]
    fn test_linear_moves() {
        let board = board::Board::default();
        let result = linear_moves((4, 3).into(), &board, &PieceColor::Light);
        // Make a new Vector and fill it with all possible Coordinates
        let expected: Vec<BasicMove> = vec![
            // North
            BasicMove {
                to: (4, 4).into(),
                capture: false,
            },
            BasicMove {
                to: (4, 5).into(),
                capture: false,
            },
            BasicMove {
                to: (4, 6).into(),
                capture: true,
            },
            // East
            BasicMove {
                to: (5, 3).into(),
                capture: false,
            },
            BasicMove {
                to: (6, 3).into(),
                capture: false,
            },
            BasicMove {
                to: (7, 3).into(),
                capture: false,
            },
            // South
            BasicMove {
                to: (4, 2).into(),
                capture: false,
            },
            // West
            BasicMove {
                to: (3, 3).into(),
                capture: false,
            },
            BasicMove {
                to: (2, 3).into(),
                capture: false,
            },
            BasicMove {
                to: (1, 3).into(),
                capture: false,
            },
            BasicMove {
                to: (0, 3).into(),
                capture: false,
            },
        ];

        assert_eq!(result, expected);

        let gotc: Board =
            Fen::from_str("r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R b - - 3 17")
                .unwrap()
                .into();
        let moves_a1 = linear_moves((0, 7).into(), &gotc, &PieceColor::Dark);
        let expected_moves_a1: Vec<BasicMove> = vec![
            BasicMove {
                to: (1, 7).into(),
                capture: false,
            },
            BasicMove {
                to: (2, 7).into(),
                capture: false,
            },
            BasicMove {
                to: (3, 7).into(),
                capture: false,
            },
        ];
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
        let expected: Vec<BasicMove> = vec![
            BasicMove {
                to: (4, 3).into(),
                capture: false,
            },
            BasicMove {
                to: (5, 4).into(),
                capture: false,
            },
            BasicMove {
                to: (6, 5).into(),
                capture: false,
            },
            BasicMove {
                to: (7, 6).into(),
                capture: false,
            },
        ];
        assert_eq!(result, expected);

        // Do the same for the North-west (upper-left) direction from h1
        let result2 = explore_diagonal_direction(
            DiagonalDirections::NW,
            &7,
            &0,
            &PieceColor::Dark,
            &empty_board,
        );
        let expected2: Vec<BasicMove> = vec![
            BasicMove {
                to: (6, 1).into(),
                capture: false,
            },
            BasicMove {
                to: (5, 2).into(),
                capture: false,
            },
            BasicMove {
                to: (4, 3).into(),
                capture: false,
            },
            BasicMove {
                to: (3, 4).into(),
                capture: false,
            },
            BasicMove {
                to: (2, 5).into(),
                capture: false,
            },
            BasicMove {
                to: (1, 6).into(),
                capture: false,
            },
            BasicMove {
                to: (0, 7).into(),
                capture: false,
            },
        ];
        assert_eq!(result2, expected2);

        // Now do the whole thing with a filled board in the direction of NW (upper left) from e3
        // The fen string for the bishop from this position would be: 'rnbqkbnr/pppppppp/8/8/8/4B3/PPPPPPPP/RNBQKBNR w KQkq - 0 1'
        let default_board = Board::default();
        let result3 = explore_diagonal_direction(
            DiagonalDirections::NW,
            &4,
            &2,
            &PieceColor::Light,
            &default_board,
        );
        let expected3: Vec<BasicMove> = vec![
            BasicMove {
                to: (3, 3).into(),
                capture: false,
            },
            BasicMove {
                to: (2, 4).into(),
                capture: false,
            },
            BasicMove {
                to: (1, 5).into(),
                capture: false,
            },
            BasicMove {
                to: (0, 6).into(),
                capture: true,
            },
        ];
        assert_eq!(result3, expected3);

        // This should be empty as there are only two of our own pieces in that direction.
        let result4 = explore_diagonal_direction(
            DiagonalDirections::SE,
            &3,
            &2,
            &PieceColor::Light,
            &default_board,
        );
        let expected4: Vec<BasicMove> = vec![];
        assert_eq!(result4, expected4);
    }

    #[test]
    fn test_diagonal_moves() {
        let board = Board::empty();
        let result = diagonal_moves(&(4, 3).into(), &PieceColor::Dark, &board);
        let expected: Vec<BasicMove> = vec![
            // North-west (upper left)
            BasicMove {
                to: (3, 4).into(),
                capture: false,
            },
            BasicMove {
                to: (2, 5).into(),
                capture: false,
            },
            BasicMove {
                to: (1, 6).into(),
                capture: false,
            },
            BasicMove {
                to: (0, 7).into(),
                capture: false,
            },
            // North-east (upper right)
            BasicMove {
                to: (5, 4).into(),
                capture: false,
            },
            BasicMove {
                to: (6, 5).into(),
                capture: false,
            },
            BasicMove {
                to: (7, 6).into(),
                capture: false,
            },
            // South-east (lower right)
            BasicMove {
                to: (5, 2).into(),
                capture: false,
            },
            BasicMove {
                to: (6, 1).into(),
                capture: false,
            },
            BasicMove {
                to: (7, 0).into(),
                capture: false,
            },
            // South-west (lower left)
            BasicMove {
                to: (3, 2).into(),
                capture: false,
            },
            BasicMove {
                to: (2, 1).into(),
                capture: false,
            },
            BasicMove {
                to: (1, 0).into(),
                capture: false,
            },
        ];
        assert_eq!(result, expected);
        // TODO: Test this with a filled board
    }

    #[test]
    fn test_piece_is_on_square() {
        let default_board = board::Board::default();
        // Check where the pawn is in the default position
        let pawn_coords: Coordinate = (0, 1).into();
        let pawn = BoardPiece::new_from_type(PieceType::Pawn, pawn_coords, PieceColor::Light);
        let piece = piece_on_square(&pawn_coords, &default_board);
        assert_eq!(*piece.unwrap().as_ref().borrow().deref(), pawn);

        let king_coords: Coordinate = (4, 7).into();
        let king = BoardPiece::new_from_type(PieceType::King, king_coords, PieceColor::Dark);
        let piece2 = piece_on_square(&king_coords, &default_board);
        assert_eq!(*piece2.unwrap().as_ref().borrow().deref(), king);
    }

    #[test]
    fn test_pawn_moves() {
        let default_board = board::Board::default();
        let result = pawn_moves(&(0, 1).into(), &PieceColor::Light, &default_board, false);
        let expected = vec![
            BasicMove {
                to: (0, 2).into(),
                capture: false,
            },
            BasicMove {
                to: (0, 3).into(),
                capture: false,
            },
        ];
        assert_eq!(result, expected);

        let result2 = pawn_moves(&(2, 5).into(), &PieceColor::Light, &default_board, false);
        let expected2 = vec![
            BasicMove {
                to: (1, 6).into(),
                capture: true,
            },
            BasicMove {
                to: (3, 6).into(),
                capture: true,
            },
        ];
        assert_eq!(result2, expected2);

        let result3 = pawn_moves(&(7, 1).into(), &PieceColor::Light, &default_board, true);
        let expected3 = vec![BasicMove {
            to: (7, 2).into(),
            capture: false,
        }];
        assert_eq!(result3, expected3);
    }

    #[test]
    fn test_knight_moves() {
        let default_board = board::Board::default();
        let result = knight_moves(&(3, 3).into(), &PieceColor::Light, &default_board);
        let expected: Vec<BasicMove> = vec![
            BasicMove {
                to: (5, 2).into(),
                capture: false,
            },
            BasicMove {
                to: (5, 4).into(),
                capture: false,
            },
            BasicMove {
                to: (4, 5).into(),
                capture: false,
            },
            BasicMove {
                to: (2, 5).into(),
                capture: false,
            },
            BasicMove {
                to: (1, 4).into(),
                capture: false,
            },
            BasicMove {
                to: (1, 2).into(),
                capture: false,
            },
        ];
        assert_eq!(result, expected);
    }
}
