use crate::pieces::PieceType;

use super::Piece;

#[derive(Debug, PartialEq, Clone)]
pub struct Knight {}

impl Piece for Knight {
    fn get_type(&self) -> PieceType {
        PieceType::Knight
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