//! Pseudo-legal moves are generated here. For moves during check we'll use another generator.

use std::ops::Deref;

use ecr_shared::coordinate::Coordinate;
use ecr_shared::pieces::PieceType;

use crate::board;
use crate::board::{Board, BoardCastleState};
use crate::move_gen::directions::*;
use crate::move_gen::utils::{
    coordinate_check, next_row, no_piece_in_the_way, piece_in_front, piece_on_square,
};
use crate::move_gen::utils::{distance_to_border, DistanceToBorder};
use crate::move_gen::Capture;
use crate::move_gen::{BasicMove, CastleMove, CastleMoveType};
use crate::pieces::PieceColor;
use crate::{check_square_in_loop, check_this_move};

/// Returns the possible linear moves of a piece with the given coordinates as a vector of
/// coordinates, also checks whether there are pieces in the way. An example of a piece that moves
/// this way is a rook.
pub fn linear_moves(
    start: Coordinate,
    board: &board::Board,
    team_color: PieceColor,
) -> Vec<BasicMove> {
    // First we initialize a new vector, which we later return
    let mut result: Vec<BasicMove> = Vec::new();

    // Bind the given coordinates to variables because we obviously can
    let from_x = start.get_x();
    let from_y = start.get_y();

    // explore all directions
    result.append(&mut explore_direction(
        Directions::N,
        from_x,
        from_y,
        team_color,
        board,
    ));
    result.append(&mut explore_direction(
        Directions::E,
        from_x,
        from_y,
        team_color,
        board,
    ));
    result.append(&mut explore_direction(
        Directions::S,
        from_x,
        from_y,
        team_color,
        board,
    ));
    result.append(&mut explore_direction(
        Directions::W,
        from_x,
        from_y,
        team_color,
        board,
    ));

    result
}

/// Useful for exploring the squares in a linear direction of a piece. Used for
/// rook and Queen move generation.
fn explore_direction(
    direction: Directions,
    from_x: u8,
    from_y: u8,
    team_color: PieceColor,
    board: &board::Board,
) -> Vec<BasicMove> {
    // Create a vector that will be returned at the end.
    let mut result: Vec<BasicMove> = Vec::new();
    let mut x = from_x;
    let mut y = from_y;
    match direction {
        Directions::N => {
            while y < 7 {
                y += 1;
                check_square_in_loop!(x, y, team_color, result, board);
            }
        }
        Directions::E => {
            while x < 7 {
                x += 1;
                check_square_in_loop!(x, y, team_color, result, board);
            }
        }
        Directions::S => {
            while y > 0 {
                y -= 1;
                check_square_in_loop!(x, y, team_color, result, board);
            }
        }
        Directions::W => {
            while x > 0 {
                x -= 1;
                check_square_in_loop!(x, y, team_color, result, board);
            }
        }
        // upper-left
        Directions::NW => {
            while x > 0 && y < 7 {
                // First we modify the coordinates so we can calculate the new possible coordinates
                x -= 1;
                y += 1;
                // We can safely unwrap here since the variables can't be less than 0
                check_square_in_loop!(x, y, team_color, result, board);
            }
        }
        // upper-right
        Directions::NE => {
            while x < 7 && y < 7 {
                x += 1;
                y += 1;
                // We can safely unwrap here since the variables can't be less than 0
                check_square_in_loop!(x, y, team_color, result, board);
            }
        }
        // down-right
        Directions::SE => {
            while x < 7 && y > 0 {
                x += 1;
                y -= 1;
                // We can safely unwrap here since the variables can't be less than 0
                check_square_in_loop!(x, y, team_color, result, board);
            }
        }
        // down-left
        Directions::SW => {
            while x > 0 && y > 0 {
                x -= 1;
                y -= 1;
                // We can safely unwrap here since the variables can't be less than 0
                check_square_in_loop!(x, y, team_color, result, board);
            }
        }
    };
    result
}

