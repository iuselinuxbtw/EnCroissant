//! Definition and implementation of the different chess pieces.

use std::fmt::Debug;
use std::ops::Deref;

use dyn_clonable::clonable;

use crate::coordinate::Coordinate;
use crate::formats::fen::FenPiece;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod move_gen;
pub mod pawn;
pub mod queen;
pub mod rook;

/// A [`Piece`] represents a chess figure on the [`Board`](struct@crate::board::Board).
#[clonable]
pub trait Piece: Debug + Clone {
    /// Returns the short code of [`Piece`]'s type according to the algebraic standard.
    fn get_shortcode_algebraic(&self) -> &'static str {
        self.get_type().get_shortcode_algebraic()
    }

    /// Returns the [`PieceType`] of the piece.
    fn get_type(&self) -> PieceType;
}

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
    fn get_shortcode_algebraic(&self) -> &'static str {
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

impl From<PieceType> for Box<dyn Piece> {
    /// Converts the [`PieceType`] into a [`Box<dyn Piece>`] that holds a piece of the supplied
    /// type.
    fn from(t: PieceType) -> Box<dyn Piece> {
        match t {
            PieceType::Pawn => Box::new(pawn::Pawn {}),
            PieceType::Knight => Box::new(knight::Knight {}),
            PieceType::Bishop => Box::new(bishop::Bishop {}),
            PieceType::Rook => Box::new(rook::Rook {}),
            PieceType::Queen => Box::new(queen::Queen {}),
            PieceType::King => Box::new(king::King {}),
        }
    }
}

/// The color of a piece.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceColor {
    Light,
    Dark,
}

/// A [`Piece`] that has additional properties so it can sit on a [`Board`](struct@crate::board::Board).
#[derive(Debug, Clone)]
pub struct BoardPiece {
    piece: Box<dyn Piece>,
    color: PieceColor,
    coordinate: Coordinate,
    out_of_game: bool,
    has_moved: bool,
}

impl BoardPiece {
    pub fn new(piece: Box<dyn Piece>, coordinate: Coordinate, color: PieceColor) -> BoardPiece {
        BoardPiece {
            piece,
            color,
            coordinate,
            out_of_game: false,
            has_moved: false,
        }
    }

    /// Creates a new piece from the supplied [`PieceType`]. Under the hood, this method calls
    /// [`BoardPiece::new`] with a [`Box<dyn Piece>`] generated according to the supplied piece type.
    pub fn new_from_type(
        piece_type: PieceType,
        coordinate: Coordinate,
        color: PieceColor,
    ) -> BoardPiece {
        BoardPiece::new(piece_type.into(), coordinate, color)
    }

    pub fn get_color(&self) -> PieceColor {
        self.color
    }

    pub fn get_coordinate(&self) -> Coordinate {
        self.coordinate
    }

    pub fn get_piece(&self) -> &dyn Piece {
        self.piece.deref()
    }

