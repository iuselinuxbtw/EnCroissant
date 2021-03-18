use super::Piece;

pub struct Pawn {}

impl Piece for Pawn {
    fn get_shortcode_algebraic(&self) -> &'static str {
        ""
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
}