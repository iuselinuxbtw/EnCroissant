use super::Piece;

#[derive(Debug, PartialEq, Clone)]
pub struct Knight {}

impl Piece for Knight {
    fn get_shortcode_algebraic(&self) -> &'static str {
        "N"
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
}