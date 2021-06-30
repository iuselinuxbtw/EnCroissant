use ecr_shared::coordinate::Coordinate;

use crate::board::Board;
use crate::move_gen::BasicMove;
use crate::pieces::PieceType;

/// The type of a move. Can contain various information about
#[derive(Debug, PartialEq, Clone)]
pub enum MoveType {
    Move {
        from: Coordinate,
        to: Coordinate,
    },
    Capture {
        from: Coordinate,
        to: Coordinate,
        capture_at: Coordinate,
        en_passant: bool,
    },
    Castle {
        king_from: Coordinate,
        queen_side: bool,
    },
}

/// Represents the possible Moves of a Piece on the board with the starting coordinate of that
/// piece.
#[derive(Debug, PartialEq, Clone)]
pub struct Moves {
    pub from: Coordinate,
    pub basic_move: Vec<BasicMove>,
}

impl Moves {
    /// Returns whether the moves of a piece contain a check(If the piece could capture the king if nothing is done)
    pub fn contains_check(&self, board: &Board) -> bool {
        // Do every possible move and test whether the board where the move is done has a move where the king could be captured
        for mv in self.basic_move.clone() {
            let board_clone = board.move_on_cloned_board(self.from, &mv);
            let inner = board_clone.get_at(mv.to).unwrap();
            let color = inner.as_ref().borrow().get_color();
            // We need to get the moves in the future
            let new_move = inner
                .as_ref()
                .borrow()
                .get_piece()
                .get_pseudo_legal_moves(board, mv.to, color, true);
            // Turn it into a Moves
            let new_moves = Moves {
                from: mv.to,
                basic_move: new_move,
            };
            if new_moves.contains_king() {
                return true;
            }
        }
        false
    }

    /// Returns whether Moves contains a move that would capture the king
    fn contains_king(&self) -> bool {
        // This could be made with a iterator
        for basic_move in self.basic_move.clone() {
            if let Some(capture) = basic_move.capture {
                if capture.piece_type == PieceType::King {
                    return true;
                }
            }
        }
        false
    }

    /// Removes all illegal moves from the Basic_Moves
    pub fn remove_illegal_moves(&mut self, board: &Board) {
        // FIXME: This returns less moves than are possible
        let cloned_board = board.clone();
        // We have to iterate from the highest index to the lowest since we want to remove moves
        for i in (0..self.basic_move.len()).rev() {
            // If the Move is illegal we want to remove it from the vector.
            if !cloned_board.check_if_legal_move(self.from, &self.basic_move[i]) {
                self.basic_move.remove(i);
            }
        }
    }

    /// Removes all captures from Moves. Could be written in a more functional style
    /// just as well...
    pub fn remove_captures(&mut self) {
        for i in (0..self.basic_move.len()).rev() {
            if self.basic_move[i].capture.is_some() {
                self.basic_move.remove(i);
            }
        }
    }
}

/// Represents a move. Can be used to modify the positions of pieces on the board. Does not do any
/// validity detection and just holds the move that should be done.
#[derive(Debug, PartialEq, Clone)]
pub struct Move {
    pub move_type: MoveType,
    pub promotion: Option<PieceType>,
    pub check: bool,
    pub check_mate: bool,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ecr_formats::fen::Fen;

    use super::*;
    use ecr_shared::pieces::PieceColor;

    #[test]
    fn test_contains_check() {
        let board: Board =
            (Fen::from_str("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2"))
                .unwrap()
                .into();
        let multiple_moves = board.get_all_pseudo_legal_moves();
        let mut checks = 0;
        for moves in multiple_moves {
            if moves.contains_check(&board) {
                checks += 1;
            }
        }
        assert_eq!(1, checks);

        let board_2: Board = (Fen::from_str("1k6/8/8/8/2r5/8/8/3KR2r b - - 0 1"))
            .unwrap()
            .into();
        let multiple_moves_2 = board_2.get_pseudo_legal_moves();
        assert_eq!(3, multiple_moves_2.len());

        //FIXME: This should not be None
        assert!(board_2.get_at(multiple_moves_2[1].from).is_some());

        checks = 0;
        // Go through the black moves
        for moves in multiple_moves_2 {
            //FIXME: One of the moves here doesnt have a starting square apparently
            if moves.contains_check(&board_2) {
                checks += 1;
            }
        }

        let rook_1_coordinate = (2, 3).into();
        let rook_1_moves = board_2
            .get_at(rook_1_coordinate)
            .unwrap()
            .borrow()
            .get_piece()
            .get_pseudo_legal_moves(&board_2, rook_1_coordinate, PieceColor::Dark, true);
        assert!(Moves {
            from: (2, 3).into(),
            basic_move: rook_1_moves
        }
        .contains_check(&board_2));

        let rook_2_coordinate = (7, 0).into();
        let rook_2 = board_2.get_at(rook_2_coordinate).unwrap();
        let rook_2_moves = rook_2.borrow().get_piece().get_pseudo_legal_moves(
            &board_2,
            rook_2_coordinate,
            PieceColor::Dark,
            true,
        );

        assert!(Moves {
            from: rook_2_coordinate,
            basic_move: rook_2_moves
        }
        .contains_check(&board_2));
        assert_eq!(2, checks);
    }

    #[test]
    fn test_remove_illegal_moves() {
        let board: Board = Fen::from_str("1k6/8/8/8/8/8/8/3KR2r w - - 0 1")
            .unwrap()
            .into();
        let piece_coordinate: Coordinate = (4, 0).into();
        let mut moves: Moves = Moves {
            from: piece_coordinate,
            basic_move: board
                .get_at(piece_coordinate)
                .unwrap()
                .borrow()
                .get_piece()
                .get_pseudo_legal_moves(&board, piece_coordinate, PieceColor::Light, true),
        };

        // The Rook has 10 pseudo-legal moves, 4 of which are legal
        assert_eq!(10, moves.basic_move.len());
        moves.remove_illegal_moves(&board);
        assert_eq!(3, moves.basic_move.len());
        assert!(!moves.contains_check(&board));
        let legal_moves = vec![
            BasicMove::new_move((5, 0).into()),
            BasicMove::new_move((6, 0).into()),
            BasicMove::new_capture((7, 0).into(), PieceType::Rook),
        ];
        for m in &legal_moves {
            assert!(moves.basic_move.contains(m));
        }
        assert_eq!(legal_moves.len(), moves.basic_move.len())
    }
}
