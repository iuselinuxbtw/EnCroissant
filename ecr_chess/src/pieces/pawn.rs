use crate::pieces::{PieceType, PieceColor};

use super::Piece;
use crate::pieces::move_gen::{BasicMove, pawn_moves};
use crate::coordinate::Coordinate;
use crate::board::Board;

#[derive(Debug, PartialEq, Clone)]
pub struct Pawn {}

impl Piece for Pawn {
    fn get_type(&self) -> PieceType {
        PieceType::Pawn
    }
    fn get_pseudo_legal_moves(
        &self,
        board: &Board,
        piece_coordinate: &Coordinate,
        piece_color: &PieceColor,
        has_moved: bool,
    ) -> Vec<BasicMove> {
        pawn_moves(piece_coordinate, board, piece_color, has_moved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_piece() -> Pawn {
        Pawn {}
    }

    #[test]
    fn test_get_shortcode_algebraic() {
        assert_eq!("", get_piece().get_shortcode_algebraic());
    }

    #[test]
    fn test_get_type() {
        assert_eq!(PieceType::Pawn, get_piece().get_type());
    }
}
