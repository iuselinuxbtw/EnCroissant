/// All available pieces.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    /// Returns the short code of the piece type according to the algebraic standard.
    pub fn get_shortcode_algebraic(&self) -> &'static str {
        match self {
            PieceType::Pawn => "",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Queen => "Q",
            PieceType::King => "K",
        }
    }
}

/// The color of a piece.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceColor {
    Light,
    Dark,
}

impl PieceColor {
    /// Returns the opposite team. Useful for checking for legal moves.
    pub fn get_opponent(&self) -> PieceColor {
        match self {
            PieceColor::Light => { PieceColor::Dark }
            PieceColor::Dark => { PieceColor::Light }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod piece_type {
        use super::*;

        #[test]
        fn test_get_shortcode_algebraic() {
            assert_eq!("", PieceType::Pawn.get_shortcode_algebraic());
            assert_eq!("N", PieceType::Knight.get_shortcode_algebraic());
            assert_eq!("B", PieceType::Bishop.get_shortcode_algebraic());
            assert_eq!("R", PieceType::Rook.get_shortcode_algebraic());
            assert_eq!("Q", PieceType::Queen.get_shortcode_algebraic());
            assert_eq!("K", PieceType::King.get_shortcode_algebraic());
        }
    }
}
