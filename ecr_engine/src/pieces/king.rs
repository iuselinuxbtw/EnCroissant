use ecr_shared::coordinate::Coordinate;

use crate::board::Board;
use crate::move_gen::move_gen::{king_moves, BasicMove};
use crate::pieces::{PieceColor, PieceType};

use super::Piece;

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
        piece_color: PieceColor,
        #[allow(unused_variables)] has_moved: bool,
    ) -> Vec<BasicMove> {
        king_moves(piece_coordinate, board, piece_color)
    }

    fn get_value(&self) -> u8 {
        // Doesn't really matter what we put in here since we lose the game when we lose the king.
        20
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
