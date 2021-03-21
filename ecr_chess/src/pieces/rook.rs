use crate::pieces::PieceType;

use super::Piece;

#[derive(Debug, PartialEq, Clone)]
pub struct Rook {}

impl Piece for Rook {
    fn get_type(&self) -> PieceType {
        PieceType::Rook
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