    pub fn get_has_moved(&self) -> bool {
        self.has_moved
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

impl From<FenPiece> for BoardPiece {
    fn from(fp: FenPiece) -> Self {
        BoardPiece::new_from_type(fp.2, fp.0, fp.1)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{Formatter, Result as FmtResult};

    use mockall::mock;

    use super::*;

    mock! {
        pub Piece {}
        impl Piece for MockPiece {
            fn get_shortcode_algebraic(&self) -> &'static str;
            fn get_type(&self) -> PieceType;
        }

        impl Clone for MockPiece {
            fn clone(&self) -> Self;
        }
    }

    impl Debug for MockPiece {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("MockPiece").finish()
        }
    }

    mod board_piece {
        use super::*;

        fn get_board_piece(p: MockPiece, color: PieceColor) -> BoardPiece {
            BoardPiece::new(Box::new(p), Coordinate::new(1, 2), color)
        }

        #[test]
        fn test_get_color() {
            let mock = MockPiece::new();
            let p = get_board_piece(mock, PieceColor::Light);
            assert_eq!(p.get_color(), PieceColor::Light);

            let mock = MockPiece::new();
            let p = get_board_piece(mock, PieceColor::Dark);
            assert_eq!(p.get_color(), PieceColor::Dark);
        }

        #[test]
        fn test_get_coordinate() {
            let mock = MockPiece::new();
            let p = get_board_piece(mock, PieceColor::Light);
            assert_eq!(p.get_coordinate(), Coordinate::new(1, 2));
        }

        #[test]
        fn test_eq_and_new() {
            // Everything is equal
            let mut mock1 = MockPiece::new();
            mock1.expect_get_shortcode_algebraic().return_const("Q");
            let p1 = BoardPiece::new(Box::new(mock1), (3, 4).into(), PieceColor::Dark);

            let mut mock2 = MockPiece::new();
            mock2.expect_get_shortcode_algebraic().return_const("Q");
            let mut p2 = BoardPiece::new(Box::new(mock2), (3, 4).into(), PieceColor::Dark);
            assert_eq!(p1, p2);

            // Piece does not has the same short code
            let mut mock3 = MockPiece::new();
            mock3.expect_get_shortcode_algebraic().return_const("K");
            let p3 = BoardPiece::new(Box::new(mock3), (3, 4).into(), PieceColor::Dark);
            assert_ne!(p1, p3);

            // Color does not match
            p2.color = PieceColor::Light;
            assert_ne!(p1, p2);
            p2.color = PieceColor::Dark;

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

        #[test]
        fn test_new_from_type() {
            // Pawn
            assert_eq!(
                BoardPiece::new(Box::new(pawn::Pawn {}), (7, 1).into(), PieceColor::Dark,),
                BoardPiece::new_from_type(PieceType::Pawn, (7, 1).into(), PieceColor::Dark),
            );

            // Knight
            assert_eq!(
                BoardPiece::new(
                    Box::new(knight::Knight {}),
                    (1, 3).into(),
                    PieceColor::Light,
                ),
                BoardPiece::new_from_type(PieceType::Knight, (1, 3).into(), PieceColor::Light),
            );

            // Bishop
            assert_eq!(
                BoardPiece::new(Box::new(bishop::Bishop {}), (4, 4).into(), PieceColor::Dark,),
                BoardPiece::new_from_type(PieceType::Bishop, (4, 4).into(), PieceColor::Dark),
            );

            // Rook
            assert_eq!(
                BoardPiece::new(Box::new(rook::Rook {}), (3, 5).into(), PieceColor::Light,),
                BoardPiece::new_from_type(PieceType::Rook, (3, 5).into(), PieceColor::Light),
            );

            // Queen
            assert_eq!(
                BoardPiece::new(Box::new(queen::Queen {}), (5, 5).into(), PieceColor::Dark,),
                BoardPiece::new_from_type(PieceType::Queen, (5, 5).into(), PieceColor::Dark),
            );

            // King
            assert_eq!(
                BoardPiece::new(Box::new(king::King {}), (2, 7).into(), PieceColor::Light,),
                BoardPiece::new_from_type(PieceType::King, (2, 7).into(), PieceColor::Light),
            );
        }

        #[test]
        fn test_from_fen_piece() {
            assert_eq!(
                BoardPiece::new_from_type(PieceType::Queen, (3, 2).into(), PieceColor::Light),
                ((3, 2).into(), PieceColor::Light, PieceType::Queen).into(),
            );
            assert_eq!(
                BoardPiece::new_from_type(PieceType::Pawn, (6, 0).into(), PieceColor::Dark),
                ((6, 0).into(), PieceColor::Dark, PieceType::Pawn).into(),
            );
        }

        #[test]
        fn test_get_has_moved() {
            let mut p =
                BoardPiece::new_from_type(PieceType::Pawn, (1, 2).into(), PieceColor::Light);
            assert!(!p.get_has_moved());
            p.has_moved = true;
            assert!(p.get_has_moved());
        }
    }

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

        #[test]
        fn test_into_box_dyn_piece() {
            // Actually this is `impl From<PieceType> for Box<dyn Piece>`
            assert_eq!(
                Box::new(pawn::Pawn {}).get_type(),
                Box::<dyn Piece>::from(PieceType::Pawn).get_type()
            );
            assert_eq!(
                Box::new(knight::Knight {}).get_type(),
                Box::<dyn Piece>::from(PieceType::Knight).get_type()
            );
            assert_eq!(
                Box::new(bishop::Bishop {}).get_type(),
                Box::<dyn Piece>::from(PieceType::Bishop).get_type()
            );
            assert_eq!(
                Box::new(rook::Rook {}).get_type(),
                Box::<dyn Piece>::from(PieceType::Rook).get_type()
            );
            assert_eq!(
                Box::new(queen::Queen {}).get_type(),
                Box::<dyn Piece>::from(PieceType::Queen).get_type()
            );
            assert_eq!(
                Box::new(king::King {}).get_type(),
                Box::<dyn Piece>::from(PieceType::King).get_type()
            );
        }
    }
}
