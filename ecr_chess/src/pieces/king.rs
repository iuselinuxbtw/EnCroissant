use crate::pieces::{PieceType, PieceColor};

use super::Piece;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::pieces::move_gen::{BasicMove, king_moves};

#[derive(Debug, PartialEq, Clone)]
pub struct King {}

impl Piece for King {
    fn get_type(&self) -> PieceType {
        PieceType::King
    }
    fn get_pseudo_legal_moves(
        &self,
        board: &Board,
        piece_coordinate: &Coordinate,
        piece_color: &PieceColor,
        has_moved: bool,
    ) -> Vec<BasicMove> {
        king_moves(piece_coordinate, board, piece_color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_piece() -> King {
        King {}
    }

    #[test]
    fn test_get_shortcode_algebraic() {
        assert_eq!("K", get_piece().get_shortcode_algebraic());
    }

    #[test]
    fn test_get_type() {
        assert_eq!(PieceType::King, get_piece().get_type());
    }
}
