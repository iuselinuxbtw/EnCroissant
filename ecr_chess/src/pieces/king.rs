use super::Piece;
use crate::pieces::PieceType;

#[derive(Debug, PartialEq, Clone)]
pub struct King {}

impl Piece for King {
    fn get_type(&self) -> PieceType {
        PieceType::King
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