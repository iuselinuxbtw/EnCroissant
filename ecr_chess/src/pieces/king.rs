use super::Piece;

pub struct King {}

impl Piece for King {
    fn get_shortcode_algebraic(&self) -> &'static str {
        "K"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_piece() -> King {
        King {}
    }

    #[test]
    fn test_get_shortcode_algebraic() {
        assert_eq!("K", get_piece().get_shortcode_algebraic());
    }
}