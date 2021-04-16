use crate::pieces::{PieceType, PieceColor};

use super::Piece;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::pieces::move_gen::{BasicMove, linear_moves};

#[derive(Debug, PartialEq, Clone)]
pub struct Rook {}

impl Piece for Rook {
    fn get_type(&self) -> PieceType {
        PieceType::Rook
    }
    fn get_pseudo_legal_moves(
        &self,
        board: &Board,
        piece_coordinate: &Coordinate,
        piece_color: &PieceColor,
        has_moved: bool,
    ) -> Vec<BasicMove> {
        linear_moves(piece_coordinate, board, piece_color)
    }

    fn get_value(&self) -> f32 {
        5.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_piece() -> Rook {
        Rook {}
    }

    #[test]
    fn test_get_shortcode_algebraic() {
        assert_eq!("R", get_piece().get_shortcode_algebraic());
    }

    #[test]
    fn test_get_type() {
        assert_eq!(PieceType::Rook, get_piece().get_type());
    }
}
