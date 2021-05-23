use ecr_formats::fen::Fen;
use ecr_shared::coordinate::Coordinate;

use crate::pieces::PieceType;
use crate::{board::Board, pieces::move_gen::BasicMove};

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
        // This could be a lot cleaner but it works
        for mv in self.basic_move.clone() {
            let mut board_clone = board.clone();
            println!("{}", Fen::from(board_clone.clone()));
            // So apparently this crashes the function and i seriously don't know why
            println!("{} to: {:?}", self.from, &mv);
            board_clone.r#move(self.from, &mv);
            let inner = board_clone.get_at(mv.to).unwrap();
            let color = inner.as_ref().borrow().get_color();
            // We need to get the moves in the future
            let new_move = inner
                .as_ref()
                .borrow()
                .get_piece()
                .get_pseudo_legal_moves(board, &mv.to, color, true);
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
    fn contains_king(&self) -> bool {
        // This could be made with a iterator
        for basic_move in self.basic_move.clone() {
            // This is bs. It has to look into the future instead of this
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
        let cloned_board = board.clone();
        for i in 0..self.basic_move.len() {
            // If the Move is illegal we want to remove it from the vector.
            if !cloned_board.check_if_legal_move(self.from, &self.basic_move[i]) {
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
            };
        }
        assert_eq!(1, checks);
    }

    #[test]
    fn test_remove_illegal_moves() {
        //TODO
    }
}