/// Used for generating moves for pawns.
pub fn pawn_moves(
    start: Coordinate,
    board: &board::Board,
    team_color: PieceColor,
    has_moved: bool,
) -> Vec<BasicMove> {
    // TODO: Cleanup
    let mut result: Vec<BasicMove> = Vec::new();
    let from_x = start.get_x() as u8;
    let from_y = start.get_y() as u8;

    let next_r = next_row(from_y, team_color, 1);

    // If there is no piece in front of our pawn we can move there.
    if !piece_in_front(start, team_color, board, 1) {
        result.push(BasicMove {
            to: (from_x, next_r).into(),
            capture: None,
        });
        // If this is the first move of the pawn and there is not a Piece in the way we can move two squares.
        if !piece_in_front(start, team_color, board, 2) && !has_moved {
            result.push(BasicMove {
                to: (from_x, next_row(from_y, team_color, 2)).into(),
                capture: None,
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

    // Iterate through both possible captures
    for possible_capture in capture_diagonal {
        let square_inner = piece_on_square(possible_capture, board);
        // If there is a piece on the square
        if let Some(e) = square_inner {
            // If it is the opponent's piece, we add the capture move.
            if e.as_ref().borrow().deref().get_color() != team_color {
                result.push(BasicMove {
                    to: possible_capture,
                    capture: Some(Capture {
                        piece_type: e.deref().borrow().get_piece().get_type(),
                        target: possible_capture,
                    }),
                });
            }
        }
        // TODO: Test en_passant
        if let Some(t) = board.get_en_passant_target() {
            if possible_capture == t.target_square {
                result.push(BasicMove {
                    to: possible_capture,
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (6, 1).into(),
                    }),
                });
            }
        }
    }
    result
}

/// This function returns the moves of a knight
pub fn knight_moves(
    start: Coordinate,
    board: &board::Board,
    team_color: PieceColor,
) -> Vec<BasicMove> {
    // This queue is used to add the directions which can be scanned without resulting in invalid coordinates.
    let mut queue: Vec<KnightDirections> = vec![];
    let mut result: Vec<BasicMove> = Vec::new();
    let border_distances = distance_to_border(start);
    // This covers the positions from the right against the clock to the left and then down
    // TODO: Cleanup
    if border_distances.right > 1 {
        if border_distances.down > 0 {
            queue.push(KnightDirections::ES);
        }
        if border_distances.up > 0 {
            queue.push(KnightDirections::EN);
        }
    }
    if border_distances.up > 1 {
        if border_distances.right > 0 {
            queue.push(KnightDirections::NE);
        }
        if border_distances.left > 0 {
            queue.push(KnightDirections::NW);
        }
    }
    if border_distances.left > 1 {
        if border_distances.up > 0 {
            queue.push(KnightDirections::WN);
        }
        if border_distances.down > 0 {
            queue.push(KnightDirections::WS);
        }
    }
    if border_distances.down > 1 {
        if border_distances.left > 0 {
            queue.push(KnightDirections::SW);
        }
        if border_distances.right > 0 {
            queue.push(KnightDirections::SE);
        }
    }
    for e in queue {
        result.append(&mut explore_knight_moves(start, team_color, board, e));
    }
    result
}

/// This function returns the knight moves in a particular direction. This function does not check
/// whether or the square is valid so to avoid overflows check the corner distance and call the
/// directions accordingly.
fn explore_knight_moves(
    start: Coordinate,
    team_color: PieceColor,
    board: &board::Board,
    direction: KnightDirections,
) -> Vec<BasicMove> {
    let from_x = start.get_x();
    let from_y = start.get_y();
    let mut result: Vec<BasicMove> = vec![];
    match direction {
        KnightDirections::WN => {
            check_this_move!(from_x - 2, from_y + 1, team_color, result, board);
        }
        KnightDirections::EN => {
            check_this_move!(from_x + 2, from_y + 1, team_color, result, board);
        }
        KnightDirections::ES => {
            check_this_move!(from_x + 2, from_y - 1, team_color, result, board);
        }
        KnightDirections::WS => {
            check_this_move!(from_x - 2, from_y - 1, team_color, result, board);
        }
        KnightDirections::NW => {
            check_this_move!(from_x - 1, from_y + 2, team_color, result, board);
        }
        KnightDirections::NE => {
            check_this_move!(from_x + 1, from_y + 2, team_color, result, board);
        }
        KnightDirections::SE => {
            check_this_move!(from_x + 1, from_y - 2, team_color, result, board);
        }
        KnightDirections::SW => {
            check_this_move!(from_x - 1, from_y - 2, team_color, result, board);
        }
    }
    result
}

/// This function gives back the possible moves for the king (For now?) without castling.
pub fn king_moves(
    start: Coordinate,
    board: &board::Board,
    team_color: PieceColor,
) -> Vec<BasicMove> {
    let mut result: Vec<BasicMove> = vec![];
    let border_distances = distance_to_border(start);
    let mut queue: Vec<Directions> = vec![];

    // This can be made smarter by only adding the linear directions and filling the diagonals afterwards
    if border_distances.right > 0 {
        queue.push(Directions::E);
        if border_distances.up > 0 {
            queue.push(Directions::NE);
        }
    }
    if border_distances.up > 0 {
        queue.push(Directions::N);
        if border_distances.left > 0 {
            queue.push(Directions::NW);
        }
    }
    if border_distances.left > 0 {
        queue.push(Directions::W);
        if border_distances.down > 0 {
            queue.push(Directions::SW);
        }
    }
    if border_distances.down > 0 {
        queue.push(Directions::S);
        if border_distances.right > 0 {
            queue.push(Directions::SE);
        }
    }
    // Now we iterate through the possible directions and check if the positions are possible.
    for d in queue {
        result.append(&mut explore_king_moves(start, team_color, board, d));
    }
    result
}

/// This function returns the king moves in a particular direction.
fn explore_king_moves(
    start: Coordinate,
    team_color: PieceColor,
    board: &board::Board,
    direction: Directions,
) -> Vec<BasicMove> {
    let mut result: Vec<BasicMove> = vec![];
    let from_x = start.get_x();
    let from_y = start.get_y();
    match direction {
        Directions::N => {
            check_this_move!((from_x), (from_y + 1), team_color, result, board);
        }
        Directions::E => {
            check_this_move!((from_x + 1), (from_y), team_color, result, board);
        }
        Directions::S => {
            check_this_move!((from_x), (from_y - 1), team_color, result, board);
        }
        Directions::W => {
            check_this_move!((from_x - 1), (from_y), team_color, result, board);
        }
        Directions::NW => {
            check_this_move!((from_x - 1), (from_y + 1), team_color, result, board);
        }
        Directions::NE => {
            check_this_move!((from_x + 1), (from_y + 1), team_color, result, board);
        }
        Directions::SE => {
            check_this_move!((from_x + 1), (from_y - 1), team_color, result, board);
        }
        Directions::SW => {
            check_this_move!((from_x - 1), (from_y - 1), team_color, result, board);
        }
    }
    // The king cannot move into a threatened square
    result.retain(|x| !x.get_is_threatened(board, team_color));
    result
}

/// Gives back the possible castle moves from a BoardCastleState. This does check neither the kings
/// position nor the rooks position, so giving a wrong BoardCastleState will probably result in an
/// error.
pub fn get_castle_moves(
    castle_state: &BoardCastleState,
    team: &PieceColor,
    board: &Board,
) -> Vec<CastleMove> {
    let mut result: Vec<CastleMove> = vec![];
    // This is probably not optimal but it works.
    // TODO: Simplify this
    // First we match the team so we can give back only the castle moves of a specific team.
    match team {
        PieceColor::Light => {
            // TODO: Simplify this using a function
            if castle_state.light_queen_side
                //&& board.is_threatened((4, 0).into()) == 0 This check is redundant since the check_move_gen will never call this function.
                // And if a piece is in the way
                && no_piece_in_the_way(board, (3, 0).into(), Directions::W, 3)
                // We have to check if one of the squares is threatened
                && board.get_threatened_state((3, 0).into()).threatened_dark == 0
                && board.get_threatened_state((2, 0).into()).threatened_dark == 0
            {
                result.push(CastleMove {
                    move_type: CastleMoveType::LightQueenSide,
                })
            }
            if castle_state.light_king_side
                && no_piece_in_the_way(board, (5, 0).into(), Directions::E, 2)
                && board.get_threatened_state((5, 0).into()).threatened_dark == 0
                && board.get_threatened_state((6, 0).into()).threatened_dark == 0
            {
                result.push(CastleMove {
                    move_type: CastleMoveType::LightKingSide,
                })
            }
        }
        PieceColor::Dark => {
            if castle_state.dark_queen_side
                && no_piece_in_the_way(board, (3, 7).into(), Directions::W, 3)
                && board.get_threatened_state((3, 7).into()).threatened_light == 0
                && board.get_threatened_state((4, 7).into()).threatened_light == 0
            {
                result.push(CastleMove {
                    move_type: CastleMoveType::DarkQueenSide,
                })
            }
            if castle_state.dark_king_side
                && no_piece_in_the_way(board, (5, 7).into(), Directions::E, 2)
                && board.get_threatened_state((5, 7).into()).threatened_light == 0
                && board.get_threatened_state((6, 7).into()).threatened_light == 0
            {
                result.push(CastleMove {
                    move_type: CastleMoveType::DarkKingSide,
                })
            }
        }
    }
    result
}

/// Returns the possible diagonal moves of a piece with the given coordinates as a vector of
/// coordinates, also checks whether there are pieces in the way. An example of a piece that moves
/// this way is a bishop.
pub fn diagonal_moves(
    start: Coordinate,
    board: &board::Board,
    team_color: PieceColor,
) -> Vec<BasicMove> {
    // Create a vector that will be returned at the end.
    let mut result: Vec<BasicMove> = Vec::new();

    // Bind the starting coordinates to variables
    let from_x = start.get_x();
    let from_y = start.get_y();

    // Explore the moves in all directions.
    result.append(&mut explore_direction(
        Directions::NW,
        from_x,
        from_y,
        team_color,
        board,
    ));
    result.append(&mut explore_direction(
        Directions::NE,
        from_x,
        from_y,
        team_color,
        board,
    ));
    result.append(&mut explore_direction(
        Directions::SE,
        from_x,
        from_y,
        team_color,
        board,
    ));
    result.append(&mut explore_direction(
        Directions::SW,
        from_x,
        from_y,
        team_color,
        board,
    ));
    result
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ecr_formats::fen::*;

    use crate::board::Board;
    use crate::pieces::PieceType;

    use super::*;
    mod movement {
        use super::*;
        #[test]
        fn test_linear_moves() {
            let board = board::Board::default();
            let result = linear_moves((4, 3).into(), &board, PieceColor::Light);
            // Make a new Vector and fill it with all possible Coordinates
            let expected: Vec<BasicMove> = vec![
                // North
                BasicMove {
                    to: (4, 4).into(),
                    capture: None,
                },
                BasicMove {
                    to: (4, 5).into(),
                    capture: None,
                },
                BasicMove {
                    to: (4, 6).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (4, 6).into(),
                    }),
                },
                // East
                BasicMove {
                    to: (5, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (6, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (7, 3).into(),
                    capture: None,
                },
                // South
                BasicMove {
                    to: (4, 2).into(),
                    capture: None,
                },
                // West
                BasicMove {
                    to: (3, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (2, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (1, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (0, 3).into(),
                    capture: None,
                },
            ];

            assert_eq!(expected, result);

            let gotc: Board =
                Fen::from_str("r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R b - - 3 17")
                    .unwrap()
                    .into();
            let moves_a1 = linear_moves((0, 7).into(), &gotc, PieceColor::Dark);
            let expected_moves_a1: Vec<BasicMove> = vec![
                BasicMove {
                    to: (1, 7).into(),
                    capture: None,
                },
                BasicMove {
                    to: (2, 7).into(),
                    capture: None,
                },
                BasicMove {
                    to: (3, 7).into(),
                    capture: None,
                },
            ];
            assert_eq!(expected_moves_a1, moves_a1);
        }

        #[test]
        fn test_explore_diagonal_moves() {
            let empty_board = board::Board::empty();
            // Calculate the moves in the North-east (upper-right) direction from 3,2(d3)
            let result = explore_direction(Directions::NE, 3, 2, PieceColor::Light, &empty_board);
            let expected: Vec<BasicMove> = vec![
                BasicMove {
                    to: (4, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (5, 4).into(),
                    capture: None,
                },
                BasicMove {
                    to: (6, 5).into(),
                    capture: None,
                },
                BasicMove {
                    to: (7, 6).into(),
                    capture: None,
                },
            ];
            assert_eq!(expected, result);

            // Do the same for the North-west (upper-left) direction from h1
            let result2 = explore_direction(Directions::NW, 7, 0, PieceColor::Dark, &empty_board);
            let expected2: Vec<BasicMove> = vec![
                BasicMove {
                    to: (6, 1).into(),
                    capture: None,
                },
                BasicMove {
                    to: (5, 2).into(),
                    capture: None,
                },
                BasicMove {
                    to: (4, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (3, 4).into(),
                    capture: None,
                },
                BasicMove {
                    to: (2, 5).into(),
                    capture: None,
                },
                BasicMove {
                    to: (1, 6).into(),
                    capture: None,
                },
                BasicMove {
                    to: (0, 7).into(),
                    capture: None,
                },
            ];
            assert_eq!(expected2, result2);

            // Now do the whole thing with a filled board in the direction of NW (upper left) from e3
            // The fen string for the bishop from this position would be: 'rnbqkbnr/pppppppp/8/8/8/4B3/PPPPPPPP/RNBQKBNR w KQkq - 0 1'
            let default_board = Board::default();
            let result3 =
                explore_direction(Directions::NW, 4, 2, PieceColor::Light, &default_board);
            let expected3: Vec<BasicMove> = vec![
                BasicMove {
                    to: (3, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (2, 4).into(),
                    capture: None,
                },
                BasicMove {
                    to: (1, 5).into(),
                    capture: None,
                },
                BasicMove {
                    to: (0, 6).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (0, 6).into(),
                    }),
                },
            ];
            assert_eq!(expected3, result3);

            // This should be empty as there are only two of our own pieces in that direction.
            let result4 =
                explore_direction(Directions::SE, 3, 2, PieceColor::Light, &default_board);
            let expected4: Vec<BasicMove> = vec![];
            assert_eq!(expected4, result4);
        }

        #[test]
        fn test_diagonal_moves() {
            let board = Board::empty();
            let result = diagonal_moves((4, 3).into(), &board, PieceColor::Dark);
            let expected: Vec<BasicMove> = vec![
                // North-west (upper left)
                BasicMove {
                    to: (3, 4).into(),
                    capture: None,
                },
                BasicMove {
                    to: (2, 5).into(),
                    capture: None,
                },
                BasicMove {
                    to: (1, 6).into(),
                    capture: None,
                },
                BasicMove {
                    to: (0, 7).into(),
                    capture: None,
                },
                // North-east (upper right)
                BasicMove {
                    to: (5, 4).into(),
                    capture: None,
                },
                BasicMove {
                    to: (6, 5).into(),
                    capture: None,
                },
                BasicMove {
                    to: (7, 6).into(),
                    capture: None,
                },
                // South-east (lower right)
                BasicMove {
                    to: (5, 2).into(),
                    capture: None,
                },
                BasicMove {
                    to: (6, 1).into(),
                    capture: None,
                },
                BasicMove {
                    to: (7, 0).into(),
                    capture: None,
                },
                // South-west (lower left)
                BasicMove {
                    to: (3, 2).into(),
                    capture: None,
                },
                BasicMove {
                    to: (2, 1).into(),
                    capture: None,
                },
                BasicMove {
                    to: (1, 0).into(),
                    capture: None,
                },
            ];
            assert_eq!(expected, result);
            let result2 = diagonal_moves((3, 4).into(), &Default::default(), PieceColor::Light);
            let expected2: Vec<BasicMove> = vec![
                // upper-left
                BasicMove {
                    to: (2, 5).into(),
                    capture: None,
                },
                BasicMove {
                    to: (1, 6).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (1, 6).into(),
                    }),
                },
                // upper-right
                BasicMove {
                    to: (4, 5).into(),
                    capture: None,
                },
                BasicMove {
                    to: (5, 6).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (5, 6).into(),
                    }),
                },
                // lower-right
                BasicMove {
                    to: (4, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (5, 2).into(),
                    capture: None,
                },
                // lower-left
                BasicMove {
                    to: (2, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (1, 2).into(),
                    capture: None,
                },
            ];
            assert_eq!(expected2, result2);
        }

        #[test]
        fn test_pawn_moves() {
            let default_board = board::Board::default();
            let result = pawn_moves((0, 1).into(), &default_board, PieceColor::Light, false);
            let expected = vec![
                BasicMove {
                    to: (0, 2).into(),
                    capture: None,
                },
                BasicMove {
                    to: (0, 3).into(),
                    capture: None,
                },
            ];
            assert_eq!(expected, result);

            let result2 = pawn_moves((2, 5).into(), &default_board, PieceColor::Light, false);
            let expected2 = vec![
                BasicMove {
                    to: (1, 6).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (1, 6).into(),
                    }),
                },
                BasicMove {
                    to: (3, 6).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (3, 6).into(),
                    }),
                },
            ];
            assert_eq!(expected2, result2);

            let result3 = pawn_moves((7, 1).into(), &default_board, PieceColor::Light, true);
            let expected3 = vec![BasicMove {
                to: (7, 2).into(),
                capture: None,
            }];
            assert_eq!(expected3, result3);

            let result4 = pawn_moves((0, 6).into(), &default_board, PieceColor::Light, true);
            let expected4 = vec![BasicMove {
                to: (1, 7).into(),
                capture: Some(Capture {
                    piece_type: PieceType::Knight,
                    target: (1, 7).into(),
                }),
            }];
            assert_eq!(expected4, result4);
        }

        #[test]
        fn test_knight_moves() {
            let default_board = board::Board::default();
            let result = knight_moves((3, 3).into(), &default_board, PieceColor::Light);
            let expected: Vec<BasicMove> = vec![
                BasicMove {
                    to: (5, 2).into(),
                    capture: None,
                },
                BasicMove {
                    to: (5, 4).into(),
                    capture: None,
                },
                BasicMove {
                    to: (4, 5).into(),
                    capture: None,
                },
                BasicMove {
                    to: (2, 5).into(),
                    capture: None,
                },
                BasicMove {
                    to: (1, 4).into(),
                    capture: None,
                },
                BasicMove {
                    to: (1, 2).into(),
                    capture: None,
                },
            ];
            assert_eq!(expected, result);
            let result2 = knight_moves((3, 2).into(), &default_board, PieceColor::Dark);
            let expected2: Vec<BasicMove> = vec![
                BasicMove {
                    to: (5, 1).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (5, 1).into(),
                    }),
                },
                BasicMove {
                    to: (5, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (4, 4).into(),
                    capture: None,
                },
                BasicMove {
                    to: (2, 4).into(),
                    capture: None,
                },
                BasicMove {
                    to: (1, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (1, 1).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (1, 1).into(),
                    }),
                },
                BasicMove {
                    to: (2, 0).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Bishop,
                        target: (2, 0).into(),
                    }),
                },
                BasicMove {
                    to: (4, 0).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::King,
                        target: (4, 0).into(),
                    }),
                },
            ];
            assert_eq!(expected2, result2);
        }

        #[test]
        fn test_king_moves() {
            let result = king_moves((4, 0).into(), &Default::default(), PieceColor::Light);
            let expected: Vec<BasicMove> = vec![];
            assert_eq!(expected, result);
            let result2 = king_moves((4, 2).into(), &Default::default(), PieceColor::Light);
            let expected2: Vec<BasicMove> = vec![
                BasicMove {
                    to: (5, 2).into(),
                    capture: None,
                },
                BasicMove {
                    to: (5, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (4, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (3, 3).into(),
                    capture: None,
                },
                BasicMove {
                    to: (3, 2).into(),
                    capture: None,
                },
            ];
            assert_eq!(expected2, result2);

            let result3 = king_moves((4, 0).into(), &Default::default(), PieceColor::Dark);
            let expected3: Vec<BasicMove> = vec![
                BasicMove {
                    to: (5, 0).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Bishop,
                        target: (5, 0).into(),
                    }),
                },
                BasicMove {
                    to: (5, 1).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (5, 1).into(),
                    }),
                },
                BasicMove {
                    to: (4, 1).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (4, 1).into(),
                    }),
                },
                BasicMove {
                    to: (3, 1).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (3, 1).into(),
                    }),
                },
                BasicMove {
                    to: (3, 0).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Queen,
                        target: (3, 0).into(),
                    }),
                },
            ];
            assert_eq!(expected3, result3);
        }

        #[test]
        fn test_get_castle_moves() {
            let default_board = board::Board::default();
            let result = get_castle_moves(
                default_board.get_castle_state(),
                &PieceColor::Dark,
                &default_board,
            );
            let expected: Vec<CastleMove> = vec![];
            assert_eq!(expected, result);
        }
    }
    mod basic_move {
        use super::*;
        #[test]
        fn test_new_move() {
            let to: Coordinate = (1, 0).into();
            let basic_move = BasicMove::new_move((1, 0).into());
            assert!(basic_move.get_capture().is_none());
            assert_eq!(to, basic_move.to);
        }

        #[test]
        fn test_new_capture() {
            let to: Coordinate = (1, 0).into();
            let basic_move = BasicMove::new_capture(to, PieceType::Bishop);
            assert_eq!(to, basic_move.to);
            assert_eq!(
                PieceType::Bishop,
                basic_move.get_capture().unwrap().piece_type
            );
            assert_eq!(to, basic_move.get_capture().unwrap().target);
        }

        #[test]
        fn test_new_en_passant() {
            let to: Coordinate = (4, 4).into();
            let target: Coordinate = (5, 5).into();
            let basic_move = BasicMove::new_en_passant((4, 4).into(), (5, 5).into());
            assert_eq!(to, basic_move.to);
            assert_eq!(target, basic_move.get_capture().unwrap().target);
        }
    }
}
