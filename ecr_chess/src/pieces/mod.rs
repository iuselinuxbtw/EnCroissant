use crate::coordinate::Coordinate;
use std::fmt::Debug;
use dyn_clonable::clonable;

pub mod king;
pub mod queen;
pub mod rook;
pub mod bishop;
pub mod knight;
pub mod pawn;

/// A `Piece` represents a chess figure on the `Board`.
#[clonable]
pub trait Piece: Debug + Clone {
    /// Returns the short code of `Piece`'s type according to the algebraic standard.
    fn get_shortcode_algebraic(&self) -> &'static str;
}

/// The color of a piece.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceColor {
    LIGHT,
    DARK,
}

/// A `Piece` that has additional properties so it can sit on a `Board`.
#[derive(Debug, Clone)]
pub struct BoardPiece {
    piece: Box<dyn Piece>,
    color: PieceColor,
    coordinate: Coordinate,
    out_of_game: bool,
}

impl BoardPiece {
    pub fn new(piece: Box<dyn Piece>, coordinate: Coordinate, color: PieceColor) -> BoardPiece {
        BoardPiece {
            piece,
            color,
            coordinate,
            out_of_game: false,
        }
    }

    pub fn get_color(&self) -> PieceColor {
        self.color
    }

    pub fn get_coordinate(&self) -> Coordinate {
        self.coordinate
    }

    pub fn get_piece(&self) -> &Box<dyn Piece> {
        &self.piece
    }
}

impl PartialEq for BoardPiece {
    /// According to this implementation, a piece on a board is equal if their corresponding short
    /// code in algebraic form is equal, their out of game state, their color and their coordinate
    /// are equal.
    fn eq(&self, other: &Self) -> bool {
        self.out_of_game == other.out_of_game
            && self.piece.get_shortcode_algebraic() == other.piece.get_shortcode_algebraic()
            && self.color == other.color
            && self.coordinate == other.coordinate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use std::fmt::{Formatter, Result as FmtResult};

    mock!{
        pub Piece {}
        impl Piece for MockPiece {
            fn get_shortcode_algebraic(&self) -> &'static str;
        }

        impl Clone for MockPiece {
            fn clone(&self) -> Self;
        }
    }

    impl Debug for MockPiece {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("MockPiece")
                .finish()
        }
    }

    fn get_board_piece(p: MockPiece, color: PieceColor) -> BoardPiece {
        BoardPiece::new(Box::new(p), Coordinate::new(1, 2), color)
    }

    #[test]
    fn test_get_color() {
        let mock = MockPiece::new();
        let p = get_board_piece(mock, PieceColor::LIGHT);
        assert_eq!(p.get_color(), PieceColor::LIGHT);

        let mock = MockPiece::new();
        let p = get_board_piece(mock, PieceColor::DARK);
        assert_eq!(p.get_color(), PieceColor::DARK);
    }

    #[test]
    fn test_get_coordinate() {
        let mock = MockPiece::new();
        let p = get_board_piece(mock, PieceColor::LIGHT);
        assert_eq!(p.get_coordinate(), Coordinate::new(1, 2));
    }

    #[test]
    fn test_eq_and_new() {
        // Everything is equal
        let mut mock1 = MockPiece::new();
        mock1.expect_get_shortcode_algebraic()
            .return_const("Q");
        let p1 = BoardPiece::new(Box::new(mock1), (3, 4).into(), PieceColor::DARK);

        let mut mock2 = MockPiece::new();
        mock2.expect_get_shortcode_algebraic()
            .return_const("Q");
        let mut p2 = BoardPiece::new(Box::new(mock2), (3, 4).into(), PieceColor::DARK);
        assert_eq!(p1, p2);

        // Piece does not has the same short code
        let mut mock3 = MockPiece::new();
        mock3.expect_get_shortcode_algebraic()
            .return_const("K");
        let p3 = BoardPiece::new(Box::new(mock3), (3, 4).into(), PieceColor::DARK);
        assert_ne!(p1, p3);

        // Color does not match
        p2.color = PieceColor::LIGHT;
        assert_ne!(p1, p2);
        p2.color = PieceColor::DARK;

        // Coordinate does not match
        p2.coordinate = (1, 2).into();
        assert_ne!(p1, p2);
        p2.coordinate = (3, 4).into();

        // Out of game does not match
        p2.out_of_game = true;
        assert_ne!(p1, p2);
        p2.out_of_game = false;

        // Everything is reset, p1 and p2 should be equal again
        assert_eq!(p1, p2);
    }
}