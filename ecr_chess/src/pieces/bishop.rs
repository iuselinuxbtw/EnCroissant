use crate::pieces::PieceType;

use super::Piece;

#[derive(Debug, PartialEq, Clone)]
pub struct Bishop {}

impl Piece for Bishop {
    fn get_type(&self) -> PieceType {
        PieceType::Bishop
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