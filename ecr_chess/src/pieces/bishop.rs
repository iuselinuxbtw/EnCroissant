use crate::pieces::{PieceColor, PieceType};

use super::Piece;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::pieces::move_gen::{BasicMove, diagonal_moves};

#[derive(Debug, PartialEq, Clone)]
pub struct Bishop {}

impl Piece for Bishop {
    fn get_type(&self) -> PieceType {
        PieceType::Bishop
    }
    fn get_pseudo_legal_moves(
        &self,
        board: &Board,
        piece_coordinate: &Coordinate,
        piece_color: &PieceColor,
        #[allow(unused_variables)]
        has_moved: bool,
    ) -> Vec<BasicMove> {
        diagonal_moves(piece_coordinate, board, piece_color)
    }

    fn get_value(&self) -> f32 {
        3.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_piece() -> Bishop {
        Bishop {}
    }

    #[test]
    fn test_get_shortcode_algebraic() {
        assert_eq!("B", get_piece().get_shortcode_algebraic());
    }

    #[test]
    fn test_get_type() {
        assert_eq!(PieceType::Bishop, get_piece().get_type());
    }
}
