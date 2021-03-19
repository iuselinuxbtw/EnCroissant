use super::Piece;
use crate::pieces::PieceType;

#[derive(Debug, PartialEq, Clone)]
pub struct Queen {}

impl Piece for Queen {
    fn get_type(&self) -> PieceType {
        PieceType::Queen
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