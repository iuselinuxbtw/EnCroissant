use crate::pieces::{PieceType, PieceColor};

use super::Piece;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::pieces::move_gen::{BasicMove, knight_moves};

#[derive(Debug, PartialEq, Clone)]
pub struct Knight {}

impl Piece for Knight {
    fn get_type(&self) -> PieceType {
        PieceType::Knight
    }
    fn get_pseudo_legal_moves(
        &self,
        board: &Board,
        piece_coordinate: &Coordinate,
        piece_color: &PieceColor,
        has_moved: bool,
    ) -> Vec<BasicMove> {
        knight_moves(piece_coordinate, board, piece_color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_piece() -> Knight {
        Knight {}
    }

    #[test]
    fn test_get_shortcode_algebraic() {
        assert_eq!("N", get_piece().get_shortcode_algebraic());
    }

    #[test]
    fn test_get_type() {
        assert_eq!(PieceType::Knight, get_piece().get_type());
    }
}
