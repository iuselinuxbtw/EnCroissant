use std::borrow::Borrow;
use std::ops::Deref;

use ecr_shared::coordinate::Coordinate;

use crate::board;
use crate::board::{Board, SquareInner};
use crate::pieces::{BoardPiece, PieceColor, PieceType};
use crate::pieces::move_gen::{BasicMove, CastleMove, CastleMoveType};
use crate::r#move::Moves;

struct MoveProperties {
    inner: SquareInner,
    piece_type: PieceType,
    target_square: Coordinate,
    capture: Option<PieceType>,
    promotion: bool,
}

impl MoveProperties {
    /// Find out the properties of a move. Useful for movement.
    fn get_properties(
        basic_move: BasicMove,
        board: board::Board,
        start: Coordinate,
    ) -> MoveProperties {
        // We can safely unwrap here since no move is generated without a piece at the start of it.
        let inner = board.get_at(start).unwrap();

        // Get the piece type
        let piece_type: PieceType = inner.deref().borrow().get_piece().get_type();

        // Get the target square
        let target_square = basic_move.get_target_square();

        // Get if it is a capture
        let capture = basic_move.get_capture();
        // By default it is no promotion
        let mut promotion = false;

        match piece_type {
            // But if it is a pawn we do kinda wanna promote our piece
            PieceType::Pawn => promotion = board.is_pawn_promotion(target_square),
            _ => {}
        }
        // And lastly we return the complete MoveProperties
        MoveProperties {
            inner,
            piece_type,
            target_square,
            capture,
            promotion,
        }
    }
}

impl board::Board {
    /// This function moves a piece from a given start square to another square, contained in a
    /// BasicMove. Note: This function doesn't complain if a piece by the wrong team is moved.
    pub fn r#move(&mut self, start: Coordinate, basic_move: &BasicMove) {
        let move_properties = MoveProperties::get_properties(*basic_move, self.clone(), start);

        self.pre_move(start);

        // Update the piece coordinate to the new coordinates.
        move_properties
            .inner
            .borrow_mut()
            .set_coordinate(move_properties.target_square);

        if move_properties.capture.is_some() {
            self.capture_piece(&move_properties.inner, move_properties.target_square);
        }

        let mut piece_to_add: BoardPiece = move_properties
            .inner
            .deref()
            .borrow()
            .borrow()
            .deref()
            .clone();
        piece_to_add.set_coordinate(move_properties.target_square);

        if move_properties.promotion {
            // TODO: We need some way to choose a different piece if we can do a promotion. For now every promotion we do is just to the queen.
            piece_to_add = BoardPiece::new_from_type(
                PieceType::Queen,
                move_properties.target_square,
                piece_to_add.get_color(),
            );
        }

        // The piece has now moved
        piece_to_add.set_has_moved();

        // Then we add the piece to the target square.
        self.add_piece(piece_to_add.clone());

        // And we of course have to increase the move number, but only if the piece is dark.
        if piece_to_add.get_color() == PieceColor::Dark {
            self.move_number += 1;
        }

        // We have to get the half moves
        self.count_half_moves(
            move_properties.piece_type,
            move_properties.capture.is_some(),
        );

