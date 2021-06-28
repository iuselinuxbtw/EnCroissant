use ecr_shared::coordinate::Coordinate;

use crate::board::Board;
use crate::move_gen::move_gen::linear_moves;
use crate::pieces::{PieceColor, PieceType};

use super::Piece;
use crate::move_gen::BasicMove;

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
        piece_color: PieceColor,
        #[allow(unused_variables)] has_moved: bool,
    ) -> Vec<BasicMove> {
        linear_moves(piece_coordinate, board, piece_color)
    }

    fn get_value(&self) -> u8 {
        50
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
