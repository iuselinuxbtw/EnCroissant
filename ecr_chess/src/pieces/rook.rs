use super::Piece;

pub struct Rook {}

impl Piece for Rook {
    fn get_shortcode_algebraic(&self) -> &'static str {
        "R"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_piece() -> Rook {
        Rook {}
    }

    #[test]
    fn test_get_shortcode_algebraic() {
        assert_eq!("R", get_piece().get_shortcode_algebraic());
    }
}