        // Change the to move team
        self.light_to_move = !self.light_to_move;
        // Calculate all new threats (This could probably be simplified)
        self.calculate_threatened_states();
        // Check if the move is legal
        // TODO: Add to move Vector
        // TODO: Update castle_state
    }
    // This function contains stuff that has to be done before every move
    fn pre_move(&mut self, start: Coordinate) {
        // Reset all ThreatenedState
        self.remove_all_threats();

        // First we remove the piece from the original square on the board.
        self.remove_piece(start);
    }

    // TODO: We need a test for this which should be some mid-game board.
    /// Executes a given CastleMove by moving the king first and then the rook
    pub fn castle(&mut self, castle_move: CastleMove) {
        // First we move the king to the target square.
        // TODO: We don't actually need the to: Coordinate in the CastleMove
        match castle_move.move_type {
            CastleMoveType::LightKingSide => {
                // Move the king
                // TODO: These increase the move counter two times and add two half_moves, which should not happen.
                self.r#move(
                    (4, 0).into(),
                    &BasicMove {
                        capture: None,
                        to: castle_move.to,
                    },
                );
                // Move the rook
                self.r#move(
                    (7, 0).into(),
                    &BasicMove {
                        capture: None,
                        to: (4, 0).into(),
                    },
                );
            }
            CastleMoveType::LightQueenSide => {
                self.r#move(
                    (4, 0).into(),
                    &BasicMove {
                        capture: None,
                        to: castle_move.to,
                    },
                );
                self.r#move(
                    (0, 0).into(),
                    &BasicMove {
                        capture: None,
                        to: (0, 3).into(),
                    },
                );
            }
            CastleMoveType::DarkKingSide => {
                self.r#move(
                    (4, 7).into(),
                    &BasicMove {
                        capture: None,
                        to: castle_move.to,
                    },
                );
                self.r#move(
                    (7, 7).into(),
                    &BasicMove {
                        capture: None,
                        to: (5, 7).into(),
                    },
                );
            }
            CastleMoveType::DarkQueenSide => {
                self.r#move(
                    (4, 7).into(),
                    &BasicMove {
                        capture: None,
                        to: castle_move.to,
                    },
                );
                self.r#move(
                    (0, 7).into(),
                    &BasicMove {
                        capture: None,
                        to: (3, 0).into(),
                    },
                );
            }
        }
    }

    fn is_pawn_promotion(&self, target: Coordinate) -> bool {
        // TODO: Testing
        // Pawns can't move backwards so checking the color is redundant
        if target.get_y() == 7 || target.get_y() == 0 {
            return true;
        }
        false
    }

    /// This function is called every move and is responsible for increasing/resetting the half move counter.
    fn count_half_moves(&mut self, piece_type: PieceType, capture: bool) {
        // TODO: Testing
        match piece_type {
            PieceType::Pawn => self.half_move_amount = 0,
            _ => self.half_move_amount += 1,
        }
        if capture {
            self.half_move_amount = 0
        }
    }

    /// This function removes the piece on the given coordinate and sets it out of game.
    fn capture_piece(&mut self, target: &SquareInner, target_square: Coordinate) {
        // TODO: Testing
        target.borrow_mut().set_out_of_game();
        self.pieces.retain(|inner| inner != target);
        self.remove_piece(target_square);
    }

    /// This function returns all possible pseudo legal moves (OF BOTH TEAMS!).
    ///
    /// We could also only get one move and bet on it being the best one which would certainly be
    /// interesting...
    pub fn get_all_pseudo_legal_moves(&self) -> Vec<Moves> {
        let mut result: Vec<Moves> = vec![];
        result.append(&mut self.get_pseudo_legal_moves(PieceColor::Light));
        result.append(&mut self.get_pseudo_legal_moves(PieceColor::Dark));
        result
    }

    /// Returns the pseudo-legal moves of a specific team.
    pub fn get_pseudo_legal_moves(&self, team_color: PieceColor) -> Vec<Moves> {
        let mut result: Vec<Moves> = vec![];
        let own_pieces = self.get_all_pieces(team_color);
        result.append(&mut self.get_moves(own_pieces));
        result
    }

    /// Returns pseudo legal moves of Vector of Pieces.
    pub fn get_moves(&self, pieces: Vec<SquareInner>) -> Vec<Moves> {
        let mut result: Vec<Moves> = vec![];
        for square_inner in pieces {
            let possible_moves: Vec<BasicMove> = square_inner
                .deref()
                .borrow()
                .get_piece()
                .get_pseudo_legal_moves(
                    &self,
                    // These calls seem kinda dumb and i don't know why we need the first deref now but it works fine. If Anyone wants to improve them please do so.
                    &square_inner.deref().borrow().borrow().get_coordinate(),
                    &square_inner.deref().borrow().borrow().get_color(),
                    square_inner.deref().borrow().borrow().get_has_moved(),
                );
            // We don't want to have Pieces which cannot move in the final array.
            if !possible_moves.is_empty() {
                result.push(Moves {
                    from: square_inner.deref().borrow().borrow().get_coordinate(),
                    basic_move: possible_moves,
                })
            }
        }
        result
    }

    /// Gets all pieces of a given Team color
    fn get_all_pieces(&self, target_color: PieceColor) -> Vec<SquareInner> {
        let mut result: Vec<SquareInner> = vec![];
        for e in self.pieces.clone() {
            if e.deref().borrow().get_color() == target_color {
                result.push(e);
            }
        }
        result
    }

    /// Returns true if the given team has a check
    fn check_checker(&self, team: PieceColor) -> bool {
        let all_moves: Vec<Moves> = self.get_pseudo_legal_moves(team);
        for moves in all_moves {
            if moves.contains_check() {
                return true;
            }
        }
        false
    }

    pub fn calculate_threatened_states(&mut self) {
        self.calculate_team_threatened_state(PieceColor::Light);
        self.calculate_team_threatened_state(PieceColor::Dark);
    }

    fn calculate_team_threatened_state(&mut self, team_color: PieceColor) {
        for moves in self.get_pseudo_legal_moves(team_color) {
            for r#move in moves.basic_move {
                self.add_threat(r#move.to, team_color);
            }
        }
    }

    /// We should not filter our normal move_gen for legal moves if we are checked, since that would
    /// be inefficient. We can make a special move generator for legal moves during being checked.
    pub fn check_move_gen(&self) -> Vec<BasicMove> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod board {
        use std::ops::Deref;

        use crate::board::Board;
        use crate::pieces::move_gen::BasicMove;
        use crate::pieces::PieceColor;

        use super::*;

        #[test]
        fn test_move() {
            let mut default_board = Board::default();
            default_board.r#move(
                (7, 1).into(),
                &BasicMove {
                    to: (7, 3).into(),
                    capture: None,
                },
            );
            assert_eq!(1, default_board.get_move_number());
            assert_eq!(0, default_board.get_half_move_amount());
            assert_eq!(false, default_board.get_light_to_move());
            assert_eq!(None, default_board.get_at((7, 1).into()));
            assert!(default_board.get_at((7, 3).into()).is_some())
            // TODO: Test the Position of all pieces.
        }

        #[test]
        fn test_capture() {
            let mut default_board = Board::default();
            let pawn = default_board.get_at((0, 1).into()).unwrap();
            default_board.capture_piece(&pawn, (0, 1).into());
            assert_eq!(None, default_board.get_at((0, 1).into()));
        }

        #[test]
        fn test_get_all_pseudo_legal_moves() {
            let default_board = Board::default();
            let moves = default_board.get_all_pseudo_legal_moves();
            let result = moves.len();
            // All possible moves in the default situation are 40, but since the possible moves of a single piece are inside the same Moves structure it is (8+2)*2=20
            assert_eq!(((8 + 2) * 2), result);
        }

        #[test]
        fn test_get_all_pieces_() {
            let default_board = Board::default();
            // TODO: Check if all pieces are there. For now i will only check the number of pieces and their color
            let result = default_board.get_all_pieces(PieceColor::Light);
            for piece in &result {
                assert_eq!(
                    PieceColor::Light,
                    piece.deref().borrow().deref().get_color()
                );
            }

            let result_len = result.len();
            assert_eq!(16, result_len);
        }
    }
}
