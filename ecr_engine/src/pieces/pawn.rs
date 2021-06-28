use ecr_shared::coordinate::Coordinate;

use crate::board::Board;
use crate::move_gen::move_gen::pawn_moves;
use crate::pieces::{PieceColor, PieceType};

use super::Piece;
use crate::move_gen::BasicMove;

#[derive(Debug, PartialEq, Clone)]
pub struct Pawn {}

impl Piece for Pawn {
    fn get_type(&self) -> PieceType {
        PieceType::Pawn
    }
    fn get_pseudo_legal_moves(
        &self,
        board: &Board,
        piece_coordinate: Coordinate,
        piece_color: PieceColor,
        has_moved: bool,
    ) -> Vec<BasicMove> {
        pawn_moves(piece_coordinate, board, piece_color, has_moved)
    }

    fn get_value(&self) -> u8 {
        10
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
