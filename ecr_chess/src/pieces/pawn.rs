use crate::pieces::PieceType;

use super::Piece;

#[derive(Debug, PartialEq, Clone)]
pub struct Pawn {}

impl Piece for Pawn {
    fn get_type(&self) -> PieceType {
        PieceType::Pawn
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