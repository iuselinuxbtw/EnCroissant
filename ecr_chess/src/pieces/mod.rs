pub mod king;
pub mod queen;
pub mod rook;
pub mod bishop;
pub mod knight;
pub mod pawn;

/// A `Piece` represents a chess figure on the `Board`.
pub trait Piece {
    /// Returns the short code of `Piece`'s type according to the algebraic standard.
    fn get_shortcode_algebraic(&self) -> &'static str;
}

/// The color of a piece.
pub enum PieceColor {
    LIGHT,
    DARK,
}