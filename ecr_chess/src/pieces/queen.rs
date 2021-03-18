use super::Piece;

pub struct Queen {}

impl Piece for Queen {
    fn get_shortcode_algebraic(&self) -> &'static str {
        "Q"
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
}