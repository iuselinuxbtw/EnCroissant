use std::fmt::{self, Display};

/// A [`Coordinate`] represents a square on the chess board.
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Coordinate {
    y: u8,
    x: u8,
}

impl Coordinate {
    /// Returns a new instance of [`Coordinate`] with the supplied x and y coordinates set.
    pub fn new(x: u8, y: u8) -> Coordinate {
        Coordinate {
            x,
            y,
        }
    }

    /// Returns the x coordinate.
    pub fn get_x(&self) -> u8 {
        self.x
    }

    /// Returns the y coordinate.
    pub fn get_y(&self) -> u8 {
        self.y
    }

    /// Returns the x coordinate as a char.
    pub fn get_x_as_char(&self) -> char {
        match self.x {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => ' ',
        }
    }
}

/// Converts the x coordinate represented as a [`char`] to a [`u8`] that represents the x coordinate
/// in the board. When the char cannot be mapped into an appropriate coordinate (e.g. the char is
/// `'z'`) it returns an invalid x coordinate (`8`).
pub fn char_to_x_coordinate(c: char) -> u8 {
    match c {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => 8, // This should not happen
    }
}

impl From<(u8, u8)> for Coordinate {
    fn from(v: (u8, u8)) -> Self {
        Coordinate::new(v.0, v.1)
    }
}

impl Display for Coordinate {
    /// Formats the [`Coordinate`] using the format `xy`, where `x` is the lower-case char and `y`
    /// is the number.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.get_x_as_char(), self.get_y() + 1)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    fn get_coordinate() -> Coordinate {
        Coordinate {
            x: 3,
            y: 4,
        }
    }

    #[test]
    fn test_coordinate_new() {
        assert_eq!(get_coordinate(), Coordinate::new(3, 4));
    }

    #[test]
    fn test_coordinate_get_x() {
        assert_eq!(3, get_coordinate().get_x());
    }

    #[test]
    fn test_coordinate_get_x_as_char() {
        assert_eq!('a', Coordinate::new(0, 0).get_x_as_char());
        assert_eq!('b', Coordinate::new(1, 0).get_x_as_char());
        assert_eq!('c', Coordinate::new(2, 0).get_x_as_char());
        assert_eq!('d', Coordinate::new(3, 0).get_x_as_char());
        assert_eq!('e', Coordinate::new(4, 0).get_x_as_char());
        assert_eq!('f', Coordinate::new(5, 0).get_x_as_char());
        assert_eq!('g', Coordinate::new(6, 0).get_x_as_char());
        assert_eq!('h', Coordinate::new(7, 0).get_x_as_char());
        assert_eq!(' ', Coordinate::new(8, 0).get_x_as_char());
        assert_eq!(' ', Coordinate::new(42, 0).get_x_as_char());
    }

    #[test]
    fn test_get_y() {
        assert_eq!(4, get_coordinate().get_y());
    }

    #[test]
    fn test_coordinate_from_tuple() {
        assert_eq!(Coordinate::from((3, 4)), Coordinate::new(3, 4));
    }

    #[test]
    fn test_char_to_x_coordinate() {
        assert_eq!(0, char_to_x_coordinate('a'));
        assert_eq!(1, char_to_x_coordinate('b'));
        assert_eq!(2, char_to_x_coordinate('c'));
        assert_eq!(3, char_to_x_coordinate('d'));
        assert_eq!(4, char_to_x_coordinate('e'));
        assert_eq!(5, char_to_x_coordinate('f'));
        assert_eq!(6, char_to_x_coordinate('g'));
        assert_eq!(7, char_to_x_coordinate('h'));
        assert_eq!(8, char_to_x_coordinate('i'));
        assert_eq!(8, char_to_x_coordinate('u'));
    }

    #[test]
    fn test_to_string() {
        assert_eq!("a1", Coordinate::new(0, 0).to_string());
        assert_eq!("e6", Coordinate::new(4, 5).to_string());
        assert_eq!("g7", Coordinate::new(6, 6).to_string());
        assert_eq!("h8", Coordinate::new(7, 7).to_string());
    }

    #[test]
    fn test_partial_cmp() {
        assert_eq!(Some(Ordering::Greater), Coordinate::new(7, 7).partial_cmp(&Coordinate::new(0, 0)));
        assert_eq!(Some(Ordering::Equal), Coordinate::new(4, 4).partial_cmp(&Coordinate::new(4, 4)));
        assert_eq!(Some(Ordering::Less), Coordinate::new(4, 3).partial_cmp(&Coordinate::new(4, 4)));
        assert_eq!(Some(Ordering::Less), Coordinate::new(3, 4).partial_cmp(&Coordinate::new(4, 4)));
        assert_eq!(Some(Ordering::Greater), Coordinate::new(5, 4).partial_cmp(&Coordinate::new(4, 4)));
        assert_eq!(Some(Ordering::Less), Coordinate::new(7, 1).partial_cmp(&Coordinate::new(1, 7)));
    }
}