use crate::pieces::{PieceType, PieceColor};

use super::Piece;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::pieces::move_gen::{BasicMove, linear_moves, diagonal_moves};

#[derive(Debug, PartialEq, Clone)]
pub struct Queen {}

impl Piece for Queen {
    fn get_type(&self) -> PieceType {
        PieceType::Queen
    }
    fn get_pseudo_legal_moves(
        &self,
        board: &Board,
        piece_coordinate: &Coordinate,
        piece_color: &PieceColor,
        has_moved: bool,
    ) -> Vec<BasicMove> {
        let mut result:Vec<BasicMove> = vec![];
        result.append(&mut linear_moves(piece_coordinate, board, piece_color));
        result.append(&mut diagonal_moves(piece_coordinate, board, piece_color));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_piece() -> Queen {
        Queen {}
    }

    #[test]
    fn test_get_shortcode_algebraic() {
        assert_eq!("Q", get_piece().get_shortcode_algebraic());
    }

    #[test]
    fn test_get_type() {
        assert_eq!(PieceType::Queen, get_piece().get_type());
    }
}
