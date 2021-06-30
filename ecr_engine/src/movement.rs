use std::borrow::Borrow;
use std::ops::Deref;

use ecr_shared::coordinate::Coordinate;

use crate::board;
use crate::board::{Board, SquareInner};
use crate::move_gen::{BasicMove, Capture, CastleMove, CastleMoveType};
use crate::pieces::{BoardPiece, PieceColor, PieceType};
use crate::r#move::Moves;
use ecr_shared::board::BoardCastleState;

struct MoveProperties {
    inner: SquareInner,
    piece_type: PieceType,
    target_square: Coordinate,
    capture: Option<Capture>,
    en_passant: bool,
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
        let mut en_passant = false;

        if let PieceType::Pawn = piece_type {
            // But if it is a pawn we do kinda wanna promote our piece
            promotion = board.is_pawn_promotion(target_square);
            en_passant = board.check_en_passant(target_square);
        }
        // And lastly we return the complete MoveProperties
        MoveProperties {
            inner,
            piece_type,
            target_square,
            capture,
            en_passant,
            promotion,
        }
    }
}

impl board::Board {
    /// This function moves a piece from a given start square to another square, contained in a
    /// BasicMove. Note: This function doesn't complain if a piece by the wrong team is moved.
    pub fn do_blunder(&mut self, start: Coordinate, basic_move: &BasicMove) {
        let move_properties = MoveProperties::get_properties(*basic_move, self.clone(), start);

        self.pre_move(start, &move_properties.inner);

        // Update the piece coordinate to the new coordinates.
        move_properties
            .inner
            .borrow_mut()
            .set_coordinate(move_properties.target_square);

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

        if move_properties.capture.is_some() {
            let mut target = move_properties.capture.unwrap().target;
            if move_properties.en_passant {
                // We can safely unwrap here since en_passant is only true if  en_passant is possible.
                target = self.get_en_passant_target().unwrap().actual_square;
            }
            self.capture_piece(&move_properties.inner, target);
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
        self.to_move = self.to_move.get_opponent();
        // Calculate all new threats (This could probably be simplified)
        self.calculate_threatened_states();
        // Check if the move is legal
        // TODO: Add to move Vector
        // TODO: Update castle_state
    }

    pub(crate) fn move_on_cloned_board(&self, start: Coordinate, basic_move: &BasicMove) -> Board {
        let mut cloned_board = self.clone();
        cloned_board.do_blunder(start, basic_move);
        return cloned_board;
    }

    // This function contains stuff that has to be done before every move
    fn pre_move(&mut self, start: Coordinate, inner: &SquareInner) {
        // Reset all ThreatenedState
        self.remove_all_threats();

        // First we remove the piece from the original square on the board.
        self.remove_piece(start);
        // TODO: Directly replace the piece in the vector
        self.pieces.retain(|piece| piece != inner);
    }

    /// This checks if a move by a pawn is en_passant. We need this so we can then capture the pawn
    /// on another square.
    pub fn check_en_passant(&self, target: Coordinate) -> bool {
        if let Some(coordinate) = self.get_en_passant_target() {
            if coordinate.target_square == target {
                return true;
            }
        }
        false
    }

    // TODO: We need a test for this which should be some mid-game board.
    /// Executes a given CastleMove by moving the king first and then the rook
    pub fn castle(&mut self, castle_move: CastleMove) {
        // First we move the king to the target square.
        match castle_move.move_type {
            CastleMoveType::LightKingSide => {
                // Move the king
                self.do_blunder((4, 0).into(), &BasicMove::new_move((6, 0).into()));
                // Move the rook
                self.do_blunder(
                    (7, 0).into(),
                    &BasicMove {
                        capture: None,
                        to: (4, 0).into(),
                    },
                );
            }
            CastleMoveType::LightQueenSide => {
                self.do_blunder((4, 0).into(), &BasicMove::new_move((2, 0).into()));
                self.do_blunder(
                    (0, 0).into(),
                    &BasicMove {
                        capture: None,
                        to: (0, 3).into(),
                    },
                );
            }
            CastleMoveType::DarkKingSide => {
                self.do_blunder((4, 7).into(), &BasicMove::new_move((6, 7).into()));
                self.do_blunder(
                    (7, 7).into(),
                    &BasicMove {
                        capture: None,
                        to: (5, 7).into(),
                    },
                );
            }
            CastleMoveType::DarkQueenSide => {
                self.do_blunder((4, 7).into(), &BasicMove::new_move((2, 7).into()));
                self.do_blunder(
                    (0, 7).into(),
                    &BasicMove {
                        capture: None,
                        to: (3, 0).into(),
                    },
                );
            }
        }
        self.half_move_amount -= 1;
        self.move_number -= 1;
        self.castle_state = BoardCastleState::empty();
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

    /// Returns the pseudo legal moves of the current team
    pub fn get_pseudo_legal_moves(&self) -> Vec<Moves> {
        self.get_pseudo_legal_moves_util(self.to_move)
    }

    /// This function returns all possible pseudo legal moves (OF BOTH TEAMS!).
    ///
    /// We could also only get one move and bet on it being the best one which would certainly be
    /// interesting...
    pub(crate) fn get_all_pseudo_legal_moves(&self) -> Vec<Moves> {
        let mut result: Vec<Moves> = vec![];
        result.append(&mut self.get_pseudo_legal_moves_util(PieceColor::Light));
        result.append(&mut self.get_pseudo_legal_moves_util(PieceColor::Dark));
        result
    }

    /// Returns the pseudo-legal moves of a specific team.
    pub fn get_pseudo_legal_moves_util(&self, team_color: PieceColor) -> Vec<Moves> {
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
                    self,
                    // These calls seem kinda dumb and i don't know why we need the first deref now but it works fine. If Anyone wants to improve them please do so.
                    square_inner.deref().borrow().borrow().get_coordinate(),
                    square_inner.deref().borrow().borrow().get_color(),
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

    /// Returns true if the given team is currently checking the other team
    fn check_checker(&self, team: PieceColor) -> bool {
        let all_moves: Vec<Moves> = self.get_pseudo_legal_moves_util(team);
        for moves in all_moves {
            if moves.contains_check(self) {
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
        for moves in self.get_pseudo_legal_moves_util(team_color) {
            for do_blunder in moves.basic_move {
                self.add_threat(do_blunder.to, team_color);
            }
        }
    }

    /// We should not filter our normal move_gen for legal moves if we are checked, since that would
    /// be inefficient. We can make a special move generator for legal moves during being checked.
    pub fn check_move_gen(&self) -> Vec<BasicMove> {
        todo!()
    }

    /// Returns true if the move is legal, false if it is illegal.
    pub fn check_if_legal_move(&self, start: Coordinate, basic_move: &BasicMove) -> bool {
        // TODO: Testing
        // Clone the current board
        let mut future_board = self.clone();
        // Do the move in the cloned board
        future_board.do_blunder(start, basic_move);
        // Check if the the king can be captured by the team that can currently move.
        // We need to invert the result since moves where the opponent does not have check after are legal.
        !future_board.check_checker(future_board.to_move)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod board {
        use std::ops::Deref;
        use std::str::FromStr;

        use ecr_formats::fen::Fen;

        use crate::board::Board;
        use crate::move_gen::BasicMove;
        use crate::pieces::PieceColor;

        use super::*;

        #[test]
        fn test_movement() {
            let mut default_board = Board::default();
            default_board.do_blunder(
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
            assert_eq!(
                "rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b KQkq - 0 1".to_string(),
                Fen::from(default_board.clone()).to_string()
            );
            assert!(default_board.get_at((7, 3).into()).is_some());
            default_board.do_blunder(
                (6, 6).into(),
                &BasicMove {
                    to: (6, 4).into(),
                    capture: None,
                },
            );
            assert_eq!(
                "rnbqkbnr/pppppp1p/8/6p1/7P/8/PPPPPPP1/RNBQKBNR w KQkq - 0 2".to_string(),
                Fen::from(default_board.clone()).to_string()
            );
            default_board.do_blunder(
                (7, 0).into(),
                &BasicMove {
                    to: (7, 2).into(),
                    capture: None,
                },
            );
            // TODO: The light king can't castle kingside here, but for now this has to work.
            assert_eq!(
                "rnbqkbnr/pppppp1p/8/6p1/7P/7R/PPPPPPP1/RNBQKBN1 b KQkq - 1 2".to_string(),
                Fen::from(default_board.clone()).to_string()
            );
            default_board.do_blunder(
                (6, 4).into(),
                &BasicMove {
                    to: (7, 3).into(),
                    capture: Some(Capture {
                        piece_type: PieceType::Pawn,
                        target: (7, 3).into(),
                    }),
                },
            );
            // TODO: The King can't castle Kingside here
            assert_eq!(
                "rnbqkbnr/pppppp1p/8/8/7p/7R/PPPPPPP1/RNBQKBN1 w KQkq - 0 3".to_string(),
                Fen::from(default_board.clone()).to_string()
            );

            assert!(!default_board.clone().get_en_passant_target().is_some());
            assert_eq!(None, default_board.get_at((6, 4).into()));
            assert!(!default_board.check_checker(PieceColor::Light));
            assert!(!default_board.check_checker(PieceColor::Dark));
            default_board = Board::default();

            assert!(default_board.get_at((5, 1).into()).is_some());
            // The best opening move known to mankind
            default_board.do_blunder(
                (5, 1).into(),
                &BasicMove {
                    to: (5, 2).into(),
                    capture: None,
                },
            )

            // TODO: Test Promotion
        }

        #[test]
        fn test_capture() {
            let mut default_board = Board::default();
            let pawn = default_board.get_at((0, 1).into()).unwrap();
            default_board.capture_piece(&pawn, (0, 1).into());
            assert_eq!(None, default_board.get_at((0, 1).into()));
        }

        #[test]
        fn test_check_checker() {
            let default_board = Board::default();
            let mut light_check = default_board.check_checker(PieceColor::Light);
            let dark_check = default_board.check_checker(PieceColor::Dark);
            assert!(!(light_check || dark_check));
            let check_board: Board =
                Board::from(Fen::from_str("2k5/8/8/8/8/2R5/8/2K5 b - - 3 6").unwrap());
            light_check = check_board.check_checker(PieceColor::Light);
            assert_eq!(true, light_check);
        }

        #[test]
        fn test_get_pseudo_legal_moves() {
            let board: Board = (Fen::from_str("1k6/8/8/8/2r5/8/8/3KR2r b - - 0 1"))
                .unwrap()
                .into();
            let moves = board.get_pseudo_legal_moves();
            let piece_positions = vec![(1, 7).into(), (2, 3).into(), (7, 0).into()];
            for m in &moves {
                assert!(piece_positions.contains(&m.from));
                assert!(board.get_at(m.from).is_some());
            }
            assert_eq!(3, moves.len());
        }

        /*#[test]
        fn test_check_if_legal_move() {
            let mut default_board = Board::default();
            todo!()
        }*/

        #[test]
        fn test_count_half_moves() {
            let mut board = Board::default();
            board.count_half_moves(PieceType::Pawn, false);
            assert_eq!(0, board.get_half_move_amount());
            board.count_half_moves(PieceType::Bishop, false);
            assert_eq!(1, board.get_half_move_amount());
            board.count_half_moves(PieceType::Queen, false);
            assert_eq!(2, board.get_half_move_amount());
            board.count_half_moves(PieceType::Queen, true);
            assert_eq!(0, board.get_half_move_amount());
